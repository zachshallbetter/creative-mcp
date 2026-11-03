---
title: "MCP API reference"
description: "How the Adobe MCP servers expose tools over stdio: discovery, tool metadata, calls, results, execution boundaries and where to find exact parameters."
url: https://creativemcp.com/docs/api.html
author: Zach Shallbetter (https://creativemcp.com/about.html)
modified: 2026-11-03T14:32:15-08:00
site: Creative MCP (https://creativemcp.com/, index: https://creativemcp.com/llms.txt)
copyright: © 2026 Zach Shallbetter (https://github.com/zachshallbetter/creative-mcp/blob/main/NOTICE.md)
---

[Docs](https://creativemcp.com/docs/) / MCP API reference

Protocol surface · tool schemas

# MCP API reference

The Adobe servers expose tools through MCP over stdio. Use the server’s advertised schema as the executable contract; this page explains discovery, calls, instructions, results and where to find exact per-tool parameters.

[Docs home](https://creativemcp.com/docs/)[Getting started](https://creativemcp.com/docs/getting-started.html)[Content](https://creativemcp.com/docs/content.html)[MCP API](https://creativemcp.com/docs/api.html)[CLI & config](https://creativemcp.com/docs/cli.html)[Architecture](https://creativemcp.com/docs/architecture.html)[InDesign](https://creativemcp.com/docs/implementations/indesign.html)[Illustrator](https://creativemcp.com/docs/implementations/illustrator.html)[Photoshop](https://creativemcp.com/docs/implementations/photoshop.html)[After Effects](https://creativemcp.com/docs/implementations/aftereffects.html)[Premiere Pro](https://creativemcp.com/docs/implementations/premiere.html)[Suite](https://creativemcp.com/docs/implementations/suite.html)[Shared core](https://creativemcp.com/docs/implementations/core.html)[Blender MCP](https://creativemcp.com/docs/implementations/blender.html)[Contribute](https://creativemcp.com/docs/contributing.html)

## Transport and lifecycle

The MCP client launches the configured command as a local process and communicates over standard input/output. The process stdout is protocol traffic; startup diagnostics belong on stderr. There is no public HTTP API for the desktop servers.

- The client starts the server and negotiates MCP initialization.
- The server returns its identity, capabilities and app-specific usage instructions.
- The client discovers tool names and JSON input schemas with `tools/list`.
- The client invokes a tool with `tools/call` and arguments that match the advertised schema.
- The server validates inputs, performs the app operation, then returns structured text/content or an error result.

## Discover the live API

Do not treat a marketing summary, Atlas row or old copy of a README as the current schema. The connected server’s `tools/list` is authoritative for the tool names and inputs exposed by that running package.

Discovery

### `tools/list`

Returns advertised tools and their JSON input schemas. Reconnect after a package upgrade so the client refreshes its list.

Detailed guidance

### `help`

App servers with the help utility can describe a tool, list categories, and provide detailed usage/examples. Consult the app-specific guide for its exact formats.

Execution

### `tools/call`

Calls one tool using the exact tool name and schema-defined argument object. Respect enum, unit, target and path constraints.

```
Client action (conceptual):
discover tools → choose an advertised name
call tool with arguments matching that tool's input schema
inspect isError and returned content before continuing
```

Use the package tool references for full parameter tables: [InDesign generated tool reference](https://github.com/zachshallbetter/adobe-mcp/tree/main/packages/indesign/docs), and the [Illustrator](https://github.com/zachshallbetter/adobe-mcp/tree/main/packages/illustrator), [Photoshop](https://github.com/zachshallbetter/adobe-mcp/tree/main/packages/photoshop), [After Effects](https://github.com/zachshallbetter/adobe-mcp/tree/main/packages/aftereffects), [Premiere](https://github.com/zachshallbetter/adobe-mcp/tree/main/packages/premiere) and [Suite](https://github.com/zachshallbetter/adobe-mcp/tree/main/packages/suite) READMEs.

## Tool metadata and results

- Input schema: use the exact property names, required fields, types, ranges and enum values returned by the server.
- Instructions: server instructions explain app conventions such as units, indices, colors, path rules, errors and safe targeting. MCP clients may pass them to the model as connection context.
- Annotations: supported tools may identify read-only, destructive, idempotent and open-world behavior. Treat annotations as useful metadata, not a replacement for checking the target.
- Errors: a failed tool may return `isError: true` with a message. Stop and interpret that result; do not assume the operation partially or fully succeeded.
- Stateful application: active document, selection, current sequence and UI state can change between calls. Re-read state when later steps depend on it.

Suite API is intentionally narrow. The coordinator exposes its cross-app workflows; it does not proxy every child server’s tools. Connect an app package separately for direct application tools.

## Execution boundaries

Most app tools generate a script for ExtendScript or UXP and send it to a local Adobe app or plugin. Premiere Pro uses its UXP Bridge panel. The tool call is local computer automation with the permissions of the configured process; it is not a hosted service request. Review app-specific notes about unrestricted code-execution tools before using them.
