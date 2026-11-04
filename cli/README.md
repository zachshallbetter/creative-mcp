# creative-mcp

Set up the [Creative MCP](https://creativemcp.com) series on your machine in one command.
Created by [Zach Shallbetter](https://zachshallbetter.com/).

```sh
brew tap zachshallbetter/tap
brew trust zachshallbetter/tap        # Homebrew 7 and later
brew install zachshallbetter/tap/creative-mcp
creative-mcp setup
```

`creative-mcp setup`:

1. **Detects** the creative apps on this machine (macOS: Spotlight by bundle ID, falling back to reading the app bundles in `/Applications` when Spotlight is off or slow) and the MCP clients you use (their command, including the copy of Claude Code bundled with Claude Desktop, or their config file).
2. **Asks** which tools to set up. Tools that match an installed app are suggested; nothing is chosen for you without a prompt unless you pass `--yes`.
3. **Reads what is already registered** in each client and leaves it alone. An entry with the same name but another command (an old `npx` line, a path that no longer exists) is replaced, and the plan says so.
4. **Shows the full plan** (every command it will run and every config entry it will write) and asks before changing anything.
5. **Installs** each tool with Homebrew, or with npm or Cargo when Homebrew isn't available, then runs post-install steps such as `blender-mcp addon install`. The Homebrew tap is added and trusted only if it isn't already.
6. **Registers** the servers with your clients:
   - Claude Code, Codex and Gemini CLI through their own `mcp add` commands
   - Claude Desktop and Cursor (and Claude Code when its CLI can't be found) by adding `mcpServers` entries to their JSON config. The file is backed up once, before the first change, and every other setting is kept.
   - Anything else gets the exact snippet to add yourself.
7. **Checks** that every server answers an MCP `initialize` request, then tells you which clients to restart and gives a few requests to try.

Run it again at any time: when everything is in place it says so and changes nothing.

Only the servers that fit your apps are registered. The Adobe Suite coordinator, for example, is offered only when both Illustrator and InDesign are present.

## Commands

| Command | What it does |
| --- | --- |
| `creative-mcp setup` | Detect, choose, install, register and verify |
| `creative-mcp add <tools>` | Set up named tools even without a detected app, e.g. `creative-mcp add blender` |
| `creative-mcp update [tools]` | `brew update`, upgrade the installed tools (and creative-mcp itself), rerun post-install steps and fix registrations that point at an old command |
| `creative-mcp remove <tools>` | Remove the tools' servers from every client, then uninstall them (`--no-install` keeps them installed; `--clients` limits it to some clients) |
| `creative-mcp doctor` | Check each server answers, each running app is reachable, and which clients have which servers registered. Every problem ends with the command that fixes it |
| `creative-mcp detect` | List the creative apps and MCP clients found |
| `creative-mcp list` | Show every tool in the catalogue and whether it is installed |

Options: `--yes`, `--dry-run`, `--tools adobe,blender`, `--clients claude-code,codex,cursor`, `--no-install`, `--no-verify`, `--json` (detect, list, doctor) and `--registry <file>`.

`doctor` never starts an app. It asks a server's status tool about its app only while the app is already running; otherwise it reports the app as not running.

## Privacy

Detection reads app bundles, `PATH` and client config files on your machine. Nothing is sent anywhere, and there is no telemetry. The only network access is Homebrew, npm or Cargo downloading the tools you chose.

## Status

**Implemented and tested** (version 0.2.1):

- **Platforms:** macOS is fully supported. Windows detects the Adobe apps and Blender in their default install folders under `C:\Program Files` and finds Claude Desktop's config under `%APPDATA%`; this hasn't been checked on a real Windows machine yet, and `doctor` can't tell whether an app is running there. Linux detects Blender when `blender` is on `PATH`. Anywhere, `creative-mcp add <tool>` sets up a tool without detection.
- **Tests:** unit tests cover catalogue parsing, server selection, install planning, reading each client's existing registrations (JSON and Codex TOML), client registration and removal, the config backup, Spotlight timeouts and the bundle-scan fallback.
- **Manual testing (2026-11-03, macOS):** detection found Illustrator, Photoshop, After Effects and Blender. `setup`, a second `setup` (no changes), `doctor` and `remove` were run end to end against a throwaway home directory with Claude Code (through the CLI bundled with Claude Desktop, and through `~/.claude.json`), Claude Desktop, Codex, Cursor and Gemini CLI. `doctor` reached a running Photoshop through `photoshop_status`.

The tool list comes from the series catalogue, [`registry/registry.json`](../registry/registry.json), which is built into the binary. It is MIT licensed.
