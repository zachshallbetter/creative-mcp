//! creative-mcp: set up the Creative MCP series on this machine.
//! Detects creative apps and MCP clients, installs the matching tools, registers them, and checks they answer.

mod clients;
mod detect;
mod exec;
mod plan;
mod registry;

use clients::State;
use detect::{FoundApp, FoundClient};
use plan::{Existing, Step};
use registry::{Registry, Tool};
use serde_json::{json, Value};
use std::collections::BTreeMap;
use std::io::{BufRead, IsTerminal, Write};
use std::process::{Command, ExitCode};
use std::time::Duration;

const HELP: &str = "creative-mcp: set up Creative MCP servers for the creative apps on this machine

USAGE:
    creative-mcp <COMMAND> [TOOLS] [OPTIONS]

COMMANDS:
    setup     Detect apps and MCP clients, pick tools, install, register and verify
    add       Set up named tools even without a detected app (creative-mcp add blender)
    update    Upgrade installed tools and refresh their registrations
    remove    Unregister tools from every client and uninstall them
    doctor    Check servers answer, apps are reachable and clients have them registered
    detect    Show which creative apps and MCP clients were found
    list      Show every tool in the catalogue and whether it is installed
    help      Show this message

OPTIONS:
    -y, --yes            Accept the detected selection without prompting
        --dry-run        Print the plan and change nothing
        --tools <ids>    Only these tools (comma-separated, e.g. adobe,blender)
        --clients <ids>  Only these clients (e.g. claude-code,codex,cursor)
        --no-install     setup: only register and verify; remove: keep the tools installed
        --no-verify      Skip the MCP initialize check
        --json           Machine-readable output (detect, list, doctor)
        --registry <f>   Use a catalogue file instead of the built-in one
    -V, --version        Print the version

Every change is shown before it is made, and client configs are backed up first.
Detection reads only app bundles, PATH and client config files on this machine.
Nothing is sent anywhere. Docs: https://creativemcp.com/docs/cli.html";

const VERIFY_TIMEOUT: Duration = Duration::from_secs(20);

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

fn parse_args(args: impl IntoIterator<Item = String>) -> Result<Options, String> {
    let mut o = Options {
        install: true,
        verify: true,
        ..Default::default()
    };
    let mut args = args.into_iter();
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
            // `add blender`, `remove adobe,blender`: tool ids after the command.
            s if matches!(o.command.as_str(), "add" | "remove" | "update" | "setup") => o
                .tools
                .get_or_insert_with(Vec::new)
                .extend(list(Some(s.to_string()), "tools")?),
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

/// Each detected client's user-level config file, when it has one.
fn configs(registry: &Registry, clients: &[FoundClient]) -> BTreeMap<String, String> {
    clients
        .iter()
        .filter_map(|c| {
            clients::config_path(registry, &c.id).map(|p| (c.id.clone(), p.display().to_string()))
        })
        .collect()
}

fn existing(configs: &BTreeMap<String, String>, clients: &[FoundClient]) -> Existing {
    clients
        .iter()
        .map(|c| {
            let have = configs
                .get(&c.id)
                .and_then(|p| clients::registered(std::path::Path::new(p)));
            (c.id.clone(), have)
        })
        .collect()
}

/// Whether a detected app is running, from the process list. Never starts it.
/// macOS: a process inside the app bundle. Windows: the executable's image name in `tasklist`.
/// Linux: a process whose program is the detected executable.
fn app_running(app: &FoundApp) -> Option<bool> {
    match detect::os_key() {
        "macos" if app.path.ends_with(".app") => {
            let inside = format!("{}/Contents/MacOS/", app.path);
            Some(
                process_lines(&["ps", "-axo", "command="])?
                    .iter()
                    .any(|l| l.starts_with(&inside)),
            )
        }
        "windows" => {
            let image = std::path::Path::new(&app.path)
                .file_name()?
                .to_string_lossy()
                .to_lowercase();
            let filter = format!("IMAGENAME eq {image}");
            Some(
                process_lines(&["tasklist", "/FI", &filter, "/NH"])?
                    .iter()
                    .any(|l| l.to_lowercase().starts_with(&image)),
            )
        }
        "linux" => {
            let name = std::path::Path::new(&app.path)
                .file_name()?
                .to_string_lossy()
                .to_string();
            Some(
                process_lines(&["ps", "-eo", "comm="])?
                    .iter()
                    .any(|l| l.trim() == name),
            )
        }
        _ => None,
    }
}

fn process_lines(command: &[&str]) -> Option<Vec<String>> {
    let out = Command::new(command[0]).args(&command[1..]).output().ok()?;
    out.status.success().then(|| {
        String::from_utf8_lossy(&out.stdout)
            .lines()
            .map(String::from)
            .collect()
    })
}

/// The error text inside a failed tool reply (`{ success: false, result: { error } }` and similar).
fn failure_message(v: &Value) -> String {
    [
        &v["result"]["error"],
        &v["error"],
        &v["message"],
        &v["result"]["message"],
    ]
    .into_iter()
    .find_map(|x| {
        x.as_str()
            .map(String::from)
            .or_else(|| x["message"].as_str().map(String::from))
    })
    .unwrap_or_else(|| "see the server log".into())
}

/// Turn a failed app check into what to do about it.
fn explain_failure(raw: &str, app: &str) -> String {
    let message = serde_json::from_str::<Value>(raw)
        .map(|v| failure_message(&v))
        .unwrap_or_else(|_| raw.lines().next().unwrap_or(raw).to_string());
    let lower = message.to_lowercase();
    if lower.contains("-1743")
        || lower.contains("not authorized to send apple events")
        || lower.contains("not allowed to send")
    {
        format!(
            "macOS is blocking automation. Allow your MCP client (and Terminal) to control {app} in System Settings → Privacy & Security → Automation: \
             open 'x-apple.systempreferences:com.apple.preference.security?Privacy_Automation'"
        )
    } else if lower.contains("did not answer")
        || lower.contains("timed out")
        || lower.contains("timeout")
    {
        format!("did not answer; if {app} is showing a dialog, close it and run doctor again")
    } else {
        message
    }
}

/// One line from a status tool's reply: version and open documents when it reports them.
fn summarize_status(text: &str) -> String {
    let Ok(v) = serde_json::from_str::<Value>(text) else {
        return text
            .lines()
            .next()
            .unwrap_or("")
            .chars()
            .take(100)
            .collect();
    };
    if v["success"] == json!(false) {
        return format!("not reachable: {}", failure_message(&v));
    }
    let result = if v["result"].is_object() {
        &v["result"]
    } else {
        &v
    };
    // The app's own block is the object that carries a version.
    let app = result
        .as_object()
        .and_then(|o| o.values().find(|x| x.get("version").is_some()))
        .unwrap_or(result);
    let mut parts = Vec::new();
    if let Some(version) = app["version"].as_str() {
        parts.push(format!("version {version}"));
    }
    match &app["documents"] {
        Value::Number(n) => parts.push(format!(
            "{n} open document{}",
            if n.as_u64() == Some(1) { "" } else { "s" }
        )),
        Value::Array(a) => parts.push(format!(
            "{} open document{}",
            a.len(),
            if a.len() == 1 { "" } else { "s" }
        )),
        _ => {}
    }
    if parts.is_empty() {
        "reachable".into()
    } else {
        format!("reachable, {}", parts.join(", "))
    }
}

fn print_detection(registry: &Registry, apps: &[FoundApp], clients: &[FoundClient]) {
    println!("Creative apps on this machine:");
    let mut any = false;
    for tool in &registry.tools {
        for a in apps_for(apps, &tool.id) {
            any = true;
            println!(
                "  • {:<22} {:<10} → {}",
                a.name,
                a.version.as_deref().unwrap_or(""),
                tool.name
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
        let how = match (&c.command, c.files.first()) {
            (Some(p), _) => format!("command {p}"),
            (None, Some(f)) => format!("config {f}"),
            (None, None) => String::new(),
        };
        println!("  • {:<16} {how}", c.name);
    }
}

fn check_known(registry: &Registry, o: &Options) -> bool {
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
    o.tools.as_ref().is_none_or(|ids| {
        known(
            ids,
            &|id| registry.tool(id).is_some(),
            "tool",
            registry.tools.iter().map(|t| t.id.as_str()).collect(),
        )
    }) && o.clients.as_ref().is_none_or(|ids| {
        known(
            ids,
            &|id| registry.client(id).is_some(),
            "client",
            registry.clients.iter().map(|c| c.id.as_str()).collect(),
        )
    })
}

fn print_plan(steps: &[Step]) {
    println!(
        "\nPlan ({} step{}):",
        steps.len(),
        if steps.len() == 1 { "" } else { "s" }
    );
    for (i, s) in steps.iter().enumerate() {
        println!("  {}. {}", i + 1, exec::describe(s));
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
        println!(
            "    {}",
            if installed(t) {
                format!("creative-mcp update {}", t.id)
            } else {
                format!("creative-mcp add {}", t.id)
            }
        );
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
    let found_clients = detect::detect_clients(registry);
    let configs = configs(registry, &found_clients);
    let have = existing(&configs, &found_clients);
    let mut problems: Vec<String> = Vec::new();
    let mut servers_report = Vec::new();
    let mut registrations_report = Vec::new();
    let mut checks_report = Vec::new();
    let say = |line: String| {
        if !o.json {
            println!("{line}");
        }
    };

    say("Servers:".into());
    for tool in &registry.tools {
        let relevant = !apps_for(&apps, &tool.id).is_empty() || installed(tool);
        if !relevant {
            continue;
        }
        for server in plan::servers_for(tool, &apps) {
            let app = server
                .app
                .as_deref()
                .and_then(|a| apps.iter().find(|f| f.tool == tool.id && f.app == a));
            let mut entry = json!({ "tool": tool.id, "command": server.command });
            if detect::which(&server.command).is_none() {
                let fix = format!("creative-mcp add {}", tool.id);
                say(format!("  ✗ {:<26} not installed → {fix}", server.command));
                problems.push(format!("{} is not installed → {fix}", server.command));
                entry["ok"] = json!(false);
                servers_report.push(entry);
                continue;
            }
            match exec::verify(&server.command, &server.args, VERIFY_TIMEOUT) {
                Ok(info) => {
                    say(format!("  ✓ {:<26} answers ({info})", server.command));
                    entry["ok"] = json!(true);
                    entry["server"] = json!(info);
                }
                Err(e) => {
                    let fix = format!("creative-mcp update {}", tool.id);
                    say(format!("  ✗ {:<26} {e} → {fix}", server.command));
                    problems.push(format!("{} does not answer: {e} → {fix}", server.command));
                    entry["ok"] = json!(false);
                    entry["detail"] = json!(e);
                    servers_report.push(entry);
                    continue;
                }
            }
            // The app connection. Status tools may start their app, so only ask while it is running.
            if let (Some(app), Some(status)) = (app, &server.status) {
                match app_running(app) {
                    Some(true) => {
                        match exec::call_tool(&server.command, &server.args, status, VERIFY_TIMEOUT)
                        {
                            Ok(text) => {
                                let mut summary = summarize_status(&text);
                                let bad = summary.starts_with("not reachable");
                                if bad {
                                    summary = format!(
                                        "not reachable: {}",
                                        explain_failure(&text, &app.name)
                                    );
                                }
                                say(format!(
                                    "      {} {}: {summary}",
                                    if bad { "✗" } else { "✓" },
                                    app.name
                                ));
                                if bad {
                                    problems.push(format!("{} is running but {summary}", app.name));
                                }
                                entry["app"] = json!({ "name": app.name, "running": true, "reachable": !bad, "detail": summary });
                            }
                            Err(e) => {
                                let e = explain_failure(&e, &app.name);
                                say(format!("      ✗ {}: {e}", app.name));
                                problems.push(format!("{} is running but {e}", app.name));
                                entry["app"] = json!({ "name": app.name, "running": true, "reachable": false, "detail": e });
                            }
                        }
                    }
                    Some(false) => {
                        say(format!("      · {}: not running (open it and run doctor again to check the connection)", app.name));
                        entry["app"] = json!({ "name": app.name, "running": false });
                    }
                    None => {}
                }
            }
            servers_report.push(entry);
        }
        for check in &tool.checks {
            let mut parts = check.split_whitespace();
            let Some(program) = parts.next() else {
                continue;
            };
            if detect::which(program).is_none() {
                continue;
            }
            let out = Command::new(program).args(parts).output();
            let (ok, text) = match out {
                Ok(o) => (
                    o.status.success(),
                    String::from_utf8_lossy(&o.stdout).trim().to_string(),
                ),
                Err(e) => (false, e.to_string()),
            };
            say(format!("  {} {check}", if ok { "✓" } else { "✗" }));
            for line in text.lines().filter(|l| !l.trim().is_empty()).take(6) {
                say(format!("      {}", line.trim()));
            }
            if !ok {
                problems.push(format!(
                    "`{check}` failed → creative-mcp update {}",
                    tool.id
                ));
            }
            checks_report
                .push(json!({ "tool": tool.id, "check": check, "ok": ok, "output": text }));
        }
    }
    if servers_report.is_empty() {
        say("  No Creative MCP tools are installed or relevant to the apps on this machine. Run `creative-mcp setup`.".into());
    }

    say("\nRegistrations:".into());
    for client in &found_clients {
        let wanted: Vec<_> = registry
            .tools
            .iter()
            .filter(|t| t.clients.contains(&client.id) && installed(t))
            .flat_map(|t| plan::servers_for(t, &apps).into_iter().map(move |s| (t, s)))
            .collect();
        if wanted.is_empty() {
            continue;
        }
        let Some(Some(registered)) = have.get(&client.id) else {
            say(format!(
                "  · {:<16} config not readable by creative-mcp; check it in the client",
                client.name
            ));
            continue;
        };
        let mut row = Vec::new();
        let mut cells = Vec::new();
        for (tool, server) in wanted {
            let state = clients::state(registered, server);
            let mark = match &state {
                State::Current => "✓",
                State::Missing => "–",
                State::Different(e)
                    if std::path::Path::new(&e.command).is_absolute()
                        && !std::path::Path::new(&e.command).exists() =>
                {
                    let fix = format!(
                        "creative-mcp setup --tools {} --clients {} --no-install",
                        tool.id, client.id
                    );
                    problems.push(format!(
                        "{} in {} points to {}, which no longer exists → {fix}",
                        server.register_as, client.name, e.command
                    ));
                    "✗"
                }
                State::Different(_) => "≠",
            };
            cells.push(format!("{} {}", mark, server.register_as));
            row.push(json!({ "server": server.register_as, "state": match state { State::Current => "current", State::Missing => "missing", State::Different(_) => "different" } }));
        }
        say(format!("  {:<16} {}", client.name, cells.join("  ")));
        registrations_report.push(json!({ "client": client.id, "servers": row }));
    }
    // An installed tool no client knows about can't be used.
    for tool in registry.tools.iter().filter(|t| installed(t)) {
        let anywhere = plan::servers_for(tool, &apps).iter().any(|s| {
            have.values()
                .flatten()
                .any(|r| matches!(clients::state(r, s), State::Current | State::Different(_)))
        });
        let can = found_clients.iter().any(|c| tool.clients.contains(&c.id));
        if !anywhere && can {
            problems.push(format!("{} is installed but not registered with any client → creative-mcp setup --tools {}", tool.name, tool.id));
        }
    }
    if registrations_report.is_empty() {
        say("  (no clients with Creative MCP servers to check)".into());
    } else {
        say(
            "  ✓ registered   – not registered   ≠ registered with another command   ✗ broken"
                .into(),
        );
    }

    if o.json {
        println!(
            "{}",
            json!({ "apps": apps, "clients": found_clients, "servers": servers_report, "registrations": registrations_report,
                    "checks": checks_report, "problems": problems })
        );
    } else if problems.is_empty() {
        println!("\nNo problems found.");
    } else {
        println!(
            "\n{} problem{}:",
            problems.len(),
            if problems.len() == 1 { "" } else { "s" }
        );
        for p in &problems {
            println!("  ✗ {p}");
        }
    }
    if problems.is_empty() {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}

/// Pick tools (setup, add) or installed tools (update, remove), honouring --tools and prompts.
fn choose_tools<'a>(
    registry: &'a Registry,
    apps: &[FoundApp],
    o: &Options,
    interactive: bool,
    verb: &str,
) -> Vec<&'a Tool> {
    let mut tools = Vec::new();
    for tool in &registry.tools {
        let wanted = match (&o.tools, o.command.as_str()) {
            (Some(ids), _) => ids.contains(&tool.id),
            (None, "update" | "remove") => {
                installed(tool)
                    && (o.command == "update"
                        || !interactive
                        || ask(&format!("\n{verb} {}?", tool.name), false))
            }
            (None, _) => {
                let found = apps_for(apps, &tool.id);
                if found.is_empty() {
                    false
                } else {
                    let names = found
                        .iter()
                        .map(|a| a.name.as_str())
                        .collect::<Vec<_>>()
                        .join(", ");
                    !interactive || ask(&format!("\n{verb} {} for {names}?", tool.name), true)
                }
            }
        };
        if wanted {
            tools.push(tool);
        }
    }
    tools
}

fn cmd_setup(registry: &Registry, o: &Options) -> ExitCode {
    if !check_known(registry, o) {
        return ExitCode::from(2);
    }
    let interactive = !o.yes && std::io::stdin().is_terminal();
    let apps = detect::detect_apps(registry);
    let found_clients = detect::detect_clients(registry);
    print_detection(registry, &apps, &found_clients);

    let tools = choose_tools(registry, &apps, o, interactive, "Set up");
    if tools.is_empty() {
        println!("\nNothing to set up. Name a tool to set it up without a detected app: creative-mcp add blender");
        return ExitCode::SUCCESS;
    }

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
    if found_clients.is_empty() {
        println!("\nNo MCP client found. Install one (Claude Desktop, Claude Code, Codex, Cursor or Gemini CLI), then run setup again to register the servers.");
    }

    let configs = configs(registry, &clients);
    let have = existing(&configs, &clients);
    let env = exec::environment();
    let plan = plan::build(&tools, &apps, &clients, &configs, &have, &env, o.install);
    if !plan.already.is_empty() {
        println!("\nAlready registered:");
        let mut by_client: Vec<(&str, Vec<&str>)> = Vec::new();
        for (client, server) in &plan.already {
            match by_client.iter_mut().find(|(c, _)| c == client) {
                Some((_, servers)) => servers.push(server),
                None => by_client.push((client, vec![server])),
            }
        }
        for (client, servers) in by_client {
            println!("  • {client:<16} {}", servers.join(", "));
        }
    }
    if plan.steps.is_empty() {
        println!("\nEverything is already set up. Check it with: creative-mcp doctor");
        return ExitCode::SUCCESS;
    }
    print_plan(&plan.steps);
    if o.dry_run {
        println!("\nDry run: nothing was changed.");
        return ExitCode::SUCCESS;
    }
    if interactive && !ask("\nProceed?", true) {
        println!("Cancelled. Nothing was changed.");
        return ExitCode::SUCCESS;
    }
    let outcome = exec::execute(&plan.steps);
    let verify_failures = if o.verify {
        verify_servers(&tools, &apps)
    } else {
        0
    };
    report(&outcome);

    // Next steps: which clients to restart, what to try, where to look.
    let changed: Vec<&FoundClient> = clients
        .iter()
        .filter(|c| {
            plan.steps
                .iter()
                .any(|s| step_why(s).ends_with(&format!("with {}", c.name)))
        })
        .collect();
    if !changed.is_empty() {
        println!("\nNext:");
        let cli: Vec<&str> = changed
            .iter()
            .filter(|c| {
                registry
                    .client(&c.id)
                    .and_then(|r| r.restart.as_ref())
                    .is_none()
            })
            .map(|c| c.name.as_str())
            .collect();
        for c in &changed {
            if let Some(restart) = registry.client(&c.id).and_then(|r| r.restart.as_deref()) {
                println!("  • {restart} to load the new servers.");
            }
        }
        if !cli.is_empty() {
            println!(
                "  • {}: start a new session to load the new servers.",
                cli.join(", ")
            );
        }
    }
    let examples: Vec<(&Tool, &String)> = tools
        .iter()
        .flat_map(|t| t.examples.iter().take(2).map(move |e| (*t, e)))
        .collect();
    if !examples.is_empty() {
        println!("\nThings to try:");
        for (tool, example) in examples {
            println!("  {}: \"{example}\"", tool.name);
        }
    }
    println!("\nCheck the setup any time: creative-mcp doctor");
    for tool in &tools {
        println!("{} docs: {}", tool.name, tool.docs);
    }
    if outcome.failed.is_empty() && verify_failures == 0 {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}

fn step_why(step: &Step) -> &str {
    match step {
        Step::Run { why, .. }
        | Step::WriteJson { why, .. }
        | Step::RemoveJson { why, .. }
        | Step::Manual { why, .. } => why,
    }
}

fn verify_servers(tools: &[&Tool], apps: &[FoundApp]) -> usize {
    println!("\nChecking each server answers MCP initialize:");
    let mut failures = 0;
    for tool in tools {
        for server in plan::servers_for(tool, apps) {
            match exec::verify(&server.command, &server.args, VERIFY_TIMEOUT) {
                Ok(info) => println!("  ✓ {:<26} {info}", server.command),
                Err(e) => {
                    failures += 1;
                    println!("  ✗ {:<26} {e}", server.command);
                }
            }
        }
    }
    failures
}

fn report(outcome: &exec::Outcome) {
    println!(
        "\nDone: {} step{} completed, {} failed.",
        outcome.ok,
        if outcome.ok == 1 { "" } else { "s" },
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
}

fn cmd_update(registry: &Registry, o: &Options) -> ExitCode {
    if !check_known(registry, o) {
        return ExitCode::from(2);
    }
    let interactive = !o.yes && std::io::stdin().is_terminal();
    let apps = detect::detect_apps(registry);
    let tools = choose_tools(registry, &apps, o, interactive, "Update");
    if tools.is_empty() {
        println!("No Creative MCP tools are installed. Set one up with: creative-mcp setup");
        return ExitCode::SUCCESS;
    }
    let env = exec::environment();
    let mut steps = Vec::new();
    if env.brew {
        steps.push(Step::Run {
            why: "Fetch the latest formulae".into(),
            program: "brew".into(),
            args: vec!["update".into()],
            allow_failure: true,
            server_arg: None,
        });
        let mut formulae: Vec<String> = tools
            .iter()
            .filter_map(|t| t.install.brew.as_ref().map(|b| b.formula.clone()))
            .filter(|f| {
                env.installed_formulae
                    .iter()
                    .any(|i| i == plan::formula_name(f))
            })
            .collect();
        if env.installed_formulae.iter().any(|i| i == "creative-mcp") {
            formulae.push(format!("{}/creative-mcp", plan::TAP));
        }
        if !formulae.is_empty() {
            let mut args = vec!["upgrade".to_string()];
            args.extend(formulae);
            steps.push(Step::Run {
                why: "Upgrade the Creative MCP tools installed with Homebrew".into(),
                program: "brew".into(),
                args,
                allow_failure: false,
                server_arg: None,
            });
        }
    }
    for tool in &tools {
        if tool.install.brew.as_ref().is_some_and(|b| {
            env.installed_formulae
                .iter()
                .any(|i| i == plan::formula_name(&b.formula))
        }) {
            for post in &tool.post_install {
                let mut parts = post.split_whitespace().map(String::from);
                if let Some(program) = parts.next() {
                    steps.push(Step::Run {
                        why: format!("{}: {post}", tool.name),
                        program,
                        args: parts.collect(),
                        allow_failure: false,
                        server_arg: None,
                    });
                }
            }
        } else {
            steps.push(Step::Manual {
                why: format!("{} was not installed with Homebrew", tool.name),
                text: format!(
                    "update it the way you installed it, or reinstall with: brew install {}",
                    tool.install
                        .brew
                        .as_ref()
                        .map(|b| b.formula.as_str())
                        .unwrap_or(&tool.id)
                ),
            });
        }
    }
    // Refresh registrations that point at an old path or command; leave missing ones to setup.
    let found_clients = detect::detect_clients(registry);
    let configs = configs(registry, &found_clients);
    let have = existing(&configs, &found_clients);
    for tool in &tools {
        for client in found_clients
            .iter()
            .filter(|c| tool.clients.contains(&c.id))
        {
            let registered = have.get(&client.id).cloned().flatten().unwrap_or_default();
            for server in plan::servers_for(tool, &apps) {
                if let State::Different(_) = clients::state(&registered, server) {
                    steps.extend(plan::register_steps(
                        client,
                        configs.get(&client.id).map(String::as_str),
                        server,
                        true,
                    ));
                }
            }
        }
    }
    print_plan(&steps);
    if o.dry_run {
        println!("\nDry run: nothing was changed.");
        return ExitCode::SUCCESS;
    }
    if interactive && !ask("\nProceed?", true) {
        println!("Cancelled. Nothing was changed.");
        return ExitCode::SUCCESS;
    }
    let outcome = exec::execute(&steps);
    let failures = if o.verify {
        verify_servers(&tools, &apps)
    } else {
        0
    };
    report(&outcome);
    println!("\nRestart your MCP clients (or start a new session) to use the updated servers.");
    if outcome.failed.is_empty() && failures == 0 {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}

fn cmd_remove(registry: &Registry, o: &Options) -> ExitCode {
    if !check_known(registry, o) {
        return ExitCode::from(2);
    }
    let interactive = !o.yes && std::io::stdin().is_terminal();
    let found_clients = detect::detect_clients(registry);
    let configs = configs(registry, &found_clients);
    let have = existing(&configs, &found_clients);
    let registered_anywhere = |tool: &Tool| {
        tool.servers.iter().any(|s| {
            have.values()
                .flatten()
                .any(|r| r.contains_key(&s.register_as))
        })
    };
    if o.tools.is_none() && !interactive {
        eprintln!("Name the tools to remove, e.g. creative-mcp remove blender --yes");
        return ExitCode::from(2);
    }
    let tools: Vec<&Tool> = registry
        .tools
        .iter()
        .filter(|t| match &o.tools {
            Some(ids) => ids.contains(&t.id),
            None => {
                (installed(t) || registered_anywhere(t))
                    && ask(&format!("Remove {}?", t.name), false)
            }
        })
        .collect();
    if tools.is_empty() {
        println!("Nothing to remove.");
        return ExitCode::SUCCESS;
    }
    let mut steps = Vec::new();
    for tool in &tools {
        for client in &found_clients {
            if o.clients
                .as_ref()
                .is_some_and(|ids| !ids.contains(&client.id))
            {
                continue;
            }
            let Some(Some(registered)) = have.get(&client.id) else {
                continue;
            };
            for server in &tool.servers {
                // Only entries this tool would have written: same name and the same program.
                if matches!(clients::state(registered, server), State::Current)
                    || registered
                        .get(&server.register_as)
                        .is_some_and(|e| e.command.ends_with(&server.command))
                {
                    steps.push(plan::unregister_step(
                        client,
                        configs.get(&client.id).map(String::as_str),
                        &server.register_as,
                    ));
                }
            }
        }
        if o.install && o.clients.is_none() {
            let env = exec::environment();
            if let Some(b) = tool.install.brew.as_ref().filter(|b| {
                env.installed_formulae
                    .iter()
                    .any(|i| i == plan::formula_name(&b.formula))
            }) {
                steps.push(Step::Run {
                    why: format!("Uninstall {}", tool.name),
                    program: "brew".into(),
                    args: vec!["uninstall".into(), b.formula.clone()],
                    allow_failure: false,
                    server_arg: None,
                });
            }
        }
    }
    print_plan(&steps);
    if steps.is_empty() {
        println!("\nNothing to remove: no client has these servers registered and they are not installed with Homebrew.");
        return ExitCode::SUCCESS;
    }
    if o.dry_run {
        println!("\nDry run: nothing was changed.");
        return ExitCode::SUCCESS;
    }
    if interactive && !ask("\nProceed?", false) {
        println!("Cancelled. Nothing was changed.");
        return ExitCode::SUCCESS;
    }
    let outcome = exec::execute(&steps);
    report(&outcome);
    println!(
        "\nRestart your MCP clients (or start a new session) so they drop the removed servers."
    );
    if outcome.failed.is_empty() {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}

fn main() -> ExitCode {
    let o = match parse_args(std::env::args().skip(1)) {
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
        "add" => {
            if o.tools.is_none() {
                eprintln!("Name the tools to add, e.g. creative-mcp add blender");
                return ExitCode::from(2);
            }
            cmd_setup(&registry, &o)
        }
        "update" | "upgrade" => cmd_update(&registry, &o),
        "remove" | "uninstall" => cmd_remove(&registry, &o),
        "detect" => cmd_detect(&registry, &o),
        "list" => cmd_list(&registry, &o),
        "doctor" => cmd_doctor(&registry, &o),
        other => {
            eprintln!("unknown command '{other}'\n\n{HELP}");
            ExitCode::from(2)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn args(s: &str) -> Vec<String> {
        s.split_whitespace().map(String::from).collect()
    }

    #[test]
    fn tool_names_after_a_command() {
        let o = parse_args(args("add blender --yes")).unwrap();
        assert_eq!(o.command, "add");
        assert_eq!(o.tools, Some(vec!["blender".to_string()]));
        let o = parse_args(args("remove adobe,blender")).unwrap();
        assert_eq!(
            o.tools,
            Some(vec!["adobe".to_string(), "blender".to_string()])
        );
        assert!(parse_args(args("doctor blender")).is_err());
    }

    #[test]
    fn status_summaries() {
        let ps = r#"{"success":true,"result":{"running":true,"photoshop":{"version":"27.11.0","documents":1}}}"#;
        assert_eq!(
            summarize_status(ps),
            "reachable, version 27.11.0, 1 open document"
        );
        let down =
            r#"{"success":false,"result":{"running":true,"error":"Photoshop did not respond"}}"#;
        assert_eq!(
            summarize_status(down),
            "not reachable: Photoshop did not respond"
        );
        let denied = r#"{"success":false,"result":{"error":"execution error: Not authorized to send Apple events to Adobe Photoshop 2026. (-1743)"}}"#;
        assert!(
            explain_failure(denied, "Adobe Photoshop").contains("Privacy & Security → Automation")
        );
        assert!(explain_failure(
            "photoshop_status did not answer within 20s",
            "Adobe Photoshop"
        )
        .contains("dialog"));
        assert_eq!(
            summarize_status("Addon connected on port 9876\nmore"),
            "Addon connected on port 9876"
        );
    }
}
