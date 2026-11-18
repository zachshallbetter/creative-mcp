# Contributing to Creative MCP

Thanks for helping. This repository holds the website, docs and catalogue for the series. Changes to a tool's code belong in that tool's own repository:
[adobe-mcp](https://github.com/zachshallbetter/adobe-mcp) · [blender-mcp-rs](https://github.com/zachshallbetter/blender-mcp-rs) · [homebrew-tap](https://github.com/zachshallbetter/homebrew-tap).

Maintainers and coding agents: [docs/MAINTAINING.md](docs/MAINTAINING.md) has the site pipeline, release steps and where open work is tracked.

## Good changes here

- Fixing or clarifying documentation in `site/dist/docs/`.
- Updating a catalogue entry in `registry/registry.json`, for example a detection rule you checked on a real machine.
- Proposing a new tool for the series (open an issue first).

## Ground rules

- **Keep claims tied to evidence.** Use Documented, Implemented, Tested or Validated exactly. Only Validated means the workflow ran in the real application and its result was checked.
- **Label future work as Planned.** Don't describe it as available.
- **Detection rules:** set `"verified": true` only when you checked the bundle ID, path or registry key on a machine with the app installed, and say which app version in the pull request.
- **Accessibility:** pages must keep passing WCAG 2.2 AA (contrast, target size, keyboard access, heading order).
- **No personal data:** never commit IP addresses, API secrets, tokens or personal contact details.

## Checks to run

```sh
python3 registry/validate.py                    # catalogue schema and cross-references (needs `pip install jsonschema`)
python3 site/scripts/generate-catalogue.py      # after editing registry/registry.json; CI fails if its output isn't committed
python3 site/scripts/generate-discovery.py      # after committing content changes
python3 site/scripts/generate-social-images.py  # when a page is added or its title changes
python3 site/scripts/check-site.py              # every page, card, JSON-LD block and discovery entry agree
```

For changes to the installer in `cli/`, run these from `cli/`, as CI does:

```sh
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
```

The site is plain HTML, CSS and JavaScript with no build step; the [README](README.md#working-on-the-site) shows how to
preview it.

## Commits and pull requests

Commit titles are short descriptive sentences; a `ci:` or `ci(scope):` prefix marks workflow changes. Commit content
changes before regenerating the sitemap, and revert generator churn on pages you didn't mean to touch. Pull requests
are merged by the maintainer; the pull request template asks what you ran.

## Adding a tool to the series

Follow the checklist in [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md#6-adding-a-tool-to-the-series): own repository and releases, a package and Homebrew formula, a catalogue entry, an implementation guide and an honest capability status.

## Security

Report vulnerabilities privately, as [SECURITY.md](SECURITY.md) describes, not in a public issue.
