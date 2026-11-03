---
title: "Photoshop implementation"
description: "Drive Photoshop from an AI agent: layers, masks, selections, adjustments, filters, type and export. How the MCP server connects, its conventions and setup."
url: https://creativemcp.com/docs/implementations/photoshop.html
author: Zach Shallbetter (https://creativemcp.com/about.html)
modified: 2026-11-03T14:32:15-08:00
site: Creative MCP (https://creativemcp.com/, index: https://creativemcp.com/llms.txt)
copyright: © 2026 Zach Shallbetter (https://github.com/zachshallbetter/creative-mcp/blob/main/NOTICE.md)
---

[Docs](https://creativemcp.com/docs/) / [Implementation guides](https://creativemcp.com/docs/#implementation-guides) / Photoshop

Implementation guide · Photoshop

# Photoshop

The Photoshop MCP server works with documents and formats, canvas and image data, layers and masks, selections and channels, adjustments, filters, painting and retouching, type, paths, actions, batch operations and export.

[Docs home](https://creativemcp.com/docs/)[Getting started](https://creativemcp.com/docs/getting-started.html)[Content](https://creativemcp.com/docs/content.html)[MCP API](https://creativemcp.com/docs/api.html)[CLI & config](https://creativemcp.com/docs/cli.html)[Architecture](https://creativemcp.com/docs/architecture.html)[InDesign](https://creativemcp.com/docs/implementations/indesign.html)[Illustrator](https://creativemcp.com/docs/implementations/illustrator.html)[Photoshop](https://creativemcp.com/docs/implementations/photoshop.html)[After Effects](https://creativemcp.com/docs/implementations/aftereffects.html)[Premiere Pro](https://creativemcp.com/docs/implementations/premiere.html)[Suite](https://creativemcp.com/docs/implementations/suite.html)[Shared core](https://creativemcp.com/docs/implementations/core.html)[Blender MCP](https://creativemcp.com/docs/implementations/blender.html)[Contribute](https://creativemcp.com/docs/contributing.html)

## What this implementation covers

documents and formats; canvas, image and color; layers, masks and smart objects; selections and channels; adjustment/fill layers; filters; brush and retouch; type and paths; timeline, actions and batch; export and print.

For feature-level detail and evidence, see the [Capability Atlas](https://creativemcp.com/capability-atlas.html).

## Connection and requirements

Runtime route

### How it reaches the app

ExtendScript through local desktop automation, with documented local ArtCraft adapter support for compatible tools.

Prerequisites

### What you need

Photoshop 2023 or later; Node.js 18+; macOS or Windows.

Package: `adobe-mcp-photoshop`. Add it as a local stdio MCP server using [CLI & client configuration](https://creativemcp.com/docs/cli.html).

## API and content conventions

The server includes a help tool. The package README describes tool families, units, target selection, timeout behavior and backend configuration.

Discover the running package’s exact tool names and JSON input schemas through MCP `tools/list`. If the server provides `help`, use it to inspect a tool or category. Schemas from a different package version may not match.

Target and units: Geometry is in pixels from the document’s top-left. Layer targets use IDs from list_layers, names or active:true. Supported color inputs vary by tool; review each schema. File access is home-folder confined plus PHOTOSHOP_ALLOWED_DIRS.

## CLI and configuration

This package is launched as a persistent MCP process, not as a conventional one-shot CLI. Relevant settings documented by the package include:

`PHOTOSHOP_APP_NAME, PHOTOSHOP_SCRIPT_TIMEOUT_MS, PHOTOSHOP_ALLOWED_DIRS, PHOTOSHOP_BACKEND and PHOTOSHOP_TARGET; ArtCraft control settings where applicable.`

Variable names and defaults are package-specific; consult the package README before changing them. File-operation permissions and bridge tokens should be configured in the client’s environment, not embedded in prompts or checked into source.

```
claude mcp add photoshop -- npx -y adobe-mcp-photoshop
```

For client-neutral JSON configuration and source-checkout commands, see [CLI & configuration](https://creativemcp.com/docs/cli.html).

## Full reference

The repository README is maintained alongside this implementation and covers exact setup, tool families, package-specific behavior, limitations, security and development status.

- [Photoshop package README ↗](https://github.com/zachshallbetter/adobe-mcp/blob/main/packages/photoshop/README.md)
- [Browse package source and supporting docs ↗](https://github.com/zachshallbetter/adobe-mcp/tree/main/packages/photoshop)
- [MCP API conventions ↗](https://creativemcp.com/docs/api.html)
- [Launch and environment reference ↗](https://creativemcp.com/docs/cli.html)
