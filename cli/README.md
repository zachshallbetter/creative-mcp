# creative-mcp installer (Planned)

**Status: Planned. Nothing here works yet.**

`creative-mcp setup` will detect the creative apps and MCP clients on your machine, let you pick which Creative MCP tools to install, install them (Homebrew where available), register them with your clients and confirm each server answers MCP `initialize`. It reads the catalogue in [`../registry`](../registry).

The design, commands and detection rules are in [docs/ARCHITECTURE.md](../docs/ARCHITECTURE.md#4-planned-the-creative-mcp-installer). The planned language is Rust (decision D2).
