//! Reads which MCP servers each client already has registered, from its own config file.
//! Read-only: changes go through `plan` and `exec`.

use crate::detect::{expand, which};
use crate::registry::{Registry, Server};
use serde::Serialize;
use serde_json::Value;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

/// One registered server: the command and arguments the client will start.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Entry {
    pub command: String,
    pub args: Vec<String>,
}

/// How a registered entry compares with what the catalogue would register.
#[derive(Debug, Clone, PartialEq)]
pub enum State {
    Missing,
    /// Registered with the same program and arguments.
    Current,
    /// Registered under this name, but with another command (an npx line, an old path).
    Different(Entry),
}

/// The config file that holds a client's user-level servers: the first of its catalogue files that
/// exists (they differ by platform), or the first one listed.
pub fn config_path(registry: &Registry, client_id: &str) -> Option<PathBuf> {
    let files: Vec<PathBuf> = registry
        .client(client_id)?
        .detect
        .files
        .iter()
        .map(|f| expand(f))
        .collect();
    files.iter().find(|p| p.exists()).or(files.first()).cloned()
}

/// Registered servers by name. `None` when the client keeps its config somewhere this tool can't read.
pub fn registered(path: &Path) -> Option<BTreeMap<String, Entry>> {
    let text = match std::fs::read_to_string(path) {
        Ok(t) => t,
        Err(_) if !path.exists() => return Some(BTreeMap::new()),
        Err(_) => return None,
    };
    let entry = |command: Option<&str>, args: Option<Vec<String>>| {
        command.map(|c| Entry {
            command: c.to_string(),
            args: args.unwrap_or_default(),
        })
    };
    let mut out = BTreeMap::new();
    if path.extension().is_some_and(|e| e == "toml") {
        // Codex: [mcp_servers.<name>] command = "...", args = [...]
        let doc: toml::Table = text.parse().ok()?;
        if let Some(servers) = doc.get("mcp_servers").and_then(|v| v.as_table()) {
            for (name, v) in servers {
                let args = v.get("args").and_then(|a| a.as_array()).map(|a| {
                    a.iter()
                        .filter_map(|x| x.as_str().map(String::from))
                        .collect()
                });
                if let Some(e) = entry(v.get("command").and_then(|c| c.as_str()), args) {
                    out.insert(name.clone(), e);
                }
            }
        }
    } else {
        // Claude Code (~/.claude.json, user scope), Claude Desktop, Cursor, Gemini CLI: { "mcpServers": { name: {command, args} } }
        let doc: Value = if text.trim().is_empty() {
            Value::Null
        } else {
            serde_json::from_str(&text).ok()?
        };
        if let Some(servers) = doc.get("mcpServers").and_then(|v| v.as_object()) {
            for (name, v) in servers {
                let args = v.get("args").and_then(|a| a.as_array()).map(|a| {
                    a.iter()
                        .filter_map(|x| x.as_str().map(String::from))
                        .collect()
                });
                if let Some(e) = entry(v.get("command").and_then(|c| c.as_str()), args) {
                    out.insert(name.clone(), e);
                }
            }
        }
    }
    Some(out)
}

/// Same program if the names match, or both resolve to the same file (a bare name vs. its absolute path).
fn same_program(registered: &str, wanted: &str) -> bool {
    if registered == wanted {
        return true;
    }
    let file_name = |p: &str| {
        Path::new(p)
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
    };
    if file_name(registered).as_deref() != Some(wanted) {
        return false;
    }
    // An absolute path counts only while it still exists; a stale path is a different registration.
    Path::new(registered).exists()
        && which(wanted).is_none_or(|p| {
            std::fs::canonicalize(&p).ok() == std::fs::canonicalize(registered).ok()
                || p == Path::new(registered)
        })
}

pub fn state(existing: &BTreeMap<String, Entry>, server: &Server) -> State {
    match existing.get(&server.register_as) {
        None => State::Missing,
        Some(e) if same_program(&e.command, &server.command) && e.args == server.args => {
            State::Current
        }
        Some(e) => State::Different(e.clone()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn server(command: &str, args: &[&str], name: &str) -> Server {
        Server {
            command: command.into(),
            args: args.iter().map(|s| s.to_string()).collect(),
            app: None,
            register_as: name.into(),
            requires_apps: Vec::new(),
            status: None,
        }
    }

    fn tmp(name: &str, body: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("creative-mcp-clients-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let p = dir.join(name);
        std::fs::write(&p, body).unwrap();
        p
    }

    #[test]
    fn reads_json_and_toml_configs() {
        let json = tmp(
            "settings.json",
            r#"{"theme":"x","mcpServers":{"blender":{"command":"blender-mcp","args":["serve"]},"web":{"url":"https://x"}}}"#,
        );
        let got = registered(&json).unwrap();
        assert_eq!(got.len(), 1, "URL servers have no command and are skipped");
        assert_eq!(got["blender"].args, vec!["serve"]);

        let toml = tmp(
            "config.toml",
            "model = \"x\"\n[mcp_servers.photoshop]\ncommand = \"/opt/homebrew/bin/adobe-mcp-photoshop\"\nargs = []\n",
        );
        let got = registered(&toml).unwrap();
        assert_eq!(
            got["photoshop"].command,
            "/opt/homebrew/bin/adobe-mcp-photoshop"
        );

        assert!(registered(&tmp("broken.json", "{ nope")).is_none());
        assert!(registered(Path::new("/definitely/not/here.json"))
            .unwrap()
            .is_empty());
    }

    #[test]
    fn compares_by_program_and_arguments() {
        let mut existing = BTreeMap::new();
        existing.insert(
            "blender".to_string(),
            Entry {
                command: "blender-mcp".into(),
                args: vec!["serve".into()],
            },
        );
        existing.insert(
            "photoshop".to_string(),
            Entry {
                command: "npx".into(),
                args: vec!["-y".into(), "adobe-mcp-photoshop".into()],
            },
        );
        existing.insert(
            "illustrator".to_string(),
            Entry {
                command: "/gone/bin/adobe-mcp-illustrator".into(),
                args: vec![],
            },
        );
        assert_eq!(
            state(&existing, &server("blender-mcp", &["serve"], "blender")),
            State::Current
        );
        assert!(matches!(
            state(&existing, &server("adobe-mcp-photoshop", &[], "photoshop")),
            State::Different(_)
        ));
        assert!(matches!(
            state(
                &existing,
                &server("adobe-mcp-illustrator", &[], "illustrator")
            ),
            State::Different(_)
        ));
        assert_eq!(
            state(&existing, &server("adobe-mcp-indesign", &[], "indesign")),
            State::Missing
        );
    }
}
