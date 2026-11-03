---
title: "Suite coordinator implementation"
description: "The Adobe MCP Suite coordinator runs the app servers together and automates two handoffs: placing Illustrator artwork and copying swatches into InDesign."
url: https://creativemcp.com/docs/implementations/suite.html
author: Zach Shallbetter (https://creativemcp.com/about.html)
modified: 2026-11-03T14:32:15-08:00
site: Creative MCP (https://creativemcp.com/, index: https://creativemcp.com/llms.txt)
copyright: © 2026 Zach Shallbetter (https://github.com/zachshallbetter/creative-mcp/blob/main/NOTICE.md)
---

[Docs](https://creativemcp.com/docs/) / [Implementation guides](https://creativemcp.com/docs/#implementation-guides) / Suite coordinator

Implementation guide · Suite coordinator

# Suite coordinator

The Suite coordinator joins app servers behind two explicit cross-app workflows: placing Illustrator artwork in InDesign and copying supported Illustrator swatches into InDesign.

[Docs home](https://creativemcp.com/docs/)[Getting started](https://creativemcp.com/docs/getting-started.html)[Content](https://creativemcp.com/docs/content.html)[MCP API](https://creativemcp.com/docs/api.html)[CLI & config](https://creativemcp.com/docs/cli.html)[Architecture](https://creativemcp.com/docs/architecture.html)[InDesign](https://creativemcp.com/docs/implementations/indesign.html)[Illustrator](https://creativemcp.com/docs/implementations/illustrator.html)[Photoshop](https://creativemcp.com/docs/implementations/photoshop.html)[After Effects](https://creativemcp.com/docs/implementations/aftereffects.html)[Premiere Pro](https://creativemcp.com/docs/implementations/premiere.html)[Suite](https://creativemcp.com/docs/implementations/suite.html)[Shared core](https://creativemcp.com/docs/implementations/core.html)[Blender MCP](https://creativemcp.com/docs/implementations/blender.html)[Contribute](https://creativemcp.com/docs/contributing.html)

## What this implementation covers

Workflow: place an artboard from the active Illustrator document on an InDesign page as a linked .ai file; copy supported RGB/CMYK process, spot and gray swatches while preserving InDesign built-ins. Gradients and patterns are reported, not copied.

For feature-level detail and evidence, see the [Capability Atlas](https://creativemcp.com/capability-atlas.html).

## Connection and requirements

Runtime route

### How it reaches the app

Starts standalone MCP server implementations as local child processes and delegates workflow steps through MCP. It speaks MCP over stdio to the client.

Prerequisites

### What you need

Node.js 18+ and InDesign plus Illustrator for its current workflows. Other child apps can be selected, but that does not add coordinator workflows.

Package: `adobe-mcp-suite`. Add it as a local stdio MCP server using [CLI & client configuration](https://creativemcp.com/docs/cli.html).

## API and content conventions

The coordinator advertises its own workflow tools only; it does not mirror child tool inventories. Install standalone app servers for direct app tools. Use its package README for delegation semantics and constraints.

Discover the running package’s exact tool names and JSON input schemas through MCP `tools/list`. If the server provides `help`, use it to inspect a tool or category. Schemas from a different package version may not match.

Target and units: Select child app IDs with --apps or ADOBE_MCP_APPS. Current workflows need both InDesign and Illustrator. File permissions remain app-specific: configure both participating app servers as needed.

## CLI and configuration

This package is launched as a persistent MCP process, not as a conventional one-shot CLI. Relevant settings documented by the package include:

`ADOBE_MCP_APPS plus per-app variables documented by each child package; the coordinator forwards matching variables rather than acting as a single app backend.`

Variable names and defaults are package-specific; consult the package README before changing them. File-operation permissions and bridge tokens should be configured in the client’s environment, not embedded in prompts or checked into source.

```
claude mcp add suite -- npx -y adobe-mcp-suite
```

With Homebrew, the coordinator and every app server are installed on your `PATH`:

```
brew tap zachshallbetter/tap
brew trust zachshallbetter/tap
brew install zachshallbetter/tap/adobe-mcp
claude mcp add suite -- adobe-mcp-suite
```

For client-neutral JSON configuration and source-checkout commands, see [CLI & configuration](https://creativemcp.com/docs/cli.html).

## Full reference

The repository README is maintained alongside this implementation and covers exact setup, tool families, package-specific behavior, limitations, security and development status.

- [Suite coordinator package README ↗](https://github.com/zachshallbetter/adobe-mcp/blob/main/packages/suite/README.md)
- [Browse package source and supporting docs ↗](https://github.com/zachshallbetter/adobe-mcp/tree/main/packages/suite)
- [MCP API conventions ↗](https://creativemcp.com/docs/api.html)
- [Launch and environment reference ↗](https://creativemcp.com/docs/cli.html)
