# Creative MCP

**Local MCP servers that connect AI agents to the creative tools you already use.**
Created by [Zach Shallbetter](https://zachshallbetter.com/) · [creativemcp.com](https://creativemcp.com)

Creative MCP is a series of independent, open [Model Context Protocol](https://modelcontextprotocol.io) implementations for creative software. Each tool lives in its own repository. This repository is the series' home: the website and docs, the catalogue of tools, the planned installer and the conventions every tool follows.

| Tool | Repository | Install |
| --- | --- | --- |
| **Adobe MCP**: InDesign, Illustrator, Photoshop, After Effects, Premiere Pro, plus a cross-app coordinator | [adobe-mcp](https://github.com/zachshallbetter/adobe-mcp) | `brew install zachshallbetter/tap/adobe-mcp` or `npx -y adobe-mcp-suite` |
| **Blender MCP**: Rust MCP server and companion Blender add-on | [blender-mcp-rs](https://github.com/zachshallbetter/blender-mcp-rs) | `brew install zachshallbetter/tap/blender-mcp` (v0.5.0) |

Homebrew needs the tap first, once:

```sh
brew tap zachshallbetter/tap
brew trust zachshallbetter/tap   # Homebrew 7 and later
```

Start at [creativemcp.com/start.html](https://creativemcp.com/start.html) for the exact commands for your app and MCP client.

## What's in this repository

| Path | Contents |
| --- | --- |
| [`site/`](site) | creativemcp.com: static pages in `site/dist`, generator scripts in `site/scripts`, Vercel config |
| [`registry/`](registry) | The series catalogue (`registry.json`), its JSON Schema and validator. Draft: nothing reads it yet. |
| [`cli/`](cli) | The planned `creative-mcp` installer. Not started. |
| [`docs/`](docs) | [Architecture and conventions](docs/ARCHITECTURE.md), [Digital Presence Protocol conformance](docs/DPP-CONFORMANCE.md), [analytics](docs/ANALYTICS.md) |

## Working on the site

The site is plain HTML, CSS and JavaScript with no build step. Preview it with any static server:

```sh
python3 -m http.server 4173 --directory site/dist
```

After changing page content, commit, then regenerate the derived files:

```sh
python3 site/scripts/generate-discovery.py      # sitemap, llms.txt, llms-full.txt, registry copy
python3 site/scripts/generate-social-images.py  # social cards (needs Google Chrome)
```

## Status vocabulary

Every capability claim uses one of **Documented**, **Implemented**, **Tested** or **Validated**. Planned work is labelled **Planned**. A lower level never implies a higher one.

## Licence

Code (scripts, site JavaScript and CSS, the catalogue, the installer) is [MIT](LICENSE). Written content, the Creative MCP name and symbol, the brush artwork and the portrait are not licensed for reuse; see [NOTICE](NOTICE.md).
