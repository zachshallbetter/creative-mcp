---
title: "Architecture & distribution"
description: "How the Creative MCP series fits together: one repository per tool, the Homebrew tap, the creative-mcp installer, the shared catalogue and rules for new tools."
url: https://creativemcp.com/docs/architecture.html
author: Zach Shallbetter (https://creativemcp.com/about.html)
modified: 2026-11-03T18:21:39-08:00
site: Creative MCP (https://creativemcp.com/, index: https://creativemcp.com/llms.txt)
copyright: © 2026 Zach Shallbetter (https://github.com/zachshallbetter/creative-mcp/blob/main/NOTICE.md)
---

[Docs](https://creativemcp.com/docs/) / Architecture & distribution

How the series fits together

# Architecture & distribution

Creative MCP is a series of independent MCP implementations created by Zach Shallbetter. Each tool lives in its own repository and release cycle. They share one website, one Homebrew tap, one catalogue and one installer.

[Docs home](https://creativemcp.com/docs/)[Getting started](https://creativemcp.com/docs/getting-started.html)[Content](https://creativemcp.com/docs/content.html)[MCP API](https://creativemcp.com/docs/api.html)[CLI & config](https://creativemcp.com/docs/cli.html)[Architecture](https://creativemcp.com/docs/architecture.html)[InDesign](https://creativemcp.com/docs/implementations/indesign.html)[Illustrator](https://creativemcp.com/docs/implementations/illustrator.html)[Photoshop](https://creativemcp.com/docs/implementations/photoshop.html)[After Effects](https://creativemcp.com/docs/implementations/aftereffects.html)[Premiere Pro](https://creativemcp.com/docs/implementations/premiere.html)[Suite](https://creativemcp.com/docs/implementations/suite.html)[Shared core](https://creativemcp.com/docs/implementations/core.html)[Blender MCP](https://creativemcp.com/docs/implementations/blender.html)[Contribute](https://creativemcp.com/docs/contributing.html)

## The series today

Each implementation is a separate project with its own language, licence and release process. They share no runtime, API or process. A client can load several at once, but moving files between them is a handoff you manage.

| Project | Source | Language | Install | Licence

| [Adobe MCP](https://creativemcp.com/apps.html): five app servers, Suite coordinator, shared core | [adobe-mcp](https://github.com/zachshallbetter/adobe-mcp) | Node.js 18+ | Homebrew `zachshallbetter/tap/adobe-mcp`, npm `adobe-mcp-*` | MIT

| [Blender MCP](https://creativemcp.com/docs/implementations/blender.html): Rust server, CLI and Blender add-on | [blender-mcp-rs](https://github.com/zachshallbetter/blender-mcp-rs) | Rust | Homebrew `zachshallbetter/tap/blender-mcp` (v0.5.1), Cargo from source | MIT OR Apache-2.0

| Website, docs and series catalogue | [creative-mcp](https://github.com/zachshallbetter/creative-mcp) | HTML, Python, JSON | Served at creativemcp.com | MIT for code; content and brand reserved

| Homebrew tap | [homebrew-tap](https://github.com/zachshallbetter/homebrew-tap) | Ruby formulae | `brew tap zachshallbetter/tap` |

## How releases reach you

Every tool is published through its own ecosystem first (npm for Adobe MCP, Cargo for Blender MCP), then gets a formula in the Creative MCP Homebrew tap. Homebrew installs the runtime the tool needs, so you don't have to set up Node.js or Rust yourself.

```
brew tap zachshallbetter/tap
brew trust zachshallbetter/tap        # Homebrew 7+: trust a third-party tap once
brew install zachshallbetter/tap/adobe-mcp
brew install zachshallbetter/tap/blender-mcp
```

Each formula runs a real MCP `initialize` check as its test. Update with `brew upgrade`.

## One installer for the series

Status: Implemented and tested (v0.2.1). Fully supported on macOS. Linux detects apps only when their command is on `PATH`. Windows looks for the apps in their default install folders (not yet checked on a real Windows machine); anywhere, `creative-mcp add blender` sets up a tool without detection.

A browser can't see which apps you have installed, so the series has a small local installer that can.

```
brew tap zachshallbetter/tap
brew trust zachshallbetter/tap
brew install zachshallbetter/tap/creative-mcp
creative-mcp setup
```

`creative-mcp setup`:

- detects installed creative apps (on macOS by bundle ID, for example `com.adobe.Photoshop` and `org.blenderfoundation.blender`, with their versions) and MCP clients (their command, including the Claude Code CLI bundled with Claude Desktop, or their config file)
- suggests the tools that match your apps and asks before choosing any
- reads what each client already has registered and leaves current entries alone, so running it again changes nothing
- shows every command it will run and every config entry it will write, and asks again before changing anything
- installs with Homebrew, or with npm or Cargo when Homebrew isn't available
- registers servers with Claude Code, Codex and Gemini CLI through their own `mcp add` commands, and with Claude Desktop and Cursor by editing their JSON config. Each file is backed up once before the first change, and other settings are kept.
- only registers the servers your apps need. The Suite coordinator, for example, is offered only when both Illustrator and InDesign are installed.
- checks that each server answers MCP `initialize`, then tells you which clients to restart and gives a few requests to try

For everyday use it also has `doctor` (are the servers answering, are the running apps reachable, which clients have which servers, and the command that fixes each problem), `update`, `add <tool>`, `remove <tool>`, `detect` and `list`, plus `--dry-run`, `--yes`, `--tools`, `--clients` and `--json`. The full reference is in [CLI & configuration](https://creativemcp.com/docs/cli.html#installer). Detection runs only on your machine and sends nothing anywhere. Source: [creative-mcp/cli](https://github.com/zachshallbetter/creative-mcp/tree/main/cli).

## One catalogue for every tool

Status: Draft. The first version of the catalogue is published at [creativemcp.com/registry.json](https://creativemcp.com/registry.json), with a [JSON Schema](https://creativemcp.com/registry.schema.json), and validated on every change in the [creative-mcp repository](https://github.com/zachshallbetter/creative-mcp/tree/main/registry). The `creative-mcp` installer reads it. These pages are still written by hand.

The installer is built from this catalogue. Every other place that lists tools (these docs, the setup builder, [llms.txt](https://creativemcp.com/llms.txt) and the tap) will read it too. Each entry records the tool's repository, licence, author, capability status, install methods, server commands, the apps it targets and how to detect them, and the clients it supports. Adding a tool to the series then means adding one entry.

## Conventions for new tools

| Thing | Rule | Example

| Catalogue ID | Lowercase subject, one word | `adobe`, `blender`

| Repository | `<id>-mcp` | `adobe-mcp`

| Homebrew formula | `<id>-mcp` in `zachshallbetter/tap` | `blender-mcp`

| Packages and server commands | `<id>-mcp`, plus `<id>-mcp-<app>` for per-app servers | `adobe-mcp-photoshop`

| Client registration name | `<id>`, or `<app>` for a per-app server | `claude mcp add photoshop -- adobe-mcp-photoshop`

| Docs page | `/docs/implementations/<id>.html` | [Blender MCP](https://creativemcp.com/docs/implementations/blender.html)

| Capability status | Documented, Implemented, Tested or Validated | See [Content & capability guide](https://creativemcp.com/docs/content.html)

## Adding a tool to the series

- Create the repository with a README, licence files, contribution guide and tagged releases.
- Publish a package and add a formula to the tap with a working test.
- Add the entry to [`registry/registry.json`](https://github.com/zachshallbetter/creative-mcp/blob/main/registry/registry.json), with detection rules checked on a real machine (mark unchecked ones `"verified": false`).
- Add the implementation guide to these docs.
- Record capability status honestly: only Validated means the workflow ran in the real app and its result was checked.

See the [contribution guide](https://creativemcp.com/docs/contributing.html) for review expectations.
