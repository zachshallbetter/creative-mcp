---
title: "Creative MCP — creative tools, connected"
description: "Open, local MCP servers that let AI agents work in Adobe InDesign, Illustrator, Photoshop, After Effects, Premiere Pro and Blender. Install with Homebrew."
url: https://creativemcp.com/
author: Zach Shallbetter (https://creativemcp.com/about.html)
modified: 2026-11-03T04:34:20-08:00
site: Creative MCP (https://creativemcp.com/, index: https://creativemcp.com/llms.txt)
copyright: © 2026 Zach Shallbetter (https://github.com/zachshallbetter/creative-mcp/blob/main/NOTICE.md)
---

Creative tools, connected

# Make more with the tools you already use.

Local MCP servers connect your creative applications to agent workflows, so your tools, files and context can work together.

[Get started](https://creativemcp.com/start.html)[Explore implementations](https://creativemcp.com/projects.html)

LOCAL · OPEN · CREATIVE · ON YOUR TERMS

Your creative apps

Open source

Local MCP · your machine

Total control

Creative workspaceConnected locally

Your MCP clientone connection · app-aware tools

InDesign layout

Illustrator vector

Photoshop images

After Effects motion

Premiere Pro video

Blender scenes

Adobe MCP + Blender MCP → your open creative apps

Implementations

## One protocol. Different creative worlds.

Creative MCP hosts independent, open implementations that connect creative tools to agent workflows.

[### Adobe MCP

Connect your Adobe creative apps.
Available

Five app-specific MCP servers, with a coordinator for supported cross-app workflows.
Explore Adobe MCP ↗](https://creativemcp.com/apps.html) [### Blender MCP

Work with 3D scenes, objects, materials and animation.
Available

A Rust MCP server and Blender companion add-on for scene tools, assets and pipelines.
Explore Blender MCP ↗](https://creativemcp.com/docs/implementations/blender.html)

＋ More creative tools can join as they’re ready.

Creative MCP is an open platform; implementations are developed and released independently.

How it works

## Get from idea to output in three steps.

A simple local setup connects your creative applications to your chosen agent workflow.

### Choose a tool

Pick an implementation, such as Adobe MCP or Blender MCP.

### Connect your MCP client

Install and configure the local server and any app companion it needs.

### Keep creating in your app

Your tools stay open on your machine. Work with your files and context where supported.

[See setup details and implementation requirements →](https://creativemcp.com/how-it-works.html)

Built for real work

## Move from instructions to finished creative output.

Give agents a way to work with your actual creative tools, files and context. Each implementation runs locally and documents its own setup.

Everything · creative-mcp installer

```
brew tap zachshallbetter/tap
brew trust zachshallbetter/tap
brew install zachshallbetter/tap/creative-mcp
creative-mcp setup
```

Adobe MCP · Homebrew

```
brew tap zachshallbetter/tap
brew trust zachshallbetter/tap
brew install zachshallbetter/tap/adobe-mcp
```

Blender MCP · Homebrew

```
brew tap zachshallbetter/tap
brew trust zachshallbetter/tap
brew install zachshallbetter/tap/blender-mcp
```

Adobe MCP · Claude Code with npx

```
claude mcp add --scope user adobe -- npx -y adobe-mcp-suite
```

Blender MCP · Rust install

```
cargo install --locked --git https://github.com/zachshallbetter/blender-mcp-rs --package blender-mcp-cli
```

Your instructionsbriefs · files · context

MCP clientlocal connection

Creative outputAdobe apps · Blender

Capability & evidence

## Clear signals for what’s ready.

Each implementation provides transparent information about its current capabilities, so you know what to expect.

Documented

Guides explain what the implementation does and how to set it up.

Implemented

The tools and capabilities are present in the MCP server.

Tested

Automated checks cover the tool implementation where provided.

Validated

Live-app validation is reported separately and depends on the implementation.

Adobe MCP’s [Capability Atlas](https://creativemcp.com/capability-atlas.html) tracks evidence by feature. For Blender MCP, see the project’s current repository documentation. A tool passing automated checks does not by itself prove that a workflow has been validated in a live application.

Beyond one app

## From layout to a 3D scene.

Move approved artwork and its context into Blender, then keep building in the tools you already use.

01 / 03

### Design the composition

Shape a layout and approve the artwork in InDesign.

02 / 03

### Move the working files

Export the approved assets and carry the brief into the next tool.

03 / 03

### Develop the scene

Bring those assets into Blender and continue building in 3D.

How the handoff worksThis is a file handoff between separate MCP implementations. Direct Adobe-to-Blender orchestration is not currently provided. [Explore the workflow gallery →](https://creativemcp.com/workflows.html)

Get started today

## Give your creative tools a way to work together.

Local, open and built for creators.

[Get started ↗](https://creativemcp.com/start.html)[Browse workflows →](https://creativemcp.com/workflows.html)
