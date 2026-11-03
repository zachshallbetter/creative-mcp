---
title: "How it works"
description: "How Creative MCP works: your MCP client starts a local server, the server talks to your creative app, and your files never leave your computer."
url: https://creativemcp.com/how-it-works.html
author: Zach Shallbetter (https://creativemcp.com/about.html)
modified: 2026-11-03T04:21:18-08:00
site: Creative MCP (https://creativemcp.com/, index: https://creativemcp.com/llms.txt)
copyright: © 2026 Zach Shallbetter (https://github.com/zachshallbetter/creative-mcp/blob/main/NOTICE.md)
---

Local setup · your tools stay yours

# Get from idea to output in three steps.

An MCP client starts a local server for the creative implementation you choose. That server connects to its supported application, so an agent can use the tools and context the implementation exposes.

## The three steps

### Choose an implementation

Start with Adobe MCP for Adobe apps or Blender MCP for 3D work. Check each project’s prerequisites and supported workflows.

### Connect your MCP client

Install the implementation and add its local launch command to a compatible client. Some projects also require an app-side companion add-on.

### Keep creating in your app

Open the creative application and let your client use the available tools against local work, files and context.

Choose your path

## Each project has its own setup.

### Adobe MCP

Install the Suite coordinator or an app-specific server. It runs with Node.js and connects to the Adobe desktop application on the same machine.

```
claude mcp add --scope user adobe -- npx -y adobe-mcp-suite
```

Or install every Adobe server with Homebrew:

```
brew tap zachshallbetter/tap
brew trust zachshallbetter/tap
brew install zachshallbetter/tap/adobe-mcp
```

[Adobe setup guide →](https://creativemcp.com/docs/getting-started.html)

### Blender MCP

Install the Rust CLI and its Blender companion add-on. The add-on must be enabled in Blender 4.0 or later.

```
cargo install --locked --git https://github.com/zachshallbetter/blender-mcp-rs --package blender-mcp-cli
```

Or let Homebrew build it for you:

```
brew tap zachshallbetter/tap
brew trust zachshallbetter/tap
brew install zachshallbetter/tap/blender-mcp
```

[Blender setup guide →](https://creativemcp.com/docs/implementations/blender.html)

One client · local implementations

## Follow the work from prompt to app.

The client communicates with each server over MCP. Implementations reach their target applications using the app-specific route documented by that project.

Your instructionsbriefs · files · context

MCP clientstarts local servers

Creative applicationsAdobe · Blender

Capability & evidence

## Know what a status means.

Documented

Project guides describe the behavior and requirements.

Implemented

The capability is present in that implementation’s tools.

Tested

Automated checks cover the code or tool behavior they target.

Validated

The workflow has been run in the actual application and its result checked. This state is tracked separately.

Adobe MCP publishes feature-level research and evidence in its [Capability Atlas](https://creativemcp.com/capability-atlas.html). Validation depends on the specific application and workflow; automated checks alone do not prove live-app behavior.

Project boundaries

## Separate tools. Clear handoffs.

Adobe MCP and Blender MCP are independent projects. A compatible client can expose both to the same workflow, but moving a file between them is currently a user-managed handoff rather than a direct cross-project tool.

[Explore projects →](https://creativemcp.com/projects.html) · [Browse workflow examples →](https://creativemcp.com/workflows.html) · [Browse documentation →](https://creativemcp.com/docs/)
