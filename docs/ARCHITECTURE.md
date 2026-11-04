# Creative MCP series architecture

Status (2026-11-03): the catalogue (`registry/`) and the installer (`cli/`, v0.2.1) are **Implemented and tested**. The installer fully supports macOS only so far. Author: Zach Shallbetter.

Creative MCP is a series of independent MCP implementations for creative software, presented under one brand, one website and one install path. This document records how the pieces fit today, the decisions for growing the series, and what a new tool needs to join.

## 1. What exists today

| Piece | Where | Visibility | Language | Distribution |
| --- | --- | --- | --- | --- |
| Adobe MCP: five app servers, Suite coordinator, shared core | github.com/zachshallbetter/adobe-mcp | Public, MIT | Node.js 18+ | npm (`adobe-mcp-*`), Homebrew `zachshallbetter/tap/adobe-mcp` |
| Blender MCP: Rust MCP server, CLI, Blender add-on | github.com/zachshallbetter/blender-mcp-rs | Public, MIT OR Apache-2.0 | Rust | Source (`cargo install --git`), Homebrew `zachshallbetter/tap/blender-mcp` (v0.5.0) |
| Homebrew tap | github.com/zachshallbetter/homebrew-tap | Public | Ruby formulae | `brew tap zachshallbetter/tap` then `brew trust zachshallbetter/tap` |
| Website, docs and catalogue (creativemcp.com) | github.com/zachshallbetter/creative-mcp (`site/`, `registry/`, `docs/`) | Public; MIT for code, content and brand reserved | Static HTML, Python scripts, JSON | Vercel (Root Directory `site`) |

The implementations share no runtime, API or process. A client can load several at once; moving files between them is a user-managed handoff.

The old in-repo formula `adobe-mcp/Formula/adobe-mcp-suite.rb` is being deprecated in favour of the tap (adobe-mcp#430).

## 2. Principles

1. **One repository per tool.** Each implementation keeps its own language, release cycle, issues and licence. The series never forces a monorepo or a shared runtime.
2. **One catalogue as the source of truth.** Every place that lists tools (site pages, `llms.txt`, the setup builder, the installer, the tap README) reads the same catalogue entry. Adding a tool means adding one entry, not editing a dozen pages.
3. **One install entry point.** People install the `creative-mcp` installer, which detects their apps and clients and asks what to set up. Per-tool installs (npm, Cargo, Homebrew) keep working for people who want them.
4. **Claim discipline.** Every capability carries one of Documented, Implemented, Tested or Validated. Planned work is labelled Planned and never presented as shipped (DPP 8.2).
5. **Local and inspectable.** Detection reads only app bundles, `PATH` and client config files on the user's machine. It sends nothing anywhere. Every change it makes to a client config is shown first and can be undone.

## 3. The catalogue (`registry.json`), draft

A versioned JSON file, one entry per tool. The source is [`registry/registry.json`](../registry/registry.json) with its [schema](../registry/registry.schema.json). `site/scripts/generate-discovery.py` publishes both at `https://creativemcp.com/registry.json`, and the `Validate registry` workflow checks every change. The `creative-mcp` installer builds it into its binary. A shortened example:

```json
{
  "schema": 1,
  "series": "creative-mcp",
  "updated": "2026-11-03",
  "tools": [
    {
      "id": "adobe",
      "name": "Adobe MCP",
      "summary": "Local MCP servers for InDesign, Illustrator, Photoshop, After Effects and Premiere Pro.",
      "repo": "https://github.com/zachshallbetter/adobe-mcp",
      "docs": "https://creativemcp.com/apps.html",
      "license": "MIT",
      "author": "Zach Shallbetter",
      "status": "Tested",
      "install": {
        "brew": { "formula": "zachshallbetter/tap/adobe-mcp" },
        "npm": { "package": "adobe-mcp-suite" }
      },
      "requires": ["node>=18"],
      "servers": [
        { "command": "adobe-mcp-suite", "role": "coordinator", "register_as": "adobe" },
        { "command": "adobe-mcp-photoshop", "app": "photoshop", "register_as": "photoshop" }
      ],
      "targets": [
        { "app": "photoshop", "name": "Adobe Photoshop",
          "detect": { "macos": { "bundle_ids": ["com.adobe.Photoshop"] },
                      "windows": { "globs": ["C:/Program Files/Adobe/Adobe Photoshop */Photoshop.exe"] } } },
        { "app": "illustrator", "name": "Adobe Illustrator",
          "detect": { "macos": { "bundle_ids": ["com.adobe.illustrator"] } } },
        { "app": "aftereffects", "name": "Adobe After Effects",
          "detect": { "macos": { "bundle_ids": ["com.adobe.AfterEffects.application"] } } }
      ],
      "clients": ["claude-code", "claude-desktop", "codex", "cursor", "gemini"]
    },
    {
      "id": "blender",
      "name": "Blender MCP",
      "repo": "https://github.com/zachshallbetter/blender-mcp-rs",
      "docs": "https://creativemcp.com/docs/implementations/blender.html",
      "license": "MIT OR Apache-2.0",
      "status": "Implemented",
      "install": { "brew": { "formula": "zachshallbetter/tap/blender-mcp" },
                   "cargo": { "git": "https://github.com/zachshallbetter/blender-mcp-rs", "package": "blender-mcp-cli" } },
      "servers": [{ "command": "blender-mcp", "args": ["serve"], "register_as": "blender" }],
      "post_install": ["blender-mcp addon install"],
      "targets": [{ "app": "blender", "name": "Blender",
                    "detect": { "macos": { "bundle_ids": ["org.blenderfoundation.blender"], "min_version": "4.0" } } }],
      "clients": ["claude-code", "codex", "cursor", "gemini", "antigravity"]
    }
  ],
  "clients": [
    { "id": "claude-code", "name": "Claude Code", "detect": { "commands": ["claude"], "files": ["~/.claude.json"] } },
    { "id": "claude-desktop", "name": "Claude Desktop", "detect": { "files": ["~/Library/Application Support/Claude/claude_desktop_config.json"] } },
    { "id": "codex", "name": "Codex", "detect": { "commands": ["codex"], "files": ["~/.codex/config.toml"] } },
    { "id": "cursor", "name": "Cursor", "detect": { "files": ["~/.cursor/mcp.json"] } },
    { "id": "gemini", "name": "Gemini CLI", "detect": { "commands": ["gemini"] } }
  ]
}
```

The bundle IDs above were checked on a real Mac on 2026-11-03 (Photoshop, Illustrator, After Effects 2026, Blender). InDesign and Premiere Pro IDs still need confirming on a machine that has them. Premiere's ID may include a version suffix, so the detector should match prefixes.

`registry/validate.py` checks the schema plus cross-references (unknown clients or apps, duplicate IDs). CI runs it on every pull request that touches `registry/`. Detection rules carry `"verified": true` only after being checked on a real machine.

## 4. The `creative-mcp` installer (v0.2.1)

Implemented in [`cli/`](../cli) and shipped as `brew install zachshallbetter/tap/creative-mcp`. Shipped: `setup`, `add`, `update`, `remove`, `doctor`, `detect` and `list`, with `--yes`, `--dry-run`, `--tools`, `--clients`, `--no-install`, `--no-verify` and `--json`. Windows detects apps in their default install folders but is unverified on real hardware ([#5](https://github.com/zachshallbetter/creative-mcp/issues/5)). Not built yet: installing Premiere Pro's UXP plugin without the UXP Developer Tool ([#17](https://github.com/zachshallbetter/creative-mcp/issues/17)).

```text
brew install zachshallbetter/tap/creative-mcp     # later also: npm, a Windows download
creative-mcp setup
```

| Command | Does |
| --- | --- |
| `setup` | Detects installed creative apps and MCP clients, suggests the matching tools, reads what each client already has, shows the plan, installs, registers only what is missing or stale, confirms each server answers MCP `initialize`, then says which clients to restart and what to try. Safe to run again. |
| `add <tool>` | `setup` for named tools, with or without a detected app. |
| `update [tool]` | `brew update`, upgrades the installed tools and creative-mcp itself, reruns post-install steps (such as the Blender add-on) and fixes registrations that point at an old command. |
| `remove <tool>` | Removes the tool's servers from every client, then uninstalls it (`--no-install` keeps it installed). |
| `doctor` | Checks each server answers; asks each running app's status tool whether the app is reachable (never starting an app); runs catalogue checks such as `blender-mcp addon status`; shows which clients have which servers. Every problem ends with the command that fixes it. |
| `detect` / `list` | What was found on this machine; every tool in the catalogue and whether it is installed. |

Flags: `--yes` (accept the detected defaults), `--tools adobe,blender`, `--clients claude-code,codex`, `--dry-run` (print every command and config change without doing it), `--json`.

How it works:

- **Detection.** macOS: Spotlight by bundle ID (`mdfind`, with a time limit), falling back to reading the `Info.plist` of each app in `/Applications` and `~/Applications` when Spotlight is off or stalled. Windows: the catalogue's install-path globs (`C:/Program Files/Adobe/Adobe Photoshop */Photoshop.exe`). Linux: `PATH`. Clients: commands on `PATH` and their config files. A client counts as present if either exists. The catalogue can list extra places a client's CLI lives (`detect.paths`), which is how the Claude Code CLI bundled with Claude Desktop is found when `claude` isn't on `PATH`.
- **Install method.** Homebrew when `brew` is present (it also supplies Node or Rust). Otherwise the tool's native method (npm or Cargo), after checking the prerequisites in `requires`.
- **Client registration.** Read the client's user-level config first (JSON `mcpServers`, or Codex's TOML `mcp_servers`) and skip servers already registered with the same command. Use the client's own command where one exists (`claude mcp add --scope user`, `codex mcp add`, `gemini mcp add --scope user`); otherwise edit its JSON config, backing the file up before the first change and keeping every other setting. `remove` only removes entries with the tool's own server name and command.
- **Privacy.** No telemetry. The only network access is downloading the catalogue and the packages.

Language: **Rust**, so the installer is a single dependency-free binary that works for someone who only wants Blender, and shares tooling with Blender MCP.

## 5. Naming conventions for new tools

| Thing | Rule | Example |
| --- | --- | --- |
| Catalogue ID | lowercase subject, one word | `adobe`, `blender` |
| Repository | `<id>-mcp` (`-rs` etc. only if a second language exists) | `adobe-mcp`; existing `blender-mcp-rs` keeps its name |
| Homebrew formula | `<id>-mcp` in `zachshallbetter/tap` | `blender-mcp` |
| Packages | `<id>-mcp` and `<id>-mcp-<app>` for per-app servers | `adobe-mcp-photoshop` |
| Server command | same as the package name | `adobe-mcp-suite` |
| Client registration name | `<id>`, or `<app>` for per-app servers | `claude mcp add photoshop -- adobe-mcp-photoshop` |
| Docs page | `/docs/implementations/<id>.html` (or per app) | `/docs/implementations/blender.html` |
| Status vocabulary | Documented, Implemented, Tested, Validated | |

## 6. Adding a tool to the series

1. Create the repo with a README, LICENSE files, a CONTRIBUTING guide and tagged releases.
2. Publish a package (npm, crates or release binaries) and add a formula to `zachshallbetter/tap` with a real `test do` block.
3. Add the catalogue entry, including detection rules checked on a real machine.
4. Add the docs page and its card in `site/scripts/generate-social-images.py`, then run that script and `site/scripts/generate-discovery.py`.
5. Record capability status honestly. Only Validated claims say the workflow was run in the real app.

## 7. Tracked work

- Installer: Windows app detection ([#5](https://github.com/zachshallbetter/creative-mcp/issues/5)); Premiere Pro's UXP plugin without the UXP Developer Tool ([#17](https://github.com/zachshallbetter/creative-mcp/issues/17)).
- Adobe MCP: status tools should not launch their app ([adobe-mcp#431](https://github.com/zachshallbetter/adobe-mcp/issues/431)).
- Catalogue: verify the InDesign and Premiere Pro detection rules ([#4](https://github.com/zachshallbetter/creative-mcp/issues/4)).
- Site: generate listings from the catalogue, decision D4 ([#8](https://github.com/zachshallbetter/creative-mcp/issues/8)).

## 8. Open decisions

- **D1: where the catalogue and installer live. Decided (2026-11-03):** the public `zachshallbetter/creative-mcp` repo holds the site (`site/`), the catalogue (`registry/`), the future installer (`cli/`) and these documents (`docs/`). The earlier private site repo, whose history contained personal data, is archived privately as `creative-mcp-site-archive`.
- **D2: installer language. Decided:** Rust, shipped as creative-mcp 0.1.0; 0.2.0 added `add`, `update`, `remove` and the fuller `doctor`.
- **D3: Blender MCP release. Done (2026-11-03):** licence files added, `VERSION` reconciled, the Windows CI failure fixed, and `v0.5.0` tagged and released. Homebrew installs it without `--HEAD`.
- **D4: site generation.** Once the catalogue exists, generate the Projects grid, setup builder options and `llms.txt` key facts from it rather than hand-editing HTML.
