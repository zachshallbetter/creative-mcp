//! Local, read-only detection of creative apps and MCP clients. Nothing leaves the machine.

use crate::registry::{Client, Registry, Target};
use serde::Serialize;
use std::path::{Path, PathBuf};
use std::process::Command;

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

fn mdfind(query: &str) -> Vec<String> {
    Command::new("mdfind")
        .arg(query)
        .output()
        .ok()
        .filter(|o| o.status.success())
        .map(|o| {
            String::from_utf8_lossy(&o.stdout)
                .lines()
                .filter(|l| l.ends_with(".app"))
                .map(str::to_string)
                .collect()
        })
        .unwrap_or_default()
}

fn app_version(path: &str) -> Option<String> {
    let out = Command::new("mdls")
        .args(["-raw", "-name", "kMDItemVersion", path])
        .output()
        .ok()?;
    let v = String::from_utf8_lossy(&out.stdout).trim().to_string();
    (!v.is_empty() && v != "(null)").then_some(v)
}

fn detect_target(tool: &str, target: &Target) -> Option<FoundApp> {
    let rule = target.detect.get(os_key())?;
    let mut paths = Vec::new();
    if os_key() == "macos" {
        for id in &rule.bundle_ids {
            paths.extend(mdfind(&format!("kMDItemCFBundleIdentifier == '{id}'")));
        }
        for prefix in &rule.bundle_id_prefixes {
            paths.extend(mdfind(&format!("kMDItemCFBundleIdentifier == '{prefix}*'")));
        }
    }
    for command in &rule.commands {
        if let Some(p) = which(command) {
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

pub fn detect_client(client: &Client) -> Option<FoundClient> {
    let command = client
        .detect
        .commands
        .iter()
        .find_map(|c| which(c).map(|p| p.display().to_string()));
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
    fn which_finds_a_shell_utility() {
        if cfg!(unix) {
            assert!(which("sh").is_some());
        }
        assert!(which("definitely-not-a-real-command-xyz").is_none());
    }
}
