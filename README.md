# Creative MCP

**Local MCP servers that connect AI agents to the creative tools you already use.**
Created by [Zach Shallbetter](https://zachshallbetter.com/) · [creativemcp.com](https://creativemcp.com)

Creative MCP is a series of independent, open [Model Context Protocol](https://modelcontextprotocol.io) implementations for creative software. Each tool lives in its own repository. This repository is the series' home: the website and docs, the catalogue of tools, the `creative-mcp` installer and the conventions every tool follows.

| Tool | Repository | Install |
| --- | --- | --- |
| **Adobe MCP**: InDesign, Illustrator, Photoshop, After Effects, Premiere Pro, plus a cross-app coordinator | [adobe-mcp](https://github.com/zachshallbetter/adobe-mcp) | `brew install zachshallbetter/tap/adobe-mcp` or `npx -y adobe-mcp-suite` |
| **Blender MCP**: Rust MCP server and companion Blender add-on | [blender-mcp-rs](https://github.com/zachshallbetter/blender-mcp-rs) | `brew install zachshallbetter/tap/blender-mcp` (v0.5.1) |

Homebrew needs the tap first, once:

```sh
brew tap zachshallbetter/tap
brew trust zachshallbetter/tap   # Homebrew 7 and later
```

Or set up everything at once on a Mac. The installer detects your apps and MCP clients, shows the plan, then installs, registers and verifies:

```sh
brew install zachshallbetter/tap/creative-mcp
creative-mcp setup
```

Afterwards, `creative-mcp doctor` checks that every server answers, that each open app is reachable and which clients have which servers; `creative-mcp update` keeps everything current; `creative-mcp remove <tool>` takes a tool out again. See [the installer reference](https://creativemcp.com/docs/cli.html#installer).

To do it by hand, start at [creativemcp.com/start.html](https://creativemcp.com/start.html) for the exact commands for your app and MCP client.

## What's in this repository

| Path | Contents |
| --- | --- |
| [`site/`](site) | creativemcp.com: static pages in `site/dist`, generator scripts in `site/scripts`, Vercel config |
| [`registry/`](registry) | The series catalogue (`registry.json`), its JSON Schema and validator. The installer is built from it. |
| [`cli/`](cli) | The `creative-mcp` installer (Rust): detects apps and MCP clients, installs, registers and verifies |
| [`docs/`](docs) | [Architecture and conventions](docs/ARCHITECTURE.md), [Digital Presence Protocol conformance](docs/DPP-CONFORMANCE.md), [analytics](docs/ANALYTICS.md) |

## Working on the site

The site is plain HTML, CSS and JavaScript with no build step. Preview it with any static server:

```sh
python3 -m http.server 4173 --directory site/dist
```

After changing the catalogue or page content, regenerate the derived files and check the result (CI runs the last step on every push):

```sh
python3 site/scripts/generate-catalogue.py      # Projects grid, series table, setup builder data, from registry/registry.json
python3 site/scripts/generate-social-images.py  # social cards (needs Google Chrome), only when a title or card changes
python3 site/scripts/apply-shell.py             # shared head, JSON-LD, docs tabs, footer
python3 site/scripts/generate-discovery.py      # sitemap, llms.txt, llms-full.txt, Markdown copies
python3 site/scripts/check-site.py              # every page, card, JSON-LD and CSP hash agrees
```

## Verification

Each part has its own check, and CI runs them when the matching paths change:

```sh
python3 registry/validate.py                                                   # catalogue schema and cross-references (needs jsonschema)
python3 site/scripts/check-site.py                                             # site pages, cards, JSON-LD and discovery files agree
cd cli && cargo fmt --check && cargo clippy --all-targets -- -D warnings && cargo test   # the installer
```

These are structural checks and unit tests. Whether a tool works in a real application is stated per claim with the
status vocabulary below.

## Status and limitations

The installer's tested platforms, manual testing and gaps are listed in [`cli/README.md`](cli/README.md#status): macOS is
fully supported, Windows detection has not been checked on a real Windows machine, and Linux detects only Blender.
Open work is tracked in [GitHub issues](https://github.com/zachshallbetter/creative-mcp/issues) and in
[`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md#7-tracked-work). The tools themselves live in their own repositories and
carry their own status.

## Status vocabulary

Every capability claim uses one of **Documented**, **Implemented**, **Tested** or **Validated**. Planned work is labelled **Planned**. A lower level never implies a higher one.

## Licence

[![M8ven Score](https://m8ven.ai/badge/mcp/zachshallbetter-creative-mcp-uqcr85?v=fe216d203501d964cb91617d1ced51b3)](https://m8ven.ai/mcp/zachshallbetter-creative-mcp-uqcr85?s=readme)