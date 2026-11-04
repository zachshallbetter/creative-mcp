#!/usr/bin/env python3
"""Render every site listing of the series from the catalogue (registry/registry.json), decision D4.

Run after editing the catalogue, before apply-shell.py: python3 scripts/generate-catalogue.py
It writes:
- dist/registry.json and dist/registry.schema.json (the published catalogue)
- dist/data/setup.json: every setup builder combination (tool × client × install method), precomputed
  here so start.js only looks them up
- marked regions in the pages, leaving everything around them hand-written:
  <!-- registry:projects --> (projects.html), <!-- registry:series --> (docs/architecture.html),
  <!-- registry:builder-options --> (start.html)
generate-discovery.py reads the same catalogue for the key facts in llms.txt.
"""
import html
import json
import re
import shutil
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
DIST = ROOT / "dist"
REGISTRY = ROOT.parent / "registry" / "registry.json"
TAP = "zachshallbetter/tap"
HOMEBREW_BIN = "/opt/homebrew/bin"
METHODS = {"brew": ("Homebrew (macOS, Linux)", "Homebrew"), "npm": ("npm / npx (macOS, Windows)", "npx"),
           "cargo": ("Cargo, from source", "Cargo")}
OTHER = ("other", "Another client with MCP server configuration")


def load():
    return json.loads(REGISTRY.read_text())


def esc(text):
    return html.escape(text, quote=True)


def code(text):
    """Escape, then turn `backticks` into <code>."""
    return re.sub(r"`([^`]+)`", r"<code>\1</code>", esc(text))


def setup_entry(tool, client, method, clients):
    """Kicker, commands and explanation for one builder choice."""
    server = tool["servers"][0]
    lines = []
    if method == "brew":
        lines += [f"brew tap {TAP}", f"brew trust {TAP}", f"brew install {tool['install']['brew']['formula']}"]
    elif method == "cargo":
        cargo = tool["install"]["cargo"]
        lines.append("cargo install --locked" + (f" --git {cargo['git']}" if cargo.get("git") else "") + f" --package {cargo['package']}")
    if method != "npm":
        lines += tool.get("post_install", [])
    # npm installs nothing up front: clients start the server with npx.
    command, args = (("npx", ["-y", server["command"], *server.get("args", [])]) if method == "npm"
                     else (server["command"], server.get("args", [])))
    name = server["register_as"]
    tail = " ".join([command, *args])
    client_name = clients.get(client, {}).get("name", OTHER[1])
    json_note = ""
    if tool.get("register_command") and client in tool["clients"] and method != "npm":
        lines.append(tool["register_command"].format(client=client))
    elif client == "claude-code":
        lines.append(f"claude mcp add --scope user {name} -- {tail}")
    elif client == "codex":
        lines.append(f"codex mcp add {name} -- {tail}")
    elif client == "gemini":
        lines.append(f"gemini mcp add --scope user {name} {tail}")
    else:
        # Desktop apps don't read the shell's PATH, so name the program by its full path.
        full = f"{HOMEBREW_BIN}/{command}" if method == "brew" else command
        snippet = json.dumps({"mcpServers": {name: {"command": full, "args": args}}}, indent=2)
        lines += ["", snippet]
        files = clients.get(client, {}).get("detect", {}).get("files", [])
        where = f"{client_name}'s config file ({files[0]})" if files else "your client's MCP configuration"
        json_note = f"Add this entry to {where}, keeping any servers already there. "
        json_note += ("The path is Homebrew's on Apple silicon; on an Intel Mac it is /usr/local/bin. " if method == "brew"
                      else "If the client can't start npx, replace it with the full path that \"which npx\" prints. ")
    requires = tool.get("requires", [])
    readable = [re.sub(r"^node>=(\d+)$", r"Node.js \1 or later", r) for r in requires]
    brew_note = f"Homebrew installs {', '.join(readable)} for you. " if method == "brew" and requires else ""
    return {
        "kicker": f"{tool['name']} · {client_name} · {METHODS[method][1]}",
        "command": "\n".join(lines),
        "explanation": brew_note + json_note + tool["site"]["setup_note"],
    }


def setup_data(registry):
    clients = {c["id"]: c for c in registry["clients"]}
    tools = []
    for tool in registry["tools"]:
        methods = [m for m in METHODS if m in tool["install"]]
        client_ids = [c for c in tool["clients"] if c in clients] + [OTHER[0]]
        tools.append({
            "id": tool["id"], "name": tool["name"],
            "methods": [{"id": m, "label": METHODS[m][0]} for m in methods],
            "clients": [{"id": c, "label": clients[c]["name"] if c in clients else OTHER[1]} for c in client_ids],
            "readiness": tool["site"]["readiness"],
            "entries": {f"{c}|{m}": setup_entry(tool, c, m, clients) for c in client_ids for m in methods},
        })
    return {"catalogue": registry["updated"], "tools": tools}


def projects_region(registry):
    cards = []
    for tool in registry["tools"]:
        site = tool["site"]
        bullets = [*site["bullets"]]
        installs = []
        if "brew" in tool["install"]:
            installs.append(f"<code>brew install {esc(tool['install']['brew']['formula'])}</code>")
        if "npm" in tool["install"]:
            installs.append("npx")
        if "cargo" in tool["install"]:
            installs.append("Cargo from source")
        event = f' data-event-id="{esc(site["event"])}"' if site.get("event") else ""
        cards.append(
            f'<article class="project-detail-card" style="--tone:var(--{site["tone"]})">'
            f'<span class="project-mark"><img src="{esc(site["mark"])}" alt=""></span><h3>{esc(tool["name"])}</h3>'
            f'<p>{esc(tool["summary"])}</p><ul>' + "".join(f"<li>{code(b)}</li>" for b in bullets)
            + f'<li>Install with {" or ".join(installs)}</li></ul>'
            f'<a{event} class="button light" href="{esc(site["page"])}"><span class="button-label">Explore {esc(tool["name"])} →</span></a></article>')
    return '\n    <div class="project-detail-grid">\n      ' + "\n      ".join(cards) + "\n    </div>\n    "


def series_region(registry):
    rows = []
    for tool in registry["tools"]:
        site = tool["site"]
        installs = []
        if "brew" in tool["install"]:
            installs.append(f"Homebrew <code>{esc(tool['install']['brew']['formula'])}</code>" + (f" (v{esc(tool['version'])})" if tool.get("version") else ""))
        if "npm" in tool["install"]:
            installs.append(f"npm <code>{esc(tool['install']['npm']['package'])}</code>")
        if "cargo" in tool["install"]:
            installs.append("Cargo from source")
        repo = tool["repo"].rstrip("/")
        rows.append(f'<tr><td><a href="{esc(site["page"])}">{esc(tool["name"])}</a>: {esc(site["parts"])}</td>'
                    f'<td><a href="{esc(repo)}">{esc(repo.rsplit("/", 1)[1])}</a></td><td>{esc(tool.get("language", ""))}</td>'
                    f'<td>{", ".join(installs)}</td><td>{esc(tool["license"])}</td></tr>')
    return "".join(rows)


def builder_options(registry):
    return "".join(f'<option value="{esc(t["id"])}">{esc(t["name"])}</option>' for t in registry["tools"])


def replace_region(path, name, content):
    """Fill <!-- registry:name -->…<!-- /registry:name --> in a page."""
    file = DIST / path
    s = file.read_text()
    pattern = re.compile(rf"(<!-- registry:{name} -->).*?(<!-- /registry:{name} -->)", re.S)
    if not pattern.search(s):
        raise SystemExit(f"{path}: missing <!-- registry:{name} --> region")
    updated = pattern.sub(lambda m: m.group(1) + content + m.group(2), s, count=1)
    if updated != s:
        file.write_text(updated)
        return True
    return False


def main():
    registry = load()
    shutil.copyfile(REGISTRY, DIST / "registry.json")
    shutil.copyfile(REGISTRY.with_name("registry.schema.json"), DIST / "registry.schema.json")
    (DIST / "data").mkdir(exist_ok=True)
    (DIST / "data" / "setup.json").write_text(json.dumps(setup_data(registry), indent=2, ensure_ascii=False) + "\n")
    changed = [p for p, name, content in [
        ("projects.html", "projects", projects_region(registry)),
        ("docs/architecture.html", "series", series_region(registry)),
        ("start.html", "builder-options", builder_options(registry)),
    ] if replace_region(p, name, content)]
    print(f"{len(registry['tools'])} tools; wrote data/setup.json; updated {', '.join(changed) or 'no pages'}")


if __name__ == "__main__":
    main()
