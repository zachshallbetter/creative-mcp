---
title: "Contribute"
description: "Contribute an implementation or workflow to Creative MCP with accurate scope, setup, capability and evidence documentation."
url: https://creativemcp.com/docs/contributing.html
author: Zach Shallbetter (https://creativemcp.com/about.html)
modified: 2026-11-03T14:32:15-08:00
site: Creative MCP (https://creativemcp.com/, index: https://creativemcp.com/llms.txt)
copyright: © 2026 Zach Shallbetter (https://github.com/zachshallbetter/creative-mcp/blob/main/NOTICE.md)
---

[Docs](https://creativemcp.com/docs/) / Contribute

Contributing · implementation by implementation

# Make a useful tool easier to reach.

Creative MCP is a collection of open implementations for creative applications. A contribution should make one implementation or a documented workflow more useful while keeping its capabilities and limits clear.

[Docs home](https://creativemcp.com/docs/)[Getting started](https://creativemcp.com/docs/getting-started.html)[Content](https://creativemcp.com/docs/content.html)[MCP API](https://creativemcp.com/docs/api.html)[CLI & config](https://creativemcp.com/docs/cli.html)[Architecture](https://creativemcp.com/docs/architecture.html)[InDesign](https://creativemcp.com/docs/implementations/indesign.html)[Illustrator](https://creativemcp.com/docs/implementations/illustrator.html)[Photoshop](https://creativemcp.com/docs/implementations/photoshop.html)[After Effects](https://creativemcp.com/docs/implementations/aftereffects.html)[Premiere Pro](https://creativemcp.com/docs/implementations/premiere.html)[Suite](https://creativemcp.com/docs/implementations/suite.html)[Shared core](https://creativemcp.com/docs/implementations/core.html)[Blender MCP](https://creativemcp.com/docs/implementations/blender.html)[Contribute](https://creativemcp.com/docs/contributing.html)

Start with the project

## Choose the right home for the change.

### Adobe MCP

Adobe app servers and coordinator work live in the Adobe MCP repository. Follow that repository’s contributor instructions and app-specific requirements.
[Open Adobe MCP on GitHub ↗](https://github.com/zachshallbetter/adobe-mcp)

### Blender MCP

The Rust CLI, Blender companion add-on and Blender-specific documentation live in the Blender MCP repository. Follow its current build and contribution guidance.
[Open Blender MCP on GitHub ↗](https://github.com/zachshallbetter/blender-mcp-rs)

These implementations are maintained in separate repositories and have separate setup and capability boundaries. A shared website does not imply a shared runtime, API or automatic handoff.

Proposing a new tool

## Join the series with the same conventions.

A new implementation gets its own repository, published package, Homebrew formula in `zachshallbetter/tap`, implementation guide and, a catalogue entry with detection rules checked on a real machine. Names follow one pattern: `<id>-mcp` for the repository, formula and package, and `<id>-mcp-<app>` for per-app servers.
[Read the architecture & conventions →](https://creativemcp.com/docs/architecture.html#adding-a-tool)

Site, docs and catalogue changes go to the public [creative-mcp repository](https://github.com/zachshallbetter/creative-mcp); see its [contribution guide](https://github.com/zachshallbetter/creative-mcp/blob/main/CONTRIBUTING.md).

Make the contribution reviewable

## Show what changed and how you know.

### State the user task.

Name the application, the operation, the required input and the expected result. Keep the change scoped to the relevant implementation.

### Keep claims tied to evidence.

Distinguish documented, implemented, automated-check tested and validated in the real application. Link the source or run record behind each claim.

### Document setup and limits.

Record supported application and client versions, permissions, platform differences, failure cases and any manual handoff still required.

For a workflow example

## Describe the handoff honestly.

When an example spans implementations, list which server performs each step, what the person exports or imports, what the client coordinates, and which operations have actually been validated. Do not imply automatic file transfer or cross-server orchestration unless the code provides it.
[Review workflow examples](https://creativemcp.com/workflows.html)
