---
title: "InDesign implementation"
description: "Drive InDesign from an AI agent: pages, typography, tables, styles, images, long documents and export. How the MCP server connects, its conventions and setup."
url: https://creativemcp.com/docs/implementations/indesign.html
author: Zach Shallbetter (https://creativemcp.com/about.html)
modified: 2026-11-03T14:32:15-08:00
site: Creative MCP (https://creativemcp.com/, index: https://creativemcp.com/llms.txt)
copyright: © 2026 Zach Shallbetter (https://github.com/zachshallbetter/creative-mcp/blob/main/NOTICE.md)
---

[Docs](https://creativemcp.com/docs/) / [Implementation guides](https://creativemcp.com/docs/#implementation-guides) / InDesign

Implementation guide · InDesign

# InDesign

The InDesign MCP server controls documents, pages and spreads, typography, tables, styles, images and links, long-document workflows, interactive output, prepress and export.

[Docs home](https://creativemcp.com/docs/)[Getting started](https://creativemcp.com/docs/getting-started.html)[Content](https://creativemcp.com/docs/content.html)[MCP API](https://creativemcp.com/docs/api.html)[CLI & config](https://creativemcp.com/docs/cli.html)[Architecture](https://creativemcp.com/docs/architecture.html)[InDesign](https://creativemcp.com/docs/implementations/indesign.html)[Illustrator](https://creativemcp.com/docs/implementations/illustrator.html)[Photoshop](https://creativemcp.com/docs/implementations/photoshop.html)[After Effects](https://creativemcp.com/docs/implementations/aftereffects.html)[Premiere Pro](https://creativemcp.com/docs/implementations/premiere.html)[Suite](https://creativemcp.com/docs/implementations/suite.html)[Shared core](https://creativemcp.com/docs/implementations/core.html)[Blender MCP](https://creativemcp.com/docs/implementations/blender.html)[Contribute](https://creativemcp.com/docs/contributing.html)

## What this implementation covers

documents; pages, spreads and parent pages; text, stories and tables; typography and styles; images, links and color; books, indexing and cross-references; data merge; prepress and export; preferences and scripting.

For feature-level detail and evidence, see the [Capability Atlas](https://creativemcp.com/capability-atlas.html).

## Connection and requirements

Runtime route

### How it reaches the app

ExtendScript via AppleScript on macOS or COM through PowerShell on Windows. Optional UXP bridge backends are described in the package guide.

Prerequisites

### What you need

InDesign 2023 or later recommended; Node.js 18+; macOS or Windows.

Package: `adobe-mcp-indesign`. Add it as a local stdio MCP server using [CLI & client configuration](https://creativemcp.com/docs/cli.html).

## API and content conventions

Use MCP help to inspect tools by category and exact arguments. A generated tool reference lives in the package docs. The package also includes optional UXP backend and bridge setup.

Discover the running package’s exact tool names and JSON input schemas through MCP `tools/list`. If the server provides `help`, use it to inspect a tool or category. Schemas from a different package version may not match.

Target and units: Measurements in the main tool set are commonly millimetres. Verify each tool description; object indices and units are tool-specific. File paths stay under the home folder plus INDESIGN_ALLOWED_DIRS.

## CLI and configuration

This package is launched as a persistent MCP process, not as a conventional one-shot CLI. Relevant settings documented by the package include:

`INDESIGN_APP_NAME, INDESIGN_BACKEND, INDESIGN_SCRIPT_TIMEOUT_MS, INDESIGN_ALLOWED_DIRS, INDESIGN_TARGET; optional UXP bridge settings.`

Variable names and defaults are package-specific; consult the package README before changing them. File-operation permissions and bridge tokens should be configured in the client’s environment, not embedded in prompts or checked into source.

```
claude mcp add indesign -- npx -y adobe-mcp-indesign
```

For client-neutral JSON configuration and source-checkout commands, see [CLI & configuration](https://creativemcp.com/docs/cli.html).

## Full reference

The repository README is maintained alongside this implementation and covers exact setup, tool families, package-specific behavior, limitations, security and development status.

- [InDesign package README ↗](https://github.com/zachshallbetter/adobe-mcp/blob/main/packages/indesign/README.md)
- [Generated InDesign tool reference ↗](https://github.com/zachshallbetter/adobe-mcp/tree/main/packages/indesign/docs/TOOLS.md)
- [Browse package source and supporting docs ↗](https://github.com/zachshallbetter/adobe-mcp/tree/main/packages/indesign)
- [MCP API conventions ↗](https://creativemcp.com/docs/api.html)
- [Launch and environment reference ↗](https://creativemcp.com/docs/cli.html)
