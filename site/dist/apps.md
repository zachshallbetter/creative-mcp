---
title: "Adobe MCP"
description: "Adobe MCP gives InDesign, Illustrator, Photoshop, After Effects and Premiere Pro their own local MCP servers, plus a coordinator for two cross-app handoffs."
url: https://creativemcp.com/apps.html
author: Zach Shallbetter (https://creativemcp.com/about.html)
modified: 2026-11-03T03:44:30-08:00
site: Creative MCP (https://creativemcp.com/, index: https://creativemcp.com/llms.txt)
copyright: © 2026 Zach Shallbetter (https://github.com/zachshallbetter/creative-mcp/blob/main/NOTICE.md)
---

Adobe MCP · five creative surfaces

# Pick up the work where it lives.

Each Adobe app has its own local MCP server, tools and workflow. The Adobe MCP coordinator adds two supported handoffs, with an implementation guide for every surface.

[ID

## InDesign

Pages, layouts, typography, tables, styles, long documents and export.
adobe-mcp-indesignImplementation guide ↗](https://creativemcp.com/docs/implementations/indesign.html) [AI

## Illustrator

Artboards, vector shapes, paths, layers, type, color and artwork export.
adobe-mcp-illustratorImplementation guide ↗](https://creativemcp.com/docs/implementations/illustrator.html) [PS

## Photoshop

Documents, pixels, layers, selections, adjustments, filters and export.
adobe-mcp-photoshopImplementation guide ↗](https://creativemcp.com/docs/implementations/photoshop.html) [Ae

## After Effects

Projects, footage, compositions, keyframes, effects and rendering.
adobe-mcp-aftereffectsImplementation guide ↗](https://creativemcp.com/docs/implementations/aftereffects.html) [Pr

## Premiere Pro

Projects, bins, sequences, timeline edits, markers, effects and exports.
adobe-mcp-premiereImplementation guide ↗](https://creativemcp.com/docs/implementations/premiere.html)

Beyond separate tools

## Carry creative decisions across apps.

The suite includes explicit cross-app workflows so source and destination apps can participate in a single task.

01

Place Illustrator artwork in InDesignPlace an artboard into a page as a link to the .ai file.
`Illustrator → InDesign`

02

Copy swatches into InDesignCarry exact color values from the Illustrator document.
`Illustrator → InDesign`

[Browse the full feature inventory →](https://creativemcp.com/features.html)

Install

## Every Adobe server in one install.

Homebrew installs the Suite coordinator and all five app servers onto your `PATH`. Prefer not to install anything? Each server also runs on demand with `npx`.

```
brew tap zachshallbetter/tap
brew trust zachshallbetter/tap
brew install zachshallbetter/tap/adobe-mcp
```

[Build the setup for your client →](https://creativemcp.com/start.html)
