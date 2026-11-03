---
title: "Workflows"
description: "Worked examples of Adobe MCP and Blender MCP in real creative projects, with automated steps and human-managed handoffs clearly marked."
url: https://creativemcp.com/workflows.html
author: Zach Shallbetter (https://creativemcp.com/about.html)
modified: 2026-11-03T03:44:30-08:00
site: Creative MCP (https://creativemcp.com/, index: https://creativemcp.com/llms.txt)
copyright: © 2026 Zach Shallbetter (https://github.com/zachshallbetter/creative-mcp/blob/main/NOTICE.md)
---

Examples · real handoffs, honest boundaries

# Creative work, carried forward.

Explore concrete ways to use the tools together. Adobe MCP currently automates two Illustrator-to-InDesign workflows. Blender MCP provides its own Blender tools. Other transitions below are human-managed handoffs unless an implementation explicitly supports them.

Adobe MCP · supported workflows

## Two useful transitions, end to end.

The Adobe coordinator exposes these cross-app actions. It does not claim to connect every Adobe app or send files to Blender automatically.

01

Illustrator → InDesign

### Place artwork in a layout.

Place an Illustrator artboard into an InDesign page as a link to the source `.ai` file. Keep the editable source in Illustrator and arrange the placed artwork in the publication.
[Read the coordinator guide ↗](https://creativemcp.com/docs/implementations/suite.html)

02

Illustrator → InDesign

### Carry exact swatches across.

Copy supported swatch values from the Illustrator document into InDesign, keeping the color decisions consistent across the artwork and page layout.
[Read the coordinator guide ↗](https://creativemcp.com/docs/implementations/suite.html)

Creative MCP · independent implementations

## Build a bridge when the tools differ.

These examples pair the independently installed Adobe MCP and Blender MCP projects. Export, import and transfer steps are explicit manual actions; the projects do not share an automatic orchestration layer today.

03

Blender MCP · local scene tools

### Inspect a scene before changing it.

Connect Blender MCP to a supported client, ask for scene information with `get_scene_info`, then use Blender’s own interface or MCP tools for the next edit.
[Read the Blender guide ↗](https://creativemcp.com/docs/implementations/blender.html)

04

Human-managed handoff

### Take a layout idea into 3D.

Prepare a graphic in Illustrator or InDesign, export an asset in the format you need, then import it into Blender yourself. Ask Blender MCP to inspect the scene or help with supported operations. File movement and app-to-app coordination remain yours.
[Use the starter brief ↗](https://creativemcp.com/downloads/creative-mcp-starter.zip)

Keep the boundary clear. The browser site does not inspect your computer, open creative applications or transmit local files. MCP servers run locally through the client you configure.

Make your own

## Start with a brief, not a promise.

The starter pack includes a creative brief and reference board for a small layout-to-3D exercise. It leaves the export format, transfer and Blender setup to your chosen workflow.
[Download the starter pack](https://creativemcp.com/downloads/creative-mcp-starter.zip)
