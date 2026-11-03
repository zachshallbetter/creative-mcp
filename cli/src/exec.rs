//! Carries out a plan and checks that servers answer MCP `initialize`.

use crate::detect::which;
use crate::plan::{Environment, Step, TAP};
use serde_json::{json, Map, Value};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use std::process::{Command, Stdio};
use std::sync::{mpsc, Mutex};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub fn environment() -> Environment {
    let brew = which("brew").is_some();
    let output = |args: &[&str]| -> Option<String> {
        let o = Command::new("brew").args(args).output().ok()?;
        o.status
            .success()
            .then(|| String::from_utf8_lossy(&o.stdout).to_string())
    };
    Environment {
        brew,
        brew_has_trust: brew && output(&["trust", "--help"]).is_some(),
        tap_installed: brew && output(&["tap"]).is_some_and(|t| t.lines().any(|l| l.trim() == TAP)),
        npm: which("npm").is_some(),
        cargo: which("cargo").is_some(),
        installed_formulae: if brew {
            output(&["list", "--formula", "-1"])
                .map(|t| t.lines().map(str::to_string).collect())
                .unwrap_or_default()
        } else {
            Vec::new()
        },
    }
}

/// Use the absolute path once a command is installed, so GUI clients without a shell PATH can start it.
fn resolve(command: &str) -> String {
    which(command)
        .map(|p| p.display().to_string())
        .unwrap_or_else(|| command.to_string())
}

/// Back up a file once per run, before its first change, so the copy is always the original.
fn backup(path: &Path) -> std::io::Result<Option<String>> {
    static DONE: Mutex<Vec<(String, String)>> = Mutex::new(Vec::new());
    let key = path.display().to_string();
    let mut done = DONE.lock().unwrap_or_else(|e| e.into_inner());
    if let Some((_, existing)) = done.iter().find(|(p, _)| *p == key) {
        return Ok(Some(existing.clone()));
    }
    if !path.exists() {
        return Ok(None);
    }
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis())
        .unwrap_or(0);
    let target = format!("{key}.creative-mcp-{stamp}.bak");
    std::fs::copy(path, &target)?;
    done.push((key, target.clone()));
    Ok(Some(target))
}

fn write_json(path: &str, name: &str, command: &str, args: &[String]) -> Result<String, String> {
    let path = Path::new(path);
    let mut root: Value = if path.exists() {
        let text = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
        if text.trim().is_empty() {
            json!({})
        } else {
            serde_json::from_str(&text).map_err(|e| {
                format!("{} is not valid JSON ({e}); left unchanged", path.display())
            })?
        }
    } else {
        json!({})
    };
    let obj = root
        .as_object_mut()
        .ok_or_else(|| format!("{} is not a JSON object; left unchanged", path.display()))?;
    let servers = obj
        .entry("mcpServers")
        .or_insert_with(|| Value::Object(Map::new()));
    let servers = servers
        .as_object_mut()
        .ok_or("mcpServers is not an object; left unchanged")?;
    servers.insert(
        name.to_string(),
        json!({ "command": resolve(command), "args": args }),
    );
    let saved = backup(path).map_err(|e| format!("could not back up {}: {e}", path.display()))?;
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    std::fs::write(
        path,
        serde_json::to_string_pretty(&root).map_err(|e| e.to_string())? + "\n",
    )
    .map_err(|e| e.to_string())?;
    Ok(saved
        .map(|b| format!("backup: {b}"))
        .unwrap_or_else(|| "new file".into()))
}

pub struct Outcome {
    pub ok: usize,
    pub failed: Vec<String>,
    pub manual: Vec<String>,
}

pub fn describe(step: &Step) -> String {
    match step {
        Step::Run {
            why,
            program,
            args,
            server_arg,
            ..
        } => {
            let mut args = args.clone();
            if let Some(i) = *server_arg {
                args[i] = resolve(&args[i]);
            }
            format!("{why}\n      $ {program} {}", args.join(" "))
        }
        Step::WriteJson {
            why,
            path,
            name,
            command,
            args,
        } => {
            format!("{why}\n      edit {path}: mcpServers.{name} = {{ command: {}, args: {args:?} }} (backed up first)", resolve(command))
        }
        Step::Manual { why, text } => format!("{why}\n      you: {text}"),
    }
}

pub fn execute(steps: &[Step]) -> Outcome {
    let mut out = Outcome {
        ok: 0,
        failed: Vec::new(),
        manual: Vec::new(),
    };
    for (i, step) in steps.iter().enumerate() {
        println!("\n[{}/{}] {}", i + 1, steps.len(), describe(step));
        match step {
            Step::Run {
                why,
                program,
                args,
                allow_failure,
                server_arg,
            } => {
                let program = resolve(program);
                let mut args = args.clone();
                if let Some(i) = *server_arg {
                    args[i] = resolve(&args[i]);
                }
                match Command::new(&program).args(&args).status() {
                    Ok(s) if s.success() => out.ok += 1,
                    Ok(s) if *allow_failure => {
                        println!("      (exit {}; continuing, this step is safe to skip if it was already done)", s.code().unwrap_or(-1));
                        out.ok += 1;
                    }
                    Ok(s) => out
                        .failed
                        .push(format!("{why} (exit {})", s.code().unwrap_or(-1))),
                    Err(e) => out.failed.push(format!("{why} ({program}: {e})")),
                }
            }
            Step::WriteJson {
                why,
                path,
                name,
                command,
                args,
            } => match write_json(path, name, command, args) {
                Ok(note) => {
                    println!("      done ({note})");
                    out.ok += 1;
                }
                Err(e) => out.failed.push(format!("{why}: {e}")),
            },
            Step::Manual { why, text } => out.manual.push(format!("{why}: {text}")),
        }
    }
    out
}

/// Start a stdio MCP server, send `initialize`, and return its `serverInfo`.
pub fn verify(command: &str, args: &[String], timeout: Duration) -> Result<String, String> {
    let program = which(command).ok_or_else(|| format!("{command} is not on PATH"))?;
    let mut child = Command::new(&program)
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|e| e.to_string())?;
    let request = json!({
        "jsonrpc": "2.0", "id": 1, "method": "initialize",
        "params": { "protocolVersion": "2025-06-18", "capabilities": {}, "clientInfo": { "name": "creative-mcp", "version": env!("CARGO_PKG_VERSION") } }
    });
    // Keep stdin open until we have an answer; some servers exit on EOF.
    let mut stdin = child.stdin.take();
    if let Some(input) = stdin.as_mut() {
        let _ = writeln!(input, "{request}");
    }
    let stdout = child.stdout.take().ok_or("no stdout")?;
    let (tx, rx) = mpsc::channel();
    std::thread::spawn(move || {
        for line in BufReader::new(stdout).lines().map_while(Result::ok) {
            if let Ok(v) = serde_json::from_str::<Value>(&line) {
                if v.get("id") == Some(&json!(1)) {
                    let _ = tx.send(v);
                    return;
                }
            }
        }
    });
    let result = rx.recv_timeout(timeout);
    drop(stdin);
    let _ = child.kill();
    let _ = child.wait();
    let v = result.map_err(|_| format!("no initialize response within {}s", timeout.as_secs()))?;
    if let Some(err) = v.get("error") {
        return Err(format!("initialize error: {err}"));
    }
    let info = &v["result"]["serverInfo"];
    Ok(format!(
        "{} {}",
        info["name"].as_str().unwrap_or("?"),
        info["version"].as_str().unwrap_or("")
    )
    .trim()
    .to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn json_registration_backs_up_and_preserves_other_servers() {
        let dir = std::env::temp_dir().join(format!("creative-mcp-test-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("mcp.json");
        std::fs::write(
            &path,
            r#"{"mcpServers":{"other":{"command":"x"}},"theme":"dark"}"#,
        )
        .unwrap();
        let note = write_json(
            path.to_str().unwrap(),
            "blender",
            "blender-mcp",
            &["serve".into()],
        )
        .unwrap();
        assert!(note.starts_with("backup: "));
        // A second edit in the same run must not replace the backup of the original.
        write_json(
            path.to_str().unwrap(),
            "photoshop",
            "adobe-mcp-photoshop",
            &[],
        )
        .unwrap();
        let backup_path = note.trim_start_matches("backup: ");
        let original: Value =
            serde_json::from_str(&std::fs::read_to_string(backup_path).unwrap()).unwrap();
        assert!(
            original["mcpServers"].get("blender").is_none(),
            "backup must hold the original file"
        );
        assert_eq!(original["mcpServers"]["other"]["command"], "x");
        let v: Value = serde_json::from_str(&std::fs::read_to_string(&path).unwrap()).unwrap();
        assert_eq!(v["theme"], "dark");
        assert_eq!(v["mcpServers"]["other"]["command"], "x");
        assert_eq!(v["mcpServers"]["blender"]["args"][0], "serve");
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn invalid_json_is_left_alone() {
        let dir = std::env::temp_dir().join(format!("creative-mcp-bad-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("mcp.json");
        std::fs::write(&path, "{ not json").unwrap();
        assert!(write_json(path.to_str().unwrap(), "a", "b", &[]).is_err());
        assert_eq!(std::fs::read_to_string(&path).unwrap(), "{ not json");
        std::fs::remove_dir_all(&dir).unwrap();
    }
}
