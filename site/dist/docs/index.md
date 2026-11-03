---
title: "Documentation"
description: "Creative MCP documentation for Adobe MCP and Blender MCP: setup, content guides, MCP API, CLI, configuration and implementation references."
url: https://creativemcp.com/docs/
author: Zach Shallbetter (https://creativemcp.com/about.html)
modified: 2026-11-03T14:32:15-08:00
site: Creative MCP (https://creativemcp.com/, index: https://creativemcp.com/llms.txt)
copyright: © 2026 Zach Shallbetter (https://github.com/zachshallbetter/creative-mcp/blob/main/NOTICE.md)
---

Documentation · local servers, one guide per surface

# Find the right guide.

Setup and product concepts, the MCP tool interface, command-line launch and configuration, then detailed references for Adobe MCP and Blender MCP implementations.

PRODUCT GUIDES · PROTOCOL REFERENCE · IMPLEMENTATION MANUALS

## Start here

- [Guided setupChoose an implementation and client, then check local prerequisites.](https://creativemcp.com/start.html)
- [Getting startedRequirements, install, client setup and first local workflow.](https://creativemcp.com/docs/getting-started.html)
- [Content & capability guideWhat the tools work on, how to read coverage and evidence.](https://creativemcp.com/docs/content.html)
- [MCP API referenceInitialization, tools/list, tools/call, help and result semantics.](https://creativemcp.com/docs/api.html)
- [CLI & configurationLaunch commands, suite app selection and environment variables.](https://creativemcp.com/docs/cli.html)
- [Architecture & distributionHow the series is organized, the Homebrew tap, the creative-mcp installer and conventions for new tools.](https://creativemcp.com/docs/architecture.html)
- [Capability AtlasFeature-level data, automation routes, tool mapping and evidence.](https://creativemcp.com/capability-atlas.html)
- [Workflow gallerySupported cross-app actions and clearly labeled manual handoffs.](https://creativemcp.com/workflows.html)

## Implementation guides

Each MCP implementation has a dedicated guide for its content surface, application connection, API conventions and setup. These pages explain how the implementations differ; package READMEs and generated references remain the source for complete tool schemas and options.

- [InDesignLayout, documents, typography, tables, long documents and output.](https://creativemcp.com/docs/implementations/indesign.html)
- [IllustratorArtboards, vector artwork, paths, type, effects and export.](https://creativemcp.com/docs/implementations/illustrator.html)
- [PhotoshopPixels, documents, layers, selections, adjustments and output.](https://creativemcp.com/docs/implementations/photoshop.html)
- [After EffectsProjects, compositions, animation, effects and rendering.](https://creativemcp.com/docs/implementations/aftereffects.html)
- [Premiere ProProjects, sequences, timeline editing and UXP bridge setup.](https://creativemcp.com/docs/implementations/premiere.html)
- [Adobe MCP coordinatorCross-app workflows and child-server orchestration.](https://creativemcp.com/docs/implementations/suite.html)
- [Blender MCPRust server, Blender add-on, local IPC and 3D workflows.](https://creativemcp.com/docs/implementations/blender.html)
- [Shared coreCommon server infrastructure for contributors and integrators.](https://creativemcp.com/docs/implementations/core.html)

## Choose by task

I want to install

### Connect a client

Follow [MCP client setup](https://creativemcp.com/docs/getting-started.html#clients), then see [CLI and configuration](https://creativemcp.com/docs/cli.html) for launch options.

I want a tool schema

### Inspect the API

Use the server’s live MCP `tools/list` or `help` tool; open the matching [API guide](https://creativemcp.com/docs/api.html) for conventions and sources.

I want to see coverage

### Browse the Atlas

Explore documented features, automation routes, MCP mapping and validation status in the [Capability Atlas](https://creativemcp.com/capability-atlas.html).

I want to contribute

### Make a change

Read the [contribution guide](https://creativemcp.com/docs/contributing.html) for implementation boundaries and evidence expectations.
