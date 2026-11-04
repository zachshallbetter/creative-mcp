//! The series catalogue (`registry/registry.json`), embedded at build time.

use serde::Deserialize;
use std::collections::BTreeMap;

pub const EMBEDDED: &str = include_str!("../../registry/registry.json");

#[derive(Debug, Clone, Deserialize)]
pub struct Registry {
    pub updated: String,
    pub tools: Vec<Tool>,
    pub clients: Vec<Client>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Tool {
    pub id: String,
    pub name: String,
    pub summary: String,
    pub docs: String,
    pub status: String,
    pub install: Install,
    #[serde(default)]
    pub requires: Vec<String>,
    pub servers: Vec<Server>,
    #[serde(default)]
    pub post_install: Vec<String>,
    /// Read-only commands `doctor` runs; a non-zero exit is a problem.
    #[serde(default)]
    pub checks: Vec<String>,
    /// Example requests shown after setup.
    #[serde(default, rename = "try")]
    pub examples: Vec<String>,
    pub targets: Vec<Target>,
    pub clients: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Install {
    pub brew: Option<Brew>,
    pub npm: Option<Npm>,
    pub cargo: Option<Cargo>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Brew {
    pub formula: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Npm {
    pub package: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Cargo {
    pub git: Option<String>,
    pub package: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Server {
    pub command: String,
    #[serde(default)]
    pub args: Vec<String>,
    pub app: Option<String>,
    pub register_as: String,
    #[serde(default)]
    pub requires_apps: Vec<String>,
    /// A tool on this server that reports whether its app is reachable. Called only while the app runs,
    /// because some status tools start the app.
    pub status: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Target {
    pub app: String,
    pub name: String,
    pub detect: BTreeMap<String, Detect>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct Detect {
    #[serde(default)]
    pub bundle_ids: Vec<String>,
    #[serde(default)]
    pub bundle_id_prefixes: Vec<String>,
    #[serde(default)]
    pub commands: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Client {
    pub id: String,
    pub name: String,
    /// How to restart the client so it loads new servers; CLI clients pick them up in a new session.
    pub restart: Option<String>,
    pub detect: ClientDetect,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct ClientDetect {
    #[serde(default)]
    pub commands: Vec<String>,
    /// Where the client's CLI may live when it isn't on PATH (`*` matches one path segment; newest wins).
    #[serde(default)]
    pub paths: Vec<String>,
    #[serde(default)]
    pub files: Vec<String>,
}

impl Registry {
    pub fn load(path: Option<&str>) -> Result<Registry, String> {
        let text = match path {
            Some(p) => std::fs::read_to_string(p).map_err(|e| format!("cannot read {p}: {e}"))?,
            None => EMBEDDED.to_string(),
        };
        serde_json::from_str(&text).map_err(|e| format!("invalid registry: {e}"))
    }

    pub fn tool(&self, id: &str) -> Option<&Tool> {
        self.tools.iter().find(|t| t.id == id)
    }

    pub fn client(&self, id: &str) -> Option<&Client> {
        self.clients.iter().find(|c| c.id == id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn embedded_registry_parses_and_cross_references() {
        let r = Registry::load(None).expect("embedded registry parses");
        assert!(r.tool("adobe").is_some());
        assert!(r.tool("blender").is_some());
        for tool in &r.tools {
            for c in &tool.clients {
                assert!(
                    r.client(c).is_some(),
                    "{} references unknown client {c}",
                    tool.id
                );
            }
            for s in &tool.servers {
                if let Some(app) = &s.app {
                    assert!(
                        tool.targets.iter().any(|t| &t.app == app),
                        "{} server targets unknown app {app}",
                        tool.id
                    );
                }
            }
        }
    }
}
