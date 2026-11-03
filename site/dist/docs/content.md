---
title: "Content & capability guide"
description: "What each Adobe MCP server works on, and how an Adobe feature, an automation route, an MCP tool and a live-verified workflow differ."
url: https://creativemcp.com/docs/content.html
author: Zach Shallbetter (https://creativemcp.com/about.html)
modified: 2026-11-03T14:32:15-08:00
site: Creative MCP (https://creativemcp.com/, index: https://creativemcp.com/llms.txt)
copyright: © 2026 Zach Shallbetter (https://github.com/zachshallbetter/creative-mcp/blob/main/NOTICE.md)
---

[Docs](https://creativemcp.com/docs/) / Content & capability guide

What the tools work on

# Content & capability guide

Understand the difference between an Adobe feature, an automation route, an MCP implementation, a passing static check, and a workflow verified in the live application.

[Docs home](https://creativemcp.com/docs/)[Getting started](https://creativemcp.com/docs/getting-started.html)[Content](https://creativemcp.com/docs/content.html)[MCP API](https://creativemcp.com/docs/api.html)[CLI & config](https://creativemcp.com/docs/cli.html)[Architecture](https://creativemcp.com/docs/architecture.html)[InDesign](https://creativemcp.com/docs/implementations/indesign.html)[Illustrator](https://creativemcp.com/docs/implementations/illustrator.html)[Photoshop](https://creativemcp.com/docs/implementations/photoshop.html)[After Effects](https://creativemcp.com/docs/implementations/aftereffects.html)[Premiere Pro](https://creativemcp.com/docs/implementations/premiere.html)[Suite](https://creativemcp.com/docs/implementations/suite.html)[Shared core](https://creativemcp.com/docs/implementations/core.html)[Blender MCP](https://creativemcp.com/docs/implementations/blender.html)[Contribute](https://creativemcp.com/docs/contributing.html)

## Six different things called “coverage”

| Stage | What it means | What it does not establish

| Documented | A feature appears in Adobe product guides, scripting references or other cited source material. | That an API can reach it or an MCP server has a tool for it.

| Scriptable | A known automation route may reach the feature: object model, menu command, UXP API or another documented route. | That the route is supported on every OS/app version or covered by this project.

| Implemented | An MCP tool is mapped to the feature in the atlas dataset. | That the mapping is complete, the app accepted the operation, or the result is correct.

| Static checks | Generated scripts were exercised with a stubbed application and checked against the relevant API/type information. | Live behavior, application version compatibility, visual quality or successful production output.

| Live validated | A workflow was run in the actual Adobe app and its result checked. | Universal behavior beyond the tested version, document, platform and conditions.

Atlas percentages use that app’s documented feature count as the denominator. Tool counts represent MCP tools mapped in that dataset. Read the snapshot date and methodology before comparing releases.

## What the app servers work on

### InDesign

Documents and books, pages and spreads, text, tables, styles, color, images, long-document structure, interactive output, prepress and export.

### Illustrator

Documents, artboards, layers, vector shapes and paths, text, swatches, appearance, effects, symbols, data sets, images, actions and output.

### Photoshop

Documents and canvas, pixels, layers and masks, selections, channels, adjustments, filters, painting, retouching, type, actions and export.

### After Effects

Projects and footage, compositions, layers and time, keyframes, expressions, effects, text, masks, shapes, tracking, 3D and rendering.

### Premiere Pro

Projects and media, sequences, timeline edits, markers, effects, keyframes, color, audio, captions/transcripts, metadata and export.

### Suite coordinator

Two explicitly coordinated workflows today: place Illustrator artwork in InDesign and copy supported swatches from Illustrator to InDesign.

Use the [Capability Atlas](https://creativemcp.com/capability-atlas.html) to explore the detailed feature inventory and its sources. Use each [implementation guide](https://creativemcp.com/docs/) for tool behavior and package-specific constraints.

## Choose by content

- Read before write: request status, document/project information, list results and selected items before making edits.
- Stable target: use IDs returned by list/create tools where the app supports them; verify the selection and active document.
- App-native units: follow the specific app guide. InDesign documentation commonly specifies millimetres; Illustrator uses points from an artboard’s top-left with y increasing downward; Photoshop geometry is pixels from the document top-left; Premiere time values are seconds.
- Export is its own workflow: specify format, preset/options, path and whether overwrite is intended; confirm the generated file in the target app or viewer.
- Coverage is not endorsement: the Atlas may document features that are inaccessible through a script route or absent from our tools.
