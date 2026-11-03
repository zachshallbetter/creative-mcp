//! creative-mcp: set up the Creative MCP series on this machine.
//! Detects creative apps and MCP clients, installs the matching tools, registers them, and checks they answer.

mod detect;
mod exec;
mod plan;
mod registry;

use detect::{FoundApp, FoundClient};
use registry::{Registry, Tool};
use serde_json::json;
use std::io::{BufRead, IsTerminal, Write};
use std::process::ExitCode;
use std::time::Duration;

const HELP: &str = "creative-mcp: set up Creative MCP servers for the creative apps on this machine

USAGE:
    creative-mcp <COMMAND> [OPTIONS]

COMMANDS:
    setup     Detect apps and MCP clients, pick tools, install, register and verify
    detect    Show which creative apps and MCP clients were found
    list      Show every tool in the catalogue and whether it is installed
    doctor    Check installed tools answer MCP initialize and are registered
    help      Show this message

OPTIONS:
    -y, --yes            Accept the detected selection without prompting
        --dry-run        Print the plan and change nothing
        --tools <ids>    Only these tools (comma-separated, e.g. adobe,blender)
        --clients <ids>  Only these clients (e.g. claude-code,codex,cursor)
        --no-install     Skip installing; only register and verify
        --no-verify      Skip the MCP initialize check
        --json           Machine-readable output (detect, list, doctor)
        --registry <f>   Use a catalogue file instead of the built-in one
    -V, --version        Print the version

Detection reads only app bundles, PATH and client config files on this machine.
Nothing is sent anywhere. Docs: https://creativemcp.com/docs/architecture.html";

#[derive(Default)]
struct Options {
    command: String,
    yes: bool,
    dry_run: bool,
    json: bool,
    install: bool,
    verify: bool,
    tools: Option<Vec<String>>,
    clients: Option<Vec<String>>,
    registry: Option<String>,
}

fn parse_args() -> Result<Options, String> {
    let mut o = Options {
        install: true,
        verify: true,
        ..Default::default()
    };
    let mut args = std::env::args().skip(1);
    let list = |v: Option<String>, flag: &str| -> Result<Vec<String>, String> {
        let v = v.ok_or(format!("{flag} needs a value"))?;
        Ok(v.split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect())
    };
    while let Some(a) = args.next() {
        match a.as_str() {
            "-y" | "--yes" => o.yes = true,
            "--dry-run" => o.dry_run = true,
            "--json" => o.json = true,
            "--no-install" => o.install = false,
            "--no-verify" => o.verify = false,
            "--tools" => o.tools = Some(list(args.next(), "--tools")?),
            "--clients" => o.clients = Some(list(args.next(), "--clients")?),
            "--registry" => o.registry = Some(args.next().ok_or("--registry needs a path")?),
            "-V" | "--version" => o.command = "version".into(),
            "-h" | "--help" => o.command = "help".into(),
            s if s.starts_with('-') => return Err(format!("unknown option {s}")),
            s if o.command.is_empty() => o.command = s.to_string(),
            s => return Err(format!("unexpected argument {s}")),
        }
    }
    if o.command.is_empty() {
        o.command = "help".into();
    }
    Ok(o)
}

fn ask(question: &str, default_yes: bool) -> bool {
    print!(
        "{question} {} ",
        if default_yes { "[Y/n]" } else { "[y/N]" }
    );
    let _ = std::io::stdout().flush();
    let mut line = String::new();
    if std::io::stdin().lock().read_line(&mut line).unwrap_or(0) == 0 {
        return default_yes;
    }
    match line.trim().to_lowercase().as_str() {
        "" => default_yes,
        "y" | "yes" => true,
        _ => false,
    }
}

fn apps_for<'a>(apps: &'a [FoundApp], tool: &str) -> Vec<&'a FoundApp> {
    apps.iter().filter(|a| a.tool == tool).collect()
}

fn installed(tool: &Tool) -> bool {
    tool.servers
        .iter()
        .any(|s| detect::which(&s.command).is_some())
}

fn print_detection(registry: &Registry, apps: &[FoundApp], clients: &[FoundClient]) {
    println!("Creative apps on this machine:");
    let mut any = false;
    for tool in &registry.tools {
        for a in apps_for(apps, &tool.id) {
            any = true;
            println!(
                "  • {:<22} {:<10} → {}   {}",
                a.name,
                a.version.as_deref().unwrap_or(""),
                tool.name,
                a.path
            );
        }
    }
    if !any {
        println!("  (none found)");
    }
    println!("\nMCP clients:");
    if clients.is_empty() {
        println!("  (none found)");
    }
    for c in clients {
        let how = c
            .command
            .as_deref()
            .map(|p| format!("command {p}"))
            .unwrap_or_else(|| format!("config {}", c.files.join(", ")));
        println!("  • {:<16} {how}", c.name);
    }
}

fn cmd_detect(registry: &Registry, o: &Options) -> ExitCode {
    let apps = detect::detect_apps(registry);
    let clients = detect::detect_clients(registry);
    if o.json {
        println!(
            "{}",
            json!({ "os": detect::os_key(), "apps": apps, "clients": clients })
        );
    } else {
        print_detection(registry, &apps, &clients);
    }
    ExitCode::SUCCESS
}

fn cmd_list(registry: &Registry, o: &Options) -> ExitCode {
    if o.json {
        let rows: Vec<_> = registry
            .tools
            .iter()
            .map(|t| json!({ "id": t.id, "name": t.name, "status": t.status, "installed": installed(t), "docs": t.docs,
                              "brew": t.install.brew.as_ref().map(|b| &b.formula), "servers": t.servers.iter().map(|s| &s.command).collect::<Vec<_>>() }))
            .collect();
        println!(
            "{}",
            json!({ "catalogue": registry.updated, "tools": rows })
        );
        return ExitCode::SUCCESS;
    }
    println!("Creative MCP catalogue ({}):\n", registry.updated);
    for t in &registry.tools {
        println!(
            "  {} [{}] {}  ({})",
            t.name,
            t.id,
            if installed(t) {
                "installed"
            } else {
                "not installed"
            },
            t.status
        );
        println!("    {}", t.summary);
        if let Some(b) = &t.install.brew {
            println!("    brew install {}", b.formula);
        }
        if !t.requires.is_empty() {
            println!(
                "    requires {} (Homebrew installs it for you)",
                t.requires.join(", ")
            );
        }
        println!("    {}\n", t.docs);
    }
    ExitCode::SUCCESS
}

fn cmd_doctor(registry: &Registry, o: &Options) -> ExitCode {
    let apps = detect::detect_apps(registry);
    let clients = detect::detect_clients(registry);
    let mut problems = 0;
    let mut report = Vec::new();
    for tool in &registry.tools {
        for server in plan::servers_for(tool, &apps) {
            let state = if detect::which(&server.command).is_none() {
                if apps_for(&apps, &tool.id).is_empty() {
                    continue; // tool not relevant here
                }
                problems += 1;
                Err(format!(
                    "not installed (brew install {})",
                    tool.install
                        .brew
                        .as_ref()
                        .map(|b| b.formula.as_str())
                        .unwrap_or(&tool.id)
                ))
            } else {
                exec::verify(&server.command, &server.args, Duration::from_secs(20))
                    .inspect_err(|_| problems += 1)
            };
            if !o.json {
                match &state {
                    Ok(info) => println!("  ✓ {:<26} answers initialize ({info})", server.command),
                    Err(e) => println!("  ✗ {:<26} {e}", server.command),
                }
            }
            report.push(json!({ "tool": tool.id, "command": server.command, "ok": state.is_ok(), "detail": state.unwrap_or_else(|e| e) }));
        }
    }
    if o.json {
        println!(
            "{}",
            json!({ "apps": apps, "clients": clients, "servers": report, "problems": problems })
        );
    } else {
        if report.is_empty() {
            println!("  No Creative MCP tools are relevant to the apps on this machine. Run `creative-mcp detect`.");
        }
        println!(
            "\n{}",
            if problems == 0 {
                "No problems found.".to_string()
            } else {
                format!("{problems} problem(s). Run `creative-mcp setup` to fix installs and registrations.")
            }
        );
    }
    if problems == 0 {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}

fn cmd_setup(registry: &Registry, o: &Options) -> ExitCode {
    let interactive = !o.yes && std::io::stdin().is_terminal();
    let apps = detect::detect_apps(registry);
    let found_clients = detect::detect_clients(registry);
    print_detection(registry, &apps, &found_clients);

    // 1. Tools
    let mut tools: Vec<&Tool> = Vec::new();
    for tool in &registry.tools {
        let found = apps_for(&apps, &tool.id);
        let wanted = match &o.tools {
            Some(ids) => ids.contains(&tool.id),
            None if found.is_empty() => false,
            None => {
                let names = found
                    .iter()
                    .map(|a| a.name.as_str())
                    .collect::<Vec<_>>()
                    .join(", ");
                !interactive || ask(&format!("\nSet up {} for {names}?", tool.name), true)
            }
        };
        if wanted {
            tools.push(tool);
        }
    }
    let known =
        |ids: &[String], valid: &dyn Fn(&str) -> bool, kind: &str, all: Vec<&str>| -> bool {
            match ids.iter().find(|id| !valid(id)) {
                Some(id) => {
                    eprintln!("Unknown {kind} '{id}'. Known: {}", all.join(", "));
                    false
                }
                None => true,
            }
        };
    if let Some(ids) = &o.tools {
        if !known(
            ids,
            &|id| registry.tool(id).is_some(),
            "tool",
            registry.tools.iter().map(|t| t.id.as_str()).collect(),
        ) {
            return ExitCode::FAILURE;
        }
    }
    if let Some(ids) = &o.clients {
        if !known(
            ids,
            &|id| registry.client(id).is_some(),
            "client",
            registry.clients.iter().map(|c| c.id.as_str()).collect(),
        ) {
            return ExitCode::FAILURE;
        }
    }
    if tools.is_empty() {
        println!("\nNothing to set up. Use --tools adobe,blender to choose tools without a detected app.");
        return ExitCode::SUCCESS;
    }

    // 2. Clients
    let mut clients: Vec<FoundClient> = Vec::new();
    for c in &found_clients {
        let relevant = tools.iter().any(|t| t.clients.contains(&c.id));
        let wanted = match &o.clients {
            Some(ids) => ids.contains(&c.id),
            None => relevant && (!interactive || ask(&format!("Register with {}?", c.name), true)),
        };
        if wanted && relevant {
            clients.push(c.clone());
        }
    }

    // 3. Plan
    let env = exec::environment();
    let steps = plan::build(&tools, &apps, &clients, &env, o.install);
    println!(
        "\nPlan ({} step{}):",
        steps.len(),
        if steps.len() == 1 { "" } else { "s" }
    );
    for (i, s) in steps.iter().enumerate() {
        println!("  {}. {}", i + 1, exec::describe(s));
    }
    if o.dry_run {
        println!("\nDry run: nothing was changed.");
        return ExitCode::SUCCESS;
    }
    if interactive && !ask("\nProceed?", false) {
        println!("Cancelled. Nothing was changed.");
        return ExitCode::SUCCESS;
    }

    // 4. Execute
    let outcome = exec::execute(&steps);

    // 5. Verify
    let mut verify_failures = 0;
    if o.verify {
        println!("\nChecking each server answers MCP initialize:");
        for tool in &tools {
            for server in plan::servers_for(tool, &apps) {
                match exec::verify(&server.command, &server.args, Duration::from_secs(20)) {
                    Ok(info) => println!("  ✓ {:<26} {info}", server.command),
                    Err(e) => {
                        verify_failures += 1;
                        println!("  ✗ {:<26} {e}", server.command);
                    }
                }
            }
        }
    }

    println!(
        "\nDone: {} step(s) completed, {} failed.",
        outcome.ok,
        outcome.failed.len()
    );
    for f in &outcome.failed {
        println!("  ✗ {f}");
    }
    if !outcome.manual.is_empty() {
        println!("\nFinish these yourself:");
        for m in &outcome.manual {
            println!("  → {m}");
        }
    }
    println!("\nRestart your MCP clients to load the new servers. Check later with `creative-mcp doctor`.");
    if outcome.failed.is_empty() && verify_failures == 0 {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}

fn main() -> ExitCode {
    let o = match parse_args() {
        Ok(o) => o,
        Err(e) => {
            eprintln!("{e}\n\n{HELP}");
            return ExitCode::from(2);
        }
    };
    if o.command == "help" {
        println!("{HELP}");
        return ExitCode::SUCCESS;
    }
    if o.command == "version" {
        println!("creative-mcp {}", env!("CARGO_PKG_VERSION"));
        return ExitCode::SUCCESS;
    }
    let registry = match Registry::load(o.registry.as_deref()) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("{e}");
            return ExitCode::FAILURE;
        }
    };
    match o.command.as_str() {
        "setup" => cmd_setup(&registry, &o),
        "detect" => cmd_detect(&registry, &o),
        "list" => cmd_list(&registry, &o),
        "doctor" => cmd_doctor(&registry, &o),
        other => {
            eprintln!("unknown command '{other}'\n\n{HELP}");
            ExitCode::from(2)
        }
    }
}
