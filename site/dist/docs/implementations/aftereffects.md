---
title: "After Effects implementation"
description: "Drive After Effects from an AI agent: compositions, layers, keyframes, expressions, effects, text and the render queue. Setup, connection and configuration."
url: https://creativemcp.com/docs/implementations/aftereffects.html
author: Zach Shallbetter (https://creativemcp.com/about.html)
modified: 2026-11-03T14:32:15-08:00
site: Creative MCP (https://creativemcp.com/, index: https://creativemcp.com/llms.txt)
copyright: © 2026 Zach Shallbetter (https://github.com/zachshallbetter/creative-mcp/blob/main/NOTICE.md)
---

[Docs](https://creativemcp.com/docs/) / [Implementation guides](https://creativemcp.com/docs/#implementation-guides) / After Effects

Implementation guide · After Effects

# After Effects

The After Effects MCP server works with projects and footage, compositions and layers, time and keyframes, expressions, effects, text animators, masks, shape layers, paint/roto, 3D, tracking, color and the render queue.

[Docs home](https://creativemcp.com/docs/)[Getting started](https://creativemcp.com/docs/getting-started.html)[Content](https://creativemcp.com/docs/content.html)[MCP API](https://creativemcp.com/docs/api.html)[CLI & config](https://creativemcp.com/docs/cli.html)[Architecture](https://creativemcp.com/docs/architecture.html)[InDesign](https://creativemcp.com/docs/implementations/indesign.html)[Illustrator](https://creativemcp.com/docs/implementations/illustrator.html)[Photoshop](https://creativemcp.com/docs/implementations/photoshop.html)[After Effects](https://creativemcp.com/docs/implementations/aftereffects.html)[Premiere Pro](https://creativemcp.com/docs/implementations/premiere.html)[Suite](https://creativemcp.com/docs/implementations/suite.html)[Shared core](https://creativemcp.com/docs/implementations/core.html)[Blender MCP](https://creativemcp.com/docs/implementations/blender.html)[Contribute](https://creativemcp.com/docs/contributing.html)

## What this implementation covers

project and footage; compositions; layers and property paths; keyframes and animation; effects; text, shapes, masks and paint; tracking and 3D; render settings, output modules and queue management.

For feature-level detail and evidence, see the [Capability Atlas](https://creativemcp.com/capability-atlas.html).

## Connection and requirements

Runtime route

### How it reaches the app

ExtendScript through local desktop automation; Windows may use AfterFX.exe to run scripts. Compatible operations can route to EffectCraft.

Prerequisites

### What you need

After Effects 2023 or later; Node.js 18+; macOS or Windows.

Package: `adobe-mcp-aftereffects`. Add it as a local stdio MCP server using [CLI & client configuration](https://creativemcp.com/docs/cli.html).

## API and content conventions

Use help for the exact tool schema. Static script audit does not establish that operations have been validated in a live After Effects session; use the package README for current evidence status.

Discover the running package’s exact tool names and JSON input schemas through MCP `tools/list`. If the server provides `help`, use it to inspect a tool or category. Schemas from a different package version may not match.

Target and units: Use project item IDs or composition names as documented. Layer indices follow After Effects’ 1-based convention; time values are seconds where specified. File operations use home-folder confinement plus AFTEREFFECTS_ALLOWED_DIRS.

## CLI and configuration

This package is launched as a persistent MCP process, not as a conventional one-shot CLI. Relevant settings documented by the package include:

`AFTEREFFECTS_EXE (Windows), AFTEREFFECTS_ALLOWED_DIRS, AFTEREFFECTS_SCRIPT_TIMEOUT_MS, AFTEREFFECTS_TARGET and EffectCraft control settings where configured.`

Variable names and defaults are package-specific; consult the package README before changing them. File-operation permissions and bridge tokens should be configured in the client’s environment, not embedded in prompts or checked into source.

```
claude mcp add aftereffects -- npx -y adobe-mcp-aftereffects
```

For client-neutral JSON configuration and source-checkout commands, see [CLI & configuration](https://creativemcp.com/docs/cli.html).

## Full reference

The repository README is maintained alongside this implementation and covers exact setup, tool families, package-specific behavior, limitations, security and development status.

- [After Effects package README ↗](https://github.com/zachshallbetter/adobe-mcp/blob/main/packages/aftereffects/README.md)
- [Browse package source and supporting docs ↗](https://github.com/zachshallbetter/adobe-mcp/tree/main/packages/aftereffects)
- [MCP API conventions ↗](https://creativemcp.com/docs/api.html)
- [Launch and environment reference ↗](https://creativemcp.com/docs/cli.html)
