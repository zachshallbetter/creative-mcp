//! Local, read-only detection of creative apps and MCP clients. Nothing leaves the machine.

use crate::registry::{Client, Registry, Target};
use serde::Serialize;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::{mpsc, OnceLock};
use std::time::Duration;

#[derive(Debug, Clone, Serialize)]
pub struct FoundApp {
    pub tool: String,
    pub app: String,
    pub name: String,
    pub path: String,
    pub version: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct FoundClient {
    pub id: String,
    pub name: String,
    /// The client's own CLI, when it is on PATH.
    pub command: Option<String>,
    /// Config files that exist.
    pub files: Vec<String>,
}

pub fn os_key() -> &'static str {
    if cfg!(target_os = "macos") {
        "macos"
    } else if cfg!(target_os = "windows") {
        "windows"
    } else {
        "linux"
    }
}

pub fn home() -> PathBuf {
    std::env::var_os("HOME")
        .or_else(|| std::env::var_os("USERPROFILE"))
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."))
}

pub fn expand(path: &str) -> PathBuf {
    match path.strip_prefix("~/") {
        Some(rest) => home().join(rest),
        None => PathBuf::from(path),
    }
}

/// Find an executable on PATH.
pub fn which(command: &str) -> Option<PathBuf> {
    let path = std::env::var_os("PATH")?;
    let exts: Vec<String> = if cfg!(windows) {
        std::env::var("PATHEXT")
            .unwrap_or_else(|_| ".EXE;.CMD;.BAT".into())
            .split(';')
            .map(|s| s.to_lowercase())
            .collect()
    } else {
        vec![String::new()]
    };
    for dir in std::env::split_paths(&path) {
        for ext in &exts {
            let candidate = dir.join(format!("{command}{ext}"));
            if is_executable(&candidate) {
                return Some(candidate);
            }
        }
    }
    None
}

fn is_executable(path: &Path) -> bool {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        path.metadata()
            .map(|m| m.is_file() && m.permissions().mode() & 0o111 != 0)
            .unwrap_or(false)
    }
    #[cfg(not(unix))]
    {
        path.is_file()
    }
}

/// Run a command and return its stdout, or `None` if it fails or takes longer than `timeout`.
/// Spotlight can stall for minutes while it re-indexes; detection must not hang with it.
pub fn output_within(program: &str, args: &[&str], timeout: Duration) -> Option<String> {
    let mut child = Command::new(program)
        .args(args)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .ok()?;
    let mut stdout = child.stdout.take()?;
    let (tx, rx) = mpsc::channel();
    std::thread::spawn(move || {
        let mut text = String::new();
        let _ = stdout.read_to_string(&mut text);
        let _ = tx.send(text);
    });
    match rx.recv_timeout(timeout) {
        Ok(text) => child.wait().ok().filter(|s| s.success()).map(|_| text),
        Err(_) => {
            let _ = child.kill();
            let _ = child.wait();
            None
        }
    }
}

const SPOTLIGHT_TIMEOUT: Duration = Duration::from_secs(8);

/// App bundles by Spotlight query. `None` when Spotlight doesn't answer in time.
fn mdfind(query: &str) -> Option<Vec<String>> {
    output_within("mdfind", &[query], SPOTLIGHT_TIMEOUT).map(|out| {
        out.lines()
            .filter(|l| l.ends_with(".app"))
            .map(str::to_string)
            .collect()
    })
}

/// Bundle ID and path of every app in the usual folders, read from each Info.plist.
/// The fallback when Spotlight is off or stalled; computed once per run.
fn scanned_bundles() -> &'static [(String, String)] {
    static SCAN: OnceLock<Vec<(String, String)>> = OnceLock::new();
    SCAN.get_or_init(|| {
        let mut found = Vec::new();
        for pattern in [
            "/Applications/*.app",
            "/Applications/*/*.app",
            "~/Applications/*.app",
            "~/Applications/*/*.app",
        ] {
            for app in glob(pattern) {
                let plist = app.join("Contents/Info.plist");
                let id = output_within(
                    "plutil",
                    &[
                        "-extract",
                        "CFBundleIdentifier",
                        "raw",
                        "-o",
                        "-",
                        &plist.display().to_string(),
                    ],
                    Duration::from_secs(2),
                );
                if let Some(id) = id {
                    found.push((id.trim().to_string(), app.display().to_string()));
                }
            }
        }
        found
    })
}

/// Apps whose bundle ID equals `id`, or starts with it when `prefix` is set.
fn find_bundles(id: &str, prefix: bool) -> Vec<String> {
    let query = if prefix {
        format!("kMDItemCFBundleIdentifier == '{id}*'")
    } else {
        format!("kMDItemCFBundleIdentifier == '{id}'")
    };
    match mdfind(&query) {
        Some(paths) if !paths.is_empty() => paths,
        _ => scanned_bundles()
            .iter()
            .filter(|(bundle, _)| {
                if prefix {
                    bundle.starts_with(id)
                } else {
                    bundle == id
                }
            })
            .map(|(_, path)| path.clone())
            .collect(),
    }
}

fn app_version(path: &str) -> Option<String> {
    let from_spotlight = output_within(
        "mdls",
        &["-raw", "-name", "kMDItemVersion", path],
        SPOTLIGHT_TIMEOUT,
    )
    .map(|v| v.trim().to_string())
    .filter(|v| !v.is_empty() && v != "(null)");
    from_spotlight.or_else(|| {
        let plist = format!("{path}/Contents/Info.plist");
        output_within(
            "plutil",
            &[
                "-extract",
                "CFBundleShortVersionString",
                "raw",
                "-o",
                "-",
                &plist,
            ],
            Duration::from_secs(2),
        )
        .map(|v| v.trim().to_string())
        .filter(|v| !v.is_empty())
    })
}

fn detect_target(tool: &str, target: &Target) -> Option<FoundApp> {
    let rule = target.detect.get(os_key())?;
    let mut paths = Vec::new();
    if os_key() == "macos" {
        for id in &rule.bundle_ids {
            paths.extend(find_bundles(id, false));
        }
        for prefix in &rule.bundle_id_prefixes {
            paths.extend(find_bundles(prefix, true));
        }
    }
    for command in &rule.commands {
        if let Some(p) = which(command) {
            paths.push(p.display().to_string());
        }
    }
    for pattern in &rule.globs {
        if let Some(p) = glob(pattern).pop() {
            paths.push(p.display().to_string());
        }
    }
    // Prefer apps installed in /Applications over copies inside other bundles or build folders.
    paths.sort_by_key(|p| (!p.starts_with("/Applications"), p.len()));
    let path = paths.into_iter().next()?;
    Some(FoundApp {
        tool: tool.to_string(),
        app: target.app.clone(),
        name: target.name.clone(),
        version: if os_key() == "macos" {
            app_version(&path)
        } else {
            None
        },
        path,
    })
}

pub fn detect_apps(registry: &Registry) -> Vec<FoundApp> {
    registry
        .tools
        .iter()
        .flat_map(|tool| {
            tool.targets
                .iter()
                .filter_map(move |t| detect_target(&tool.id, t))
        })
        .collect()
}

/// Expand `*` path segments (one directory level each) into the matching files, sorted so the newest
/// version comes last (`2.1.10` after `2.1.9`).
pub fn glob(pattern: &str) -> Vec<PathBuf> {
    let mut found = vec![PathBuf::new()];
    for part in expand(pattern).components() {
        let part = part.as_os_str().to_string_lossy();
        let mut next = Vec::new();
        for base in found {
            if part.contains('*') {
                let (pre, post) = part.split_once('*').unwrap_or((&part, ""));
                if let Ok(entries) = std::fs::read_dir(if base.as_os_str().is_empty() {
                    Path::new(".")
                } else {
                    &base
                }) {
                    for e in entries.flatten() {
                        let name = e.file_name().to_string_lossy().to_string();
                        if name.starts_with(pre) && name.ends_with(post) && !name.starts_with('.') {
                            next.push(base.join(name));
                        }
                    }
                }
            } else {
                next.push(base.join(part.as_ref()));
            }
        }
        found = next;
    }
    let key = |p: &PathBuf| -> Vec<(u64, String)> {
        p.to_string_lossy()
            .split(|c: char| !c.is_ascii_alphanumeric())
            .map(|s| (s.parse().unwrap_or(0), s.to_string()))
            .collect()
    };
    found.retain(|p| p.exists());
    found.sort_by_key(key);
    found
}

pub fn detect_client(client: &Client) -> Option<FoundClient> {
    let command = client
        .detect
        .commands
        .iter()
        .find_map(|c| which(c))
        .or_else(|| {
            client
                .detect
                .paths
                .iter()
                .find_map(|p| glob(p).into_iter().rev().find(|c| is_executable(c)))
        })
        .map(|p| p.display().to_string());
    let files: Vec<String> = client
        .detect
        .files
        .iter()
        .map(|f| expand(f))
        .filter(|p| p.exists())
        .map(|p| p.display().to_string())
        .collect();
    (command.is_some() || !files.is_empty()).then(|| FoundClient {
        id: client.id.clone(),
        name: client.name.clone(),
        command,
        files,
    })
}

pub fn detect_clients(registry: &Registry) -> Vec<FoundClient> {
    registry.clients.iter().filter_map(detect_client).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expands_home() {
        assert_eq!(expand("~/x/y"), home().join("x/y"));
        assert_eq!(expand("/abs"), PathBuf::from("/abs"));
    }

    #[test]
    fn glob_finds_the_newest_version() {
        let dir = std::env::temp_dir().join(format!("creative-mcp-glob-{}", std::process::id()));
        for v in ["2.1.9", "2.1.10", "2.0.0"] {
            std::fs::create_dir_all(dir.join(v).join("abc")).unwrap();
            std::fs::write(dir.join(v).join("abc/claude"), "").unwrap();
        }
        let found = glob(&format!("{}/*/*/claude", dir.display()));
        assert_eq!(found.len(), 3);
        assert!(found.last().unwrap().to_string_lossy().contains("2.1.10"));
        assert!(glob(&format!("{}/*/none", dir.display())).is_empty());
        // A wildcard inside a name, as in "Adobe Photoshop *" or "Blender *".
        std::fs::create_dir_all(dir.join("Blender 4.2")).unwrap();
        std::fs::create_dir_all(dir.join("Blender 5.2")).unwrap();
        std::fs::write(dir.join("Blender 5.2/blender.exe"), "").unwrap();
        let found = glob(&format!("{}/Blender */blender.exe", dir.display()));
        assert_eq!(found.len(), 1);
        assert!(found[0].to_string_lossy().contains("Blender 5.2"));
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn bundle_scan_works_without_spotlight() {
        // Only meaningful on a Mac with Blender installed; elsewhere there is nothing to find.
        if Path::new("/Applications/Blender.app").exists() {
            assert!(scanned_bundles()
                .iter()
                .any(|(id, path)| id == "org.blenderfoundation.blender"
                    && path.ends_with("Blender.app")));
        }
    }

    #[test]
    fn slow_commands_time_out() {
        if cfg!(unix) {
            let start = std::time::Instant::now();
            assert!(output_within("sleep", &["5"], Duration::from_millis(200)).is_none());
            assert!(start.elapsed() < Duration::from_secs(2));
            assert_eq!(
                output_within("echo", &["hi"], Duration::from_secs(5)).as_deref(),
                Some("hi\n")
            );
        }
    }

    #[test]
    fn which_finds_a_shell_utility() {
        if cfg!(unix) {
            assert!(which("sh").is_some());
        }
        assert!(which("definitely-not-a-real-command-xyz").is_none());
    }
}
