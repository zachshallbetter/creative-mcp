---
title: "Blender MCP implementation"
description: "Blender MCP pairs a Rust MCP server with a Blender add-on so AI agents can work with scenes and assets. Install with Homebrew; setup and safety notes."
url: https://creativemcp.com/docs/implementations/blender.html
author: Zach Shallbetter (https://creativemcp.com/about.html)
modified: 2026-11-03T18:21:43-08:00
site: Creative MCP (https://creativemcp.com/, index: https://creativemcp.com/llms.txt)
copyright: © 2026 Zach Shallbetter (https://github.com/zachshallbetter/creative-mcp/blob/main/NOTICE.md)
---

[Docs](https://creativemcp.com/docs/) / [Implementation guides](https://creativemcp.com/docs/#implementation-guides) / Blender MCP

Implementation guide · Blender MCP

# Blender MCP

A Rust MCP server and Blender companion add-on for scene and asset workflows. The server speaks MCP over stdio; a local IPC connection links it to Blender.

[Docs home](https://creativemcp.com/docs/)[Getting started](https://creativemcp.com/docs/getting-started.html)[Content](https://creativemcp.com/docs/content.html)[MCP API](https://creativemcp.com/docs/api.html)[CLI & config](https://creativemcp.com/docs/cli.html)[Architecture](https://creativemcp.com/docs/architecture.html)[InDesign](https://creativemcp.com/docs/implementations/indesign.html)[Illustrator](https://creativemcp.com/docs/implementations/illustrator.html)[Photoshop](https://creativemcp.com/docs/implementations/photoshop.html)[After Effects](https://creativemcp.com/docs/implementations/aftereffects.html)[Premiere Pro](https://creativemcp.com/docs/implementations/premiere.html)[Suite](https://creativemcp.com/docs/implementations/suite.html)[Shared core](https://creativemcp.com/docs/implementations/core.html)[Blender MCP](https://creativemcp.com/docs/implementations/blender.html)[Contribute](https://creativemcp.com/docs/contributing.html)

## What this implementation covers

The project combines a Rust MCP server with a Blender add-on that performs scene operations. Its repository documents tools for scene data, geometry, asset workflows and Blender resources. For the current list of tools, parameters and behavior, use the [tool reference](https://github.com/zachshallbetter/blender-mcp-rs/blob/main/docs/tools.md) maintained with the source.

The project is independent from Adobe MCP. They can be configured as separate servers in a compatible MCP client; direct Adobe-to-Blender handoff tools are not currently provided.

## Connection and requirements

Transport

### Local server and add-on

The client starts the MCP process over stdio. The companion add-on runs inside Blender and accepts the server’s local IPC connection.

Prerequisites

### Blender and Rust

Install Rust with rustup. Blender 4.0 or later is required, and the companion add-on must be installed and enabled (`blender-mcp addon install` does both).

Source package: `blender-mcp-cli` in the [blender-mcp-rs repository](https://github.com/zachshallbetter/blender-mcp-rs).

## Install and configure

Install the CLI from the project’s Git repository:

```
cargo install --locked --git https://github.com/zachshallbetter/blender-mcp-rs --package blender-mcp-cli
```

Or install with Homebrew. Homebrew builds the v0.5.1 release for you, so you don’t need Rust installed:

```
brew tap zachshallbetter/tap
brew trust zachshallbetter/tap
brew install zachshallbetter/tap/blender-mcp
blender-mcp addon install
```

`blender-mcp addon install` copies the companion add-on into each Blender version on your Mac and turns it on, so it loads and listens on `127.0.0.1:9876` whenever Blender opens. There is nothing to enable in Preferences. Quit Blender before running it; if Blender is open, the command leaves its preferences alone and tells you. The project CLI can add configuration for supported clients, including Codex, Claude Code, Cursor, Gemini CLI and Antigravity:

```
blender-mcp client add codex
```

For client-specific scopes, add-on installation steps and exact options, follow the [current repository quick start](https://github.com/zachshallbetter/blender-mcp-rs/blob/main/README.md).

## Safety and limitations

The add-on listens on loopback and is intended for local use; it does not authenticate local clients. The `execute_blender_code` tool runs Python with Blender’s process permissions. Its pattern checks are not a security sandbox. Connect only trusted agents and review code before execution.

Scene-editing and file-writing tools can change or overwrite local work. Some asset search and import tools contact Poly Haven services; consult the project’s current README for the destinations and data involved.

## Full reference

- [Tool reference catalog ↗](https://github.com/zachshallbetter/blender-mcp-rs/blob/main/docs/tools.md)
- [Blender companion add-on guide ↗](https://github.com/zachshallbetter/blender-mcp-rs/blob/main/docs/addon.md)
- [Architecture and transport ↗](https://github.com/zachshallbetter/blender-mcp-rs/blob/main/docs/architecture.md)
- [Source repository ↗](https://github.com/zachshallbetter/blender-mcp-rs)
