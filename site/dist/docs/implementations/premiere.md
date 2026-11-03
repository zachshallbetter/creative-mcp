---
title: "Premiere Pro implementation"
description: "Drive Premiere Pro from an AI agent: sequences, timeline edits, markers, effects, captions and export, through a local UXP bridge. Setup and configuration."
url: https://creativemcp.com/docs/implementations/premiere.html
author: Zach Shallbetter (https://creativemcp.com/about.html)
modified: 2026-11-03T14:32:15-08:00
site: Creative MCP (https://creativemcp.com/, index: https://creativemcp.com/llms.txt)
copyright: © 2026 Zach Shallbetter (https://github.com/zachshallbetter/creative-mcp/blob/main/NOTICE.md)
---

[Docs](https://creativemcp.com/docs/) / [Implementation guides](https://creativemcp.com/docs/#implementation-guides) / Premiere Pro

Implementation guide · Premiere Pro

# Premiere Pro

The Premiere Pro MCP server works with projects and media, bins, sequences, timeline edits, markers, effects and keyframes, color, audio, graphics/captions, transcripts, metadata and export.

[Docs home](https://creativemcp.com/docs/)[Getting started](https://creativemcp.com/docs/getting-started.html)[Content](https://creativemcp.com/docs/content.html)[MCP API](https://creativemcp.com/docs/api.html)[CLI & config](https://creativemcp.com/docs/cli.html)[Architecture](https://creativemcp.com/docs/architecture.html)[InDesign](https://creativemcp.com/docs/implementations/indesign.html)[Illustrator](https://creativemcp.com/docs/implementations/illustrator.html)[Photoshop](https://creativemcp.com/docs/implementations/photoshop.html)[After Effects](https://creativemcp.com/docs/implementations/aftereffects.html)[Premiere Pro](https://creativemcp.com/docs/implementations/premiere.html)[Suite](https://creativemcp.com/docs/implementations/suite.html)[Shared core](https://creativemcp.com/docs/implementations/core.html)[Blender MCP](https://creativemcp.com/docs/implementations/blender.html)[Contribute](https://creativemcp.com/docs/contributing.html)

## What this implementation covers

Project and media; sequences/settings; timeline clips/tracks; markers; effects, transitions and keyframes; color and audio; graphics/captions/transcripts; metadata; export and application events.

For feature-level detail and evidence, see the [Capability Atlas](https://creativemcp.com/capability-atlas.html).

## Connection and requirements

Runtime route

### How it reaches the app

A local WebSocket server connects to the MCP Bridge UXP panel inside Premiere Pro. Premiere UXP initiates the connection to the local server.

Prerequisites

### What you need

Premiere Pro 25.6+; Node.js 18+; macOS or Windows; UXP Developer Tool to load the plugin until the package provides a published installation route.

Package: `adobe-mcp-premiere`. Add it as a local stdio MCP server using [CLI & client configuration](https://creativemcp.com/docs/cli.html).

## API and content conventions

The package guide says not yet tested against a real Premiere Pro session. Read its security, version-specific API limitations, connection steps and “What is not certain yet” section before production use.

Discover the running package’s exact tool names and JSON input schemas through MCP `tools/list`. If the server provides `help`, use it to inspect a tool or category. Schemas from a different package version may not match.

Target and units: Times are seconds. Clips use track and index values returned by list_clips; indexes shift after edits. Project item IDs come from list_project_items. Output paths must be home-folder or PREMIERE_ALLOWED_DIRS; overwrites require explicit option.

## CLI and configuration

This package is launched as a persistent MCP process, not as a conventional one-shot CLI. Relevant settings documented by the package include:

`PREMIERE_WS_PORT, BRIDGE_TOKEN or PREMIERE_BRIDGE_TOKEN, PREMIERE_ALLOWED_DIRS, PREMIERE_SCRIPT_TIMEOUT_MS, PREMIERE_CONNECT_WAIT_MS.`

Variable names and defaults are package-specific; consult the package README before changing them. File-operation permissions and bridge tokens should be configured in the client’s environment, not embedded in prompts or checked into source.

```
claude mcp add premiere -- npx -y adobe-mcp-premiere
```

For client-neutral JSON configuration and source-checkout commands, see [CLI & configuration](https://creativemcp.com/docs/cli.html).

## Full reference

The repository README is maintained alongside this implementation and covers exact setup, tool families, package-specific behavior, limitations, security and development status.

- [Premiere Pro package README ↗](https://github.com/zachshallbetter/adobe-mcp/blob/main/packages/premiere/README.md)
- [Browse package source and supporting docs ↗](https://github.com/zachshallbetter/adobe-mcp/tree/main/packages/premiere)
- [MCP API conventions ↗](https://creativemcp.com/docs/api.html)
- [Launch and environment reference ↗](https://creativemcp.com/docs/cli.html)
