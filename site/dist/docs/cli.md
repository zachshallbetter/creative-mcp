---
title: "CLI & configuration"
description: "Install and launch commands, Suite coordinator app selection, source checkouts and configuration variables for the Adobe MCP servers."
url: https://creativemcp.com/docs/cli.html
author: Zach Shallbetter (https://creativemcp.com/about.html)
modified: 2026-11-03T14:32:15-08:00
site: Creative MCP (https://creativemcp.com/, index: https://creativemcp.com/llms.txt)
copyright: © 2026 Zach Shallbetter (https://github.com/zachshallbetter/creative-mcp/blob/main/NOTICE.md)
---

[Docs](https://creativemcp.com/docs/) / CLI & configuration

Launch commands · environment

# CLI & configuration

These packages are MCP server processes launched by your client. They are not general command-line utilities; only the Suite coordinator currently documents app selection flags.

[Docs home](https://creativemcp.com/docs/)[Getting started](https://creativemcp.com/docs/getting-started.html)[Content](https://creativemcp.com/docs/content.html)[MCP API](https://creativemcp.com/docs/api.html)[CLI & config](https://creativemcp.com/docs/cli.html)[Architecture](https://creativemcp.com/docs/architecture.html)[InDesign](https://creativemcp.com/docs/implementations/indesign.html)[Illustrator](https://creativemcp.com/docs/implementations/illustrator.html)[Photoshop](https://creativemcp.com/docs/implementations/photoshop.html)[After Effects](https://creativemcp.com/docs/implementations/aftereffects.html)[Premiere Pro](https://creativemcp.com/docs/implementations/premiere.html)[Suite](https://creativemcp.com/docs/implementations/suite.html)[Shared core](https://creativemcp.com/docs/implementations/core.html)[Blender MCP](https://creativemcp.com/docs/implementations/blender.html)[Contribute](https://creativemcp.com/docs/contributing.html)

## Install and launch

### One published app server

```
claude mcp add illustrator -- npx -y adobe-mcp-illustrator
```

Substitute the desired package name. For another MCP client, configure the command and arguments instead of running it as a one-shot shell command.

### Suite coordinator

```
claude mcp add adobe -- npx -y adobe-mcp-suite
```

Optional global install is documented for the suite package:

```
npm install -g adobe-mcp-suite
```

### Homebrew

The Creative MCP tap installs the Suite coordinator and links every app server, including the InDesign UXP bridge, onto your `PATH`. Homebrew 7 and later asks you to trust a third-party tap once.

```
brew tap zachshallbetter/tap
brew trust zachshallbetter/tap
brew install zachshallbetter/tap/adobe-mcp
```

Installed commands: `adobe-mcp-suite`, `adobe-mcp-indesign`, `adobe-mcp-indesign-uxp-bridge`, `adobe-mcp-illustrator`, `adobe-mcp-photoshop`, `adobe-mcp-premiere` and `adobe-mcp-aftereffects`. Point your client at the command name instead of `npx`.

## Suite app selection

The coordinator accepts `--apps ids`, `--apps=ids`, or `ADOBE_MCP_APPS`. If none is supplied, it loads InDesign and Illustrator, the current workflow dependencies. Use `all` to load all five app servers.

```
npx -y adobe-mcp-suite --apps indesign,illustrator
npx -y adobe-mcp-suite --apps=photoshop,aftereffects
ADOBE_MCP_APPS=premiere npx -y adobe-mcp-suite
npx -y adobe-mcp-suite --apps all
```

Valid IDs: `indesign`, `illustrator`, `photoshop`, `aftereffects`, `premiere`. Unknown IDs fail startup. Loading an app does not add coordinator workflows for it.

## Run from a source checkout

```
git clone https://github.com/zachshallbetter/adobe-mcp.git
cd adobe-mcp
npm install

# Suite coordinator
node packages/suite/src/index.js --apps indesign,illustrator

# Standalone MCP server
node packages/illustrator/src/index.js
```

These entrypoints stay open and speak MCP over stdio, so a client should launch them. The repository’s root scripts are for development checks, not a separate product CLI:

```
npm run check
npm test
```

## No generic CLI command surface

For the app servers, the documented interface is the MCP protocol, not shell flags like `--help`, `status` or `export`. To inspect tools, connect the server and use `tools/list` or the server’s MCP `help` tool. To smoke-check startup, use an MCP client or send MCP initialization and tool-list requests over stdio; tests use stubs and do not require Adobe apps.

## Configuration variables

| Variable | Scope | Use

| `ADOBE_MCP_APPS` | Suite | Comma-separated app IDs; suite-only.

| `*_ALLOWED_DIRS` | App-specific | Additional file directories. Use colon-separated paths on macOS and semicolons on Windows.

| `*_APP_NAME` | Where package supports it | Select or identify the local Adobe app; see that package guide.

| `*_SCRIPT_TIMEOUT_MS` | App-specific | Script/operation wait limit; defaults vary by package.

| `*_BACKEND`, `*_TARGET` | Supported packages only | Select documented backend or compatible local ArtCraft target.

| `PREMIERE_WS_PORT` | Premiere | Loopback bridge port; configure the same value in the plugin panel.

| `BRIDGE_TOKEN`, `PREMIERE_BRIDGE_TOKEN` | Premiere | Optional shared secret for local bridge authentication; keep private.

These patterns are not universal aliases: check the app README for exact accepted names, defaults, precedence and whether the Suite coordinator forwards them. Configure variables inside the MCP client’s server entry and restart the process after changes.
