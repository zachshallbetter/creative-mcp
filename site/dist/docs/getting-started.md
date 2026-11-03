---
title: "Getting Started"
description: "Setup, configuration, workflows, troubleshooting and safety guidance for Creative MCP implementations, including Adobe MCP and Blender MCP."
url: https://creativemcp.com/docs/getting-started.html
author: Zach Shallbetter (https://creativemcp.com/about.html)
modified: 2026-11-03T14:32:15-08:00
site: Creative MCP (https://creativemcp.com/, index: https://creativemcp.com/llms.txt)
copyright: © 2026 Zach Shallbetter (https://github.com/zachshallbetter/creative-mcp/blob/main/NOTICE.md)
---

[Docs](https://creativemcp.com/docs/) / Getting started

Field guide · setup to first workflow

# Get set up.

Install the local servers, connect them to your MCP client, understand which Adobe app each package reaches, and troubleshoot the bridge between your model and your creative work.

LOCAL PROCESS · MCP OVER STDIO · macOS + WINDOWS

On this page [Docs home](https://creativemcp.com/docs/)[Content guide](https://creativemcp.com/docs/content.html)[MCP API](https://creativemcp.com/docs/api.html)[CLI & config](https://creativemcp.com/docs/cli.html)[Architecture](https://creativemcp.com/docs/architecture.html)How it worksChoose a packageRequirements Quick startMCP client setupSuite coordinator App setupFirst workflowsConfiguration Files & safetyTroubleshootingReference library

## How it works

These are desktop automation servers. Your MCP client starts a Node.js process on the same computer as the Adobe app. The server accepts MCP messages over standard input/output (stdio), translates a tool call into an application script or bridge request, and returns the result to the client.

01 · Ask

### Your MCP client

Claude Code, Claude Desktop, Cursor, Warp, or another client that can launch an MCP server process.

02 · Route

### Local MCP server

Node runs on your machine. The server validates arguments and routes the request to the selected application backend.

03 · Act

### Adobe desktop app

The app must be installed and generally open (or launchable). Your client and Adobe files stay in your local workflow.

Not a hosted Adobe endpoint. The public website provides documentation and the capability atlas. Tool execution happens locally through the server process you configure in your MCP client; this website does not proxy MCP calls.

## Choose a package

Install one app server for direct control of that application. Add the Suite coordinator when you want its current cross-app workflows. The coordinator does not expose the full tool list of its child servers.

| Package | Use it for | Connection route | Install

| InDesign
`adobe-mcp-indesign` | Documents, pages, typography, tables, long documents and export. | ExtendScript via AppleScript on macOS or COM/PowerShell on Windows; optional UXP backend. | `npx -y adobe-mcp-indesign`

| Illustrator
`adobe-mcp-illustrator` | Documents, artboards, layers, vectors, typography, effects and export. | ExtendScript via AppleScript on macOS or COM/PowerShell on Windows. | `npx -y adobe-mcp-illustrator`

| Photoshop
`adobe-mcp-photoshop` | Documents, pixels, layers, selections, adjustments, painting and export. | ExtendScript through local desktop automation. | `npx -y adobe-mcp-photoshop`

| After Effects
`adobe-mcp-aftereffects` | Projects, footage, compositions, layers, keyframes, effects and render queue. | ExtendScript through local desktop automation. | `npx -y adobe-mcp-aftereffects`

| Premiere Pro
`adobe-mcp-premiere` | Projects, sequences, timeline edits, markers, effects and export. | Local WebSocket bridge to a UXP plugin panel in Premiere Pro. | `npx -y adobe-mcp-premiere`

| Suite coordinator
`adobe-mcp-suite` | Two coordinated Illustrator → InDesign handoffs. | Starts child MCP servers locally and delegates workflow steps. | `npx -y adobe-mcp-suite`

Tool counts, exact coverage, evidence levels and known gaps are in the [Capability Atlas](https://creativemcp.com/capability-atlas.html). The individual package guides document each package's current tool surfaces and options.

## Requirements

- Node.js 18 or later. Use an up-to-date LTS release; `npx` ships with npm.
- macOS or Windows for the listed desktop server integrations. macOS may prompt you to authorize Terminal or Node to control an Adobe app in System Settings → Privacy & Security → Automation.
- The corresponding Adobe desktop app installed on the same computer. InDesign, Illustrator, Photoshop and After Effects guides specify 2023+; Premiere Pro requires 25.6+ and its MCP Bridge UXP plugin loaded.
- An MCP client that can launch a command with arguments and environment variables and communicate over stdio.
- Premiere only: UXP Developer Tool to load the plugin until a published installation path is documented; plugin/server ports and bridge token must match.

Adobe app versions and backend support can differ by operation. Consult the app guide and the tool's `help` response for version-specific behavior.

## Quick start

The fastest path on a Mac: the `creative-mcp` installer finds your Adobe apps, Blender and MCP clients, and sets everything up after showing you the plan.

```
brew tap zachshallbetter/tap
brew trust zachshallbetter/tap
brew install zachshallbetter/tap/creative-mcp
creative-mcp setup
```

To set things up by hand instead:

- Install Node.js 18 or later and the Adobe application you intend to automate.
- Register the matching package in your MCP client using the setup below.
- Restart or reconnect the MCP client so it launches the process and refreshes its tool list.
- Open the Adobe app and a suitable document (or ask a read-only status tool what is open).
- Start with a small, reversible request. Confirm the target document, selection, units and output path before asking for a change or export.

For example: “In the active Illustrator document, list the artboards and report their sizes. Don’t make changes.” Once the connection is confirmed, try a bounded edit in a copy of a document.

## MCP client setup

### Claude Code: one app server

```
claude mcp add --scope user illustrator -- npx -y adobe-mcp-illustrator
```

Replace `illustrator` and the package name with the Adobe application you want. Omit `--scope user` if you prefer the client’s default scope.

### Claude Code: the Suite coordinator

```
claude mcp add --scope user adobe -- npx -y adobe-mcp-suite
```

### Any client with an `mcpServers` configuration

```
{
"mcpServers": {
"illustrator": {
"command": "npx",
"args": ["-y", "adobe-mcp-illustrator"]
}
}
}
```

For Claude Desktop, edit its MCP server configuration file. Cursor, Warp and other clients use their own configuration screen or file; follow the client’s current instructions for where to place this block. Do not paste the JSON into a shell command. See the dedicated [CLI and configuration reference](https://creativemcp.com/docs/cli.html) for source-checkout launch and environment-variable details.

### Run a specific published version

```
{
"mcpServers": {
"illustrator": {
"command": "npx",
"args": ["-y", "adobe-mcp-illustrator@0.9.0"]
}
}
}
```

Pin the package version when you need repeatable team setups; check npm for the current published version before updating the pin.

### Install with Homebrew

On macOS or Linux, the Creative MCP tap installs the Suite coordinator and every app server onto your `PATH`, so clients can launch them without `npx`. Homebrew 7 and later asks you to trust a third-party tap once.

```
brew tap zachshallbetter/tap
brew trust zachshallbetter/tap
brew install zachshallbetter/tap/adobe-mcp
claude mcp add adobe -- adobe-mcp-suite
claude mcp add illustrator -- adobe-mcp-illustrator
```

## Suite coordinator

The coordinator currently offers two workflows across InDesign and Illustrator. It starts the app servers it needs as child MCP processes, then delegates the individual actions. Install an app's standalone package as well if you need its direct tools in your client.

Illustrator → InDesign

### Place artwork

Place an artboard from the active Illustrator document on an InDesign page as a link to its `.ai` source. The Illustrator document is saved first by default.

Illustrator → InDesign

### Copy swatches

Copy supported process and spot swatches with exact values. Gradients and patterns are reported, not copied; built-in InDesign swatches are preserved.

### Select which child servers load

```
npx -y adobe-mcp-suite --apps indesign,illustrator
# or set ADOBE_MCP_APPS=indesign,illustrator
```

The supported app IDs are `indesign`, `illustrator`, `photoshop`, `aftereffects` and `premiere`. The current coordinator workflows require both InDesign and Illustrator. Selecting other apps does not add new cross-app workflows or re-export their tools.

### Allow the files needed by a workflow

File operations are confined to the user’s home directory and any additional per-app allowed directories. For a handoff, configure both participating servers for the directory containing the source artwork and the destination document as needed.

```
{
"mcpServers": {
"adobe": {
"command": "npx",
"args": ["-y", "adobe-mcp-suite", "--apps", "indesign,illustrator"],
"env": {
"INDESIGN_ALLOWED_DIRS": "/Users/you/Projects",
"ILLUSTRATOR_ALLOWED_DIRS": "/Users/you/Projects"
}
}
}
}
```

On Windows, use a Windows path and separate multiple allowed directories with semicolons. The coordinator forwards each server only the variables it needs.

## Application setup

Each app server has its own behavior, terminology, tool names and supported operations. Its MCP `help` tool describes parameters and categories from the definitions the server advertises.

Desktop automation

### InDesign

macOS uses AppleScript; Windows uses COM via PowerShell. Optional UXP bridge support is available. The guide explains page units, document selection, path permissions and backend selection.

[Read the InDesign guide ↗](https://creativemcp.com/docs/implementations/indesign.html)

Desktop automation

### Illustrator

Learn artboard-relative geometry, coordinate direction, item targeting, color formats, fonts and allowed paths before editing artwork.

[Read the Illustrator guide ↗](https://creativemcp.com/docs/implementations/illustrator.html)

Desktop automation

### Photoshop

Review layer and selection identifiers, image dimensions, file operations, export options and its local target/backend settings.

[Read the Photoshop guide ↗](https://creativemcp.com/docs/implementations/photoshop.html)

Desktop automation

### After Effects

Review composition and layer targeting, time/keyframe conventions, render queue behavior, file access and script timeout settings.

[Read the After Effects guide ↗](https://creativemcp.com/docs/implementations/aftereffects.html)

UXP plugin bridge

### Premiere Pro

Load the MCP Bridge panel, match its port and optional token to the server, and understand its tested status and API-version limits.

[Read the Premiere Pro guide ↗](https://creativemcp.com/docs/implementations/premiere.html)

Premiere Pro validation status: the repository guide says the server has not yet been tested against a real Premiere Pro session. Script checks and a stand-in test DOM do not establish live-app validation. See the guide’s “What is not certain yet” section.

## Plan a first workflow

- Inspect before editing. Ask for app status, document/project information, selections and relevant items first.
- Specify the target. Name the active document or use stable IDs returned by list tools. Avoid relying on a repeated display name when IDs are available.
- State units and bounds. Provide page/artboard dimensions, pixel sizes, frame/time units and a bounded set of objects where the tool requires them.
- Choose an output path. Use a path in your home folder or explicitly allowed directory. Do not overwrite an existing deliverable unless you intend to.
- Review before destructive work. Duplicate the document or project, check the intended selection, and ask the client to confirm before deleting, closing without saving, or replacing output.
- Verify in the Adobe app. Open or inspect the result, check links/fonts/render status as appropriate, and save/export deliberately.

Tool calls are discrete operations. A sequence of calls is not automatically an all-or-nothing transaction; inspect intermediate state when the workflow depends on a prior change.

## Configuration

Set environment variables in the MCP client’s server entry. Do not set them in a website form or commit real credentials to a repository.

| Variable family | Applies to | Purpose

| `*_ALLOWED_DIRS` | Each app server, e.g. `PHOTOSHOP_ALLOWED_DIRS` | Add directories to the default home-folder file access scope. Delimiter is platform-specific: `:` on macOS, `;` on Windows.

| `ADOBE_MCP_APPS` | Suite coordinator | Choose which app server IDs the coordinator starts.

| `*_APP_NAME` | App-specific, where supported | Select a local Adobe application installation. Availability and behavior vary by package and operating system.

| `*_SCRIPT_TIMEOUT_MS` | App-specific | Adjust how long a server waits for a script or bridge response. See the package guide for defaults and exceptions.

| `*_TARGET`, `ADOBE_TARGET` | Supported ArtCraft-compatible servers | Route compatible operations to a named local ArtCraft adapter. Support is app-specific; consult that server’s guide.

| `PREMIERE_WS_PORT` | Premiere Pro | Choose the loopback port used by the local server and set the same port in the UXP bridge panel.

| `BRIDGE_TOKEN`, `PREMIERE_BRIDGE_TOKEN` | Premiere Pro | Optional local bridge authentication; use the per-server override when coordinating with another bridge consumer. Keep tokens private.

| `UXP_BRIDGE_URL` | Packages with a UXP backend | Set the local bridge address only where the app guide documents that backend.

Variable names do not imply every package supports every option. Confirm the exact variable, default, delimiter and forwarding behavior in the package README.

## Files, permissions & safety

- Local process, local authority. MCP tools can make changes in the desktop application under your user session. Grant only the client permissions needed for the apps you use.
- File paths are constrained. Shared path policy confines supported file operations to the home directory and configured allowed directories, and rejects system-directory paths. An app may still interpret its own document references according to its own behavior.
- Destructive operations need review. Deleting pages/layers/clips, closing unsaved work, replacing files and similar actions can lose work. Tool annotations help capable clients recognize destructive or open-world calls; review each target and result.
- Use copies for consequential edits. Save a working copy, inspect the active document/selection and verify output before relying on an automated result.
- Premiere bridge stays local. Its documented server binds to loopback. A bridge token is stored by the plugin panel in local storage; protect access to the machine and do not share the token.
- Custom code is powerful. App-specific execution tools that accept code are not equivalent to ordinary constrained editing tools. Read their package security notes before enabling or using them.

## Troubleshooting

### The server does not appear in the client

Check the client’s MCP configuration syntax, command name and package spelling. Confirm Node/npm are on the client process PATH, then restart or reconnect the client and inspect its MCP logs.

### The client cannot start `npx`

Run `node --version` and `npm --version` in a terminal. Install Node.js 18+ and ensure the client app inherits a PATH that includes Node and npm.

### The server says no document is open

Open the target Adobe app and a document/project. Ask for the app’s status or list tools first, then identify the intended active document.

### macOS asks for Automation permission

Approve the prompt for the process actually controlling the Adobe app (often Terminal, the MCP client, or Node). Review it under System Settings → Privacy & Security → Automation.

### A file path is rejected

Use an absolute path inside your home directory or add its parent directory to that server’s `*_ALLOWED_DIRS`. Restart the MCP server after changing client environment settings.

### Premiere says the bridge is disconnected

Confirm Premiere Pro 25.6+, load the UXP plugin and open its MCP Bridge panel. Match its port and token to the server configuration; then run `premiere_status`.

### A call times out

Check for a modal dialog in the Adobe app and verify the app is responsive. Some renders and long operations need a longer package-specific timeout; do not blindly retry an operation that may have completed.

### A tool or parameter is unfamiliar

Call `help` for a tool or category. Use the exact advertised argument names and inspect current document state rather than guessing IDs or enum values.

### A cross-app workflow is missing

The coordinator currently advertises its supported InDesign/Illustrator workflows only when both child servers are loaded. It does not expose standalone app tools.

## Reference library

Use the maintained package guides as the source of truth for current tools, configuration, platform behavior and limitations. The site explains the product and common setup; it does not replace the detailed references.

- [Repository overviewPackages, install options, client config and development.](https://github.com/zachshallbetter/adobe-mcp/blob/main/README.md)
- [Suite coordinatorCross-app workflows and delegation behavior.](https://github.com/zachshallbetter/adobe-mcp/tree/main/packages/suite)
- [InDesign serverTool guide, platform backends, conventions and security.](https://github.com/zachshallbetter/adobe-mcp/tree/main/packages/indesign)
- [Illustrator serverArtboards, coordinates, targeting, colors and tools.](https://github.com/zachshallbetter/adobe-mcp/tree/main/packages/illustrator)
- [Photoshop serverDocument, layer, image, export and configuration reference.](https://github.com/zachshallbetter/adobe-mcp/tree/main/packages/photoshop)
- [After Effects serverProjects, compositions, timing, scripting and rendering.](https://github.com/zachshallbetter/adobe-mcp/tree/main/packages/aftereffects)
- [Premiere Pro serverUXP setup, bridge security, API limits and validation status.](https://github.com/zachshallbetter/adobe-mcp/tree/main/packages/premiere)
- [Security policyReport security issues through the repository’s stated process.](https://github.com/zachshallbetter/adobe-mcp/blob/main/SECURITY.md)
- [ContributingDevelopment and project contribution guidance.](https://github.com/zachshallbetter/adobe-mcp/blob/main/CONTRIBUTING.md)
- [Capability AtlasDetailed feature coverage, routes, evidence and workflows.](https://creativemcp.com/capability-atlas.html)
- [Suite on npmPublished package versions and release information.](https://www.npmjs.com/package/adobe-mcp-suite)
- [GitHub repositorySource, releases and issue tracker.](https://github.com/zachshallbetter/adobe-mcp)
