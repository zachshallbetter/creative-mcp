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

1. **Detects** the creative apps on this machine (macOS: Spotlight by bundle ID, with the app version) and the MCP clients you use (their command on `PATH` or their config file).
2. **Asks** which tools to set up. Tools that match an installed app are suggested; nothing is chosen for you without a prompt unless you pass `--yes`.
3. **Shows the full plan** (every command it will run and every config entry it will write) and asks before changing anything.
4. **Installs** each tool with Homebrew, or with npm or Cargo when Homebrew isn't available, then runs post-install steps such as `blender-mcp addon install`.
5. **Registers** the servers with your clients:
   - Claude Code, Codex and Gemini CLI through their own `mcp add` commands
   - Claude Desktop and Cursor by adding `mcpServers` entries to their JSON config. The file is backed up once, before the first change, and every other setting is kept.
   - Anything else gets the exact command or snippet to add yourself.
6. **Checks** that every server answers an MCP `initialize` request.

Only the servers that fit your apps are registered. The Adobe Suite coordinator, for example, is offered only when both Illustrator and InDesign are present.

## Commands

| Command | What it does |
| --- | --- |
| `creative-mcp setup` | Detect, choose, install, register and verify |
| `creative-mcp detect` | List the creative apps and MCP clients found |
| `creative-mcp list` | Show every tool in the catalogue and whether it is installed |
| `creative-mcp doctor` | Check that installed servers answer MCP `initialize` |

Options: `--yes`, `--dry-run`, `--tools adobe,blender`, `--clients claude-code,codex,cursor`, `--no-install`, `--no-verify`, `--json` (detect, list, doctor) and `--registry <file>`.

## Privacy

Detection reads app bundles, `PATH` and client config files on your machine. Nothing is sent anywhere, and there is no telemetry. The only network access is Homebrew, npm or Cargo downloading the tools you chose.

## Status

**Implemented and tested** (version 0.1.0):

- **Platforms:** macOS is fully supported. On Linux, apps are detected only when their command is on `PATH`, so app detection there is minimal. Windows app detection isn't implemented yet; use `--tools` to choose tools yourself.
- **Tests:** unit tests cover catalogue parsing, server selection, install planning, client registration and the config backup.
- **Manual testing:** on a Mac, detection found Illustrator, Photoshop, After Effects and Blender. Registration with Claude Desktop, Cursor and Codex was tested against a throwaway home directory, and `doctor` confirmed four live servers.

The tool list comes from the series catalogue, [`registry/registry.json`](../registry/registry.json), which is built into the binary. It is MIT licensed.
