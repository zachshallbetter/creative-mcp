---
title: "Illustrator implementation"
description: "Drive Illustrator from an AI agent: artboards, layers, paths, type, color, effects, symbols and export. How the MCP server connects, its conventions and setup."
url: https://creativemcp.com/docs/implementations/illustrator.html
author: Zach Shallbetter (https://creativemcp.com/about.html)
modified: 2026-11-03T14:32:15-08:00
site: Creative MCP (https://creativemcp.com/, index: https://creativemcp.com/llms.txt)
copyright: © 2026 Zach Shallbetter (https://github.com/zachshallbetter/creative-mcp/blob/main/NOTICE.md)
---

[Docs](https://creativemcp.com/docs/) / [Implementation guides](https://creativemcp.com/docs/#implementation-guides) / Illustrator

Implementation guide · Illustrator

# Illustrator

The Illustrator MCP server works with documents, artboards, layers, vector shapes and paths, text, color, appearance, effects, symbols, patterns, images, actions, variables and export.

[Docs home](https://creativemcp.com/docs/)[Getting started](https://creativemcp.com/docs/getting-started.html)[Content](https://creativemcp.com/docs/content.html)[MCP API](https://creativemcp.com/docs/api.html)[CLI & config](https://creativemcp.com/docs/cli.html)[Architecture](https://creativemcp.com/docs/architecture.html)[InDesign](https://creativemcp.com/docs/implementations/indesign.html)[Illustrator](https://creativemcp.com/docs/implementations/illustrator.html)[Photoshop](https://creativemcp.com/docs/implementations/photoshop.html)[After Effects](https://creativemcp.com/docs/implementations/aftereffects.html)[Premiere Pro](https://creativemcp.com/docs/implementations/premiere.html)[Suite](https://creativemcp.com/docs/implementations/suite.html)[Shared core](https://creativemcp.com/docs/implementations/core.html)[Blender MCP](https://creativemcp.com/docs/implementations/blender.html)[Contribute](https://creativemcp.com/docs/contributing.html)

## What this implementation covers

document setup; artboards; artwork, shapes, paths and transforms; layers and selection; text and styles; color and appearance; effects, blends, brushes, patterns and symbols; images and export; actions and prepress.

For feature-level detail and evidence, see the [Capability Atlas](https://creativemcp.com/capability-atlas.html).

## Connection and requirements

Runtime route

### How it reaches the app

ExtendScript via AppleScript on macOS or COM through PowerShell on Windows.

Prerequisites

### What you need

Illustrator 2023 or later; Node.js 18+; macOS or Windows.

Package: `adobe-mcp-illustrator`. Add it as a local stdio MCP server using [CLI & client configuration](https://creativemcp.com/docs/cli.html).

## API and content conventions

Use live MCP tools/list and help for exact definitions. Check coordinate direction, artboard index and target selector before editing.

Discover the running package’s exact tool names and JSON input schemas through MCP `tools/list`. If the server provides `help`, use it to inspect a tool or category. Schemas from a different package version may not match.

Target and units: Geometry uses points from the active artboard’s top-left; y increases downward. Target artwork by item ids, names or selection. Color accepts named swatches or documented color strings. File paths are home-folder confined plus ILLUSTRATOR_ALLOWED_DIRS.

## CLI and configuration

This package is launched as a persistent MCP process, not as a conventional one-shot CLI. Relevant settings documented by the package include:

`ILLUSTRATOR_APP_NAME, ILLUSTRATOR_SCRIPT_TIMEOUT_MS, ILLUSTRATOR_ALLOWED_DIRS, ILLUSTRATOR_TARGET and documented shared backend settings.`

Variable names and defaults are package-specific; consult the package README before changing them. File-operation permissions and bridge tokens should be configured in the client’s environment, not embedded in prompts or checked into source.

```
claude mcp add illustrator -- npx -y adobe-mcp-illustrator
```

For client-neutral JSON configuration and source-checkout commands, see [CLI & configuration](https://creativemcp.com/docs/cli.html).

## Full reference

The repository README is maintained alongside this implementation and covers exact setup, tool families, package-specific behavior, limitations, security and development status.

- [Illustrator package README ↗](https://github.com/zachshallbetter/adobe-mcp/blob/main/packages/illustrator/README.md)
- [Browse package source and supporting docs ↗](https://github.com/zachshallbetter/adobe-mcp/tree/main/packages/illustrator)
- [MCP API conventions ↗](https://creativemcp.com/docs/api.html)
- [Launch and environment reference ↗](https://creativemcp.com/docs/cli.html)
