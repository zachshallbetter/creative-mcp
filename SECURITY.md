# Security Policy

## Reporting a vulnerability

Report it privately through GitHub: [report a vulnerability](https://github.com/zachshallbetter/creative-mcp/security/advisories/new).
Please don't open a public issue, pull request or discussion for it. Include the affected part (the site, the
catalogue or the installer), the installer version if relevant (`creative-mcp --version`), and steps to reproduce.

This project has a solo maintainer. Reports are handled on a best-effort basis, with no response-time guarantee (no SLA).

## Scope

This repository holds the creativemcp.com website, the series catalogue (`registry/`) and the `creative-mcp` installer
(`cli/`). Each tool in the series (adobe-mcp, blender-mcp-rs) has its own repository; report a problem in a tool's
code there.

In scope:

- The installer running or writing something you did not approve in its plan: it runs Homebrew, npm or Cargo
  installs and writes `mcpServers` entries into MCP clients' configuration files, backing each file up once before the
  first change.
- A catalogue entry that makes the installer fetch or run something other than the tool it names.
- The installer sending data anywhere; it has no telemetry, and its only network access is the package managers
  downloading the tools you chose.
- Script injection or other client-side problems on creativemcp.com, and secrets or personal data committed to the
  repository.

## Supported versions

The latest tagged installer release (tags are `cli-vX.Y.Z`) and the deployed site. Update to the latest release
before reporting.
