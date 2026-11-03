//! Turns a selection (tools, detected apps, clients) into an explicit list of steps.
//! Nothing here touches the system; `exec` carries the plan out.

use crate::detect::{FoundApp, FoundClient};
use crate::registry::{Server, Tool};
use serde::Serialize;

pub const TAP: &str = "zachshallbetter/tap";

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Step {
    /// Run a program. `allow_failure` marks idempotent steps (e.g. adding an entry that may exist).
    /// `server_arg` is the index of the MCP server command in `args`; it is resolved to an absolute path at run time.
    Run {
        why: String,
        program: String,
        args: Vec<String>,
        allow_failure: bool,
        server_arg: Option<usize>,
    },
    /// Add or replace `mcpServers.<name>` in a client's JSON config (after backing it up).
    WriteJson {
        why: String,
        path: String,
        name: String,
        command: String,
        args: Vec<String>,
    },
    /// Something the user has to do; printed, never executed.
    Manual { why: String, text: String },
}

#[derive(Debug, Clone, Default)]
pub struct Environment {
    pub brew: bool,
    pub brew_has_trust: bool,
    pub tap_installed: bool,
    pub npm: bool,
    pub cargo: bool,
    /// Formula short names already installed with Homebrew.
    pub installed_formulae: Vec<String>,
}

pub fn formula_name(formula: &str) -> &str {
    formula.rsplit('/').next().unwrap_or(formula)
}

fn run(why: impl Into<String>, program: &str, args: &[&str]) -> Step {
    Step::Run {
        why: why.into(),
        program: program.into(),
        args: args.iter().map(|s| s.to_string()).collect(),
        allow_failure: false,
        server_arg: None,
    }
}

/// Install steps for one tool, preferring Homebrew.
pub fn install_steps(tool: &Tool, env: &Environment, tap_steps_added: &mut bool) -> Vec<Step> {
    let mut steps = Vec::new();
    if let (true, Some(brew)) = (env.brew, &tool.install.brew) {
        if !*tap_steps_added {
            if !env.tap_installed {
                steps.push(run(
                    "Add the Creative MCP Homebrew tap",
                    "brew",
                    &["tap", TAP],
                ));
            }
            if env.brew_has_trust {
                steps.push(run(
                    "Trust the tap (Homebrew 7+ requires this once)",
                    "brew",
                    &["trust", TAP],
                ));
            }
            *tap_steps_added = true;
        }
        if env
            .installed_formulae
            .iter()
            .any(|f| f == formula_name(&brew.formula))
        {
            let mut upgrade = run(
                format!("Upgrade {} if a newer version exists", tool.name),
                "brew",
                &["upgrade", &brew.formula],
            );
            if let Step::Run { allow_failure, .. } = &mut upgrade {
                *allow_failure = true;
            }
            steps.push(upgrade);
        } else {
            steps.push(run(
                format!("Install {}", tool.name),
                "brew",
                &["install", &brew.formula],
            ));
        }
    } else if let (true, Some(npm)) = (env.npm, &tool.install.npm) {
        steps.push(run(
            format!("Install {} with npm", tool.name),
            "npm",
            &["install", "-g", &npm.package],
        ));
    } else if let (true, Some(cargo)) = (env.cargo, &tool.install.cargo) {
        let mut args = vec!["install", "--locked"];
        if let Some(git) = &cargo.git {
            args.extend(["--git", git]);
        }
        args.extend(["--package", &cargo.package]);
        steps.push(run(
            format!("Build and install {} with Cargo", tool.name),
            "cargo",
            &args,
        ));
    } else {
        steps.push(Step::Manual {
            why: format!("No supported installer found for {}", tool.name),
            text: format!(
                "Install Homebrew (https://brew.sh), then run: brew install {}",
                tool.install
                    .brew
                    .as_ref()
                    .map(|b| b.formula.as_str())
                    .unwrap_or(&tool.id)
            ),
        });
    }
    for post in &tool.post_install {
        let mut parts = post.split_whitespace();
        if let Some(program) = parts.next() {
            let args: Vec<&str> = parts.collect();
            steps.push(run(format!("{}: {post}", tool.name), program, &args));
        }
    }
    steps
}

/// Which of a tool's servers to register, given the apps found for it.
pub fn servers_for<'a>(tool: &'a Tool, apps: &[FoundApp]) -> Vec<&'a Server> {
    let found: Vec<&str> = apps
        .iter()
        .filter(|a| a.tool == tool.id)
        .map(|a| a.app.as_str())
        .collect();
    if found.is_empty() {
        // Selected explicitly without a detected app: register every server so nothing is missing.
        return tool.servers.iter().collect();
    }
    tool.servers
        .iter()
        .filter(|s| s.app.as_deref().is_none_or(|a| found.contains(&a)))
        .filter(|s| s.requires_apps.iter().all(|a| found.contains(&a.as_str())))
        .collect()
}

/// How to register one server with one client.
pub fn register_step(client: &FoundClient, server: &Server) -> Step {
    let name = server.register_as.clone();
    let command = server.command.clone();
    let mut tail: Vec<String> = vec![command.clone()];
    tail.extend(server.args.iter().cloned());
    let why = format!("Register {name} with {}", client.name);
    let cli = |program: &str, mut args: Vec<String>| {
        let server_arg = Some(args.len());
        args.extend(tail.clone());
        Step::Run {
            why: why.clone(),
            program: program.into(),
            args,
            allow_failure: true,
            server_arg,
        }
    };
    match (client.id.as_str(), &client.command) {
        ("claude-code", Some(_)) => cli("claude", vec!["mcp".into(), "add".into(), "--scope".into(), "user".into(), name, "--".into()]),
        ("codex", Some(_)) => cli("codex", vec!["mcp".into(), "add".into(), name, "--".into()]),
        ("gemini", Some(_)) => cli("gemini", vec!["mcp".into(), "add".into(), "--scope".into(), "user".into(), name]),
        ("claude-desktop", _) | ("cursor", _) if !client.files.is_empty() => Step::WriteJson {
            why,
            path: client.files[0].clone(),
            name,
            command,
            args: server.args.clone(),
        },
        ("claude-code", None) => Step::Manual {
            why,
            text: format!("Run in a terminal where `claude` works: claude mcp add --scope user {name} -- {}", tail.join(" ")),
        },
        _ => Step::Manual {
            why,
            text: format!(
                "Add to {}'s MCP config: \"{name}\": {{ \"command\": \"{command}\", \"args\": {} }}",
                client.name,
                serde_json::to_string(&server.args).unwrap_or_else(|_| "[]".into())
            ),
        },
    }
}

pub fn build(
    tools: &[&Tool],
    apps: &[FoundApp],
    clients: &[FoundClient],
    env: &Environment,
    install: bool,
) -> Vec<Step> {
    let mut steps = Vec::new();
    let mut tap_added = false;
    if install {
        for tool in tools {
            steps.extend(install_steps(tool, env, &mut tap_added));
        }
    }
    for tool in tools {
        for client in clients.iter().filter(|c| tool.clients.contains(&c.id)) {
            for server in servers_for(tool, apps) {
                steps.push(register_step(client, server));
            }
        }
    }
    steps
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::registry::Registry;

    fn app(tool: &str, app: &str) -> FoundApp {
        FoundApp {
            tool: tool.into(),
            app: app.into(),
            name: app.into(),
            path: "/Applications/x.app".into(),
            version: None,
        }
    }

    fn client(id: &str, command: bool, file: Option<&str>) -> FoundClient {
        FoundClient {
            id: id.into(),
            name: id.into(),
            command: command.then(|| format!("/bin/{id}")),
            files: file.into_iter().map(String::from).collect(),
        }
    }

    #[test]
    fn coordinator_needs_both_illustrator_and_indesign() {
        let r = Registry::load(None).unwrap();
        let adobe = r.tool("adobe").unwrap();
        let only_ps = servers_for(adobe, &[app("adobe", "photoshop")]);
        assert_eq!(
            only_ps
                .iter()
                .map(|s| s.register_as.as_str())
                .collect::<Vec<_>>(),
            vec!["photoshop"]
        );
        let both = servers_for(
            adobe,
            &[app("adobe", "illustrator"), app("adobe", "indesign")],
        );
        let names: Vec<_> = both.iter().map(|s| s.register_as.as_str()).collect();
        assert!(
            names.contains(&"adobe")
                && names.contains(&"illustrator")
                && names.contains(&"indesign")
        );
    }

    #[test]
    fn homebrew_preferred_and_tap_added_once() {
        let r = Registry::load(None).unwrap();
        let env = Environment {
            brew: true,
            brew_has_trust: true,
            npm: true,
            cargo: true,
            ..Default::default()
        };
        let tools = vec![r.tool("adobe").unwrap(), r.tool("blender").unwrap()];
        let steps = build(&tools, &[], &[], &env, true);
        let taps = steps.iter().filter(|s| matches!(s, Step::Run { args, .. } if args.first().map(String::as_str) == Some("tap"))).count();
        assert_eq!(taps, 1);
        assert!(steps.iter().any(|s| matches!(s, Step::Run { program, args, .. } if program == "brew" && args == &["install", "zachshallbetter/tap/blender-mcp"])));
        assert!(steps.iter().any(|s| matches!(s, Step::Run { program, args, .. } if program == "blender-mcp" && args == &["addon", "install"])));
    }

    #[test]
    fn registration_uses_each_clients_own_mechanism() {
        let r = Registry::load(None).unwrap();
        let blender = r.tool("blender").unwrap();
        let server = &blender.servers[0];
        match register_step(&client("codex", true, None), server) {
            Step::Run { program, args, .. } => {
                assert_eq!(program, "codex");
                assert_eq!(
                    args,
                    vec!["mcp", "add", "blender", "--", "blender-mcp", "serve"]
                );
            }
            other => panic!("unexpected {other:?}"),
        }
        assert!(matches!(
            register_step(&client("cursor", false, Some("/tmp/mcp.json")), server),
            Step::WriteJson { .. }
        ));
        assert!(matches!(
            register_step(
                &client("claude-code", false, Some("/tmp/.claude.json")),
                server
            ),
            Step::Manual { .. }
        ));
    }
}
