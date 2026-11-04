---
title: "Projects"
description: "Adobe MCP and Blender MCP compared: what each open MCP implementation connects to, what it needs and where to start. Both run locally on your machine."
url: https://creativemcp.com/projects.html
author: Zach Shallbetter (https://creativemcp.com/about.html)
modified: 2026-11-03T18:57:39-08:00
site: Creative MCP (https://creativemcp.com/, index: https://creativemcp.com/llms.txt)
copyright: © 2026 Zach Shallbetter (https://github.com/zachshallbetter/creative-mcp/blob/main/NOTICE.md)
---

Open implementations · local creative tools

# One protocol. Different creative worlds.

Creative MCP brings together independent implementations for the creative applications you already use. Each project has its own setup, requirements and capabilities.

## Implementations

### Adobe MCP

Local MCP servers for InDesign, Illustrator, Photoshop, After Effects and Premiere Pro, with a coordinator for two documented cross-app workflows.

- Five application-specific implementations
- One suite coordinator for two documented cross-app workflows
- Node.js 18+, macOS or Windows
- Install with `brew install zachshallbetter/tap/adobe-mcp` or npx
[Explore Adobe MCP →](https://creativemcp.com/apps.html)

### Blender MCP

A Rust MCP server and companion Blender add-on for scenes, objects, materials and 3D asset pipelines.

- Stdio MCP server with local Blender IPC
- Blender 4.0+ and the companion add-on
- Install with `brew install zachshallbetter/tap/blender-mcp` or Cargo from source
[Explore Blender MCP →](https://creativemcp.com/docs/implementations/blender.html)

These projects are separate MCP implementations. They can be configured in a compatible client at the same time, but direct Adobe-to-Blender orchestration is not currently part of either project.

Adobe MCP · five creative surfaces

## Choose a place to start.

Each Adobe server has its own tools and workflow. Read the implementation guide for app-specific requirements and behavior.

[Browse Adobe application implementations →](https://creativemcp.com/apps.html) · [Explore the Capability Atlas →](https://creativemcp.com/capability-atlas.html)

An open collection

## More creative tools can join as they’re ready.

New projects will be added as their implementations, documentation and evidence are ready to share. [Explore workflows across the current projects →](https://creativemcp.com/workflows.html)

## Ready to try it?

On a Mac, one installer finds your creative apps and MCP clients, shows its plan, then installs, registers and checks every server.

```
brew tap zachshallbetter/tap
brew trust zachshallbetter/tap
brew install zachshallbetter/tap/creative-mcp
creative-mcp setup
```

[Every setup option →](https://creativemcp.com/start.html)[What the installer does →](https://creativemcp.com/docs/cli.html#installer)
