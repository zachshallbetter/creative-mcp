# Maintaining Creative MCP

Notes for anyone maintaining this repository, people and coding agents alike. This repository is the hub of the Creative MCP series: the website (creativemcp.com), the series catalogue and the `creative-mcp` installer. Each tool's code lives in its own repository. Read [ARCHITECTURE.md](ARCHITECTURE.md) for how the pieces fit and [CONTRIBUTING.md](../CONTRIBUTING.md) for the ground rules. The essentials:

## Layout

| Path | What | Checks |
| --- | --- | --- |
| `site/dist/` | The static site, served as-is by Vercel (Root Directory `site`) | `python3 site/scripts/check-site.py` |
| `site/scripts/` | Generators that keep pages, social cards, JSON-LD and discovery files in sync | the `Site` workflow |
| `registry/` | `registry.json`, the catalogue every listing and the installer read | `python3 registry/validate.py` |
| `cli/` | The `creative-mcp` installer (Rust) | `cargo fmt --check && cargo clippy --all-targets -- -D warnings && cargo test` in `cli/` |
| `docs/` | Series architecture, analytics and DPP conformance notes | none |

The other repositories in the series, all under github.com/zachshallbetter:

- `adobe-mcp`: npm monorepo with the five Adobe app servers, the Suite and core. Its handbook is `docs/handbook/`.
- `blender-mcp-rs`: the Rust Blender server, CLI and add-on. Its agent notes are in its `AGENTS.md`.
- `homebrew-tap`: the formulae `creative-mcp`, `adobe-mcp` and `blender-mcp`.
- `adobe-capability-atlas` (private): the per-app feature corpus behind the Adobe coverage numbers.

On the owner's Mac these are cloned side by side under `~/Projects/creative-mcp/`, except `blender-mcp-rs` and `adobe-capability-atlas`, which are in `~/Projects/`. `~/Projects/creative-mcp/site` is a **retired** checkout of the private `creative-mcp-site-archive` repo; never edit or deploy from it. Worktrees go in `~/Projects/_worktrees/<repo>/<name>`. `AGENTS.md` and `CLAUDE.md` are in the owner's global gitignore, so durable notes for agents belong in this file, not in those.

## Changing the site

Run the generators from the repository root in this order. Each is idempotent and only touches what changed.

```sh
python3 site/scripts/generate-catalogue.py      # after editing registry/registry.json; CI fails if its output isn't committed
python3 site/scripts/generate-social-images.py  # after adding a page or changing a title (needs Chrome and sips; may hang on teardown, so kill it once it prints its summary)
python3 site/scripts/apply-shell.py             # head tags, JSON-LD nodes, docs nav and footer on every page
python3 site/scripts/generate-discovery.py      # sitemap.xml, llms.txt, llms-full.txt and the .md copy of each page
python3 site/scripts/check-site.py              # every sitemap page, card, JSON-LD block and discovery entry agree
```

- **`dateModified` and `lastmod` come from git and file times.** `generate-discovery.py` and `apply-shell.py` rewrite them on pages you didn't mean to touch. Revert that churn (`git checkout -- <file>`) before committing, and commit content changes before regenerating the sitemap.
- **Keep each JSON-LD block formatted** as `<script type="application/ld+json">`, a newline, the JSON, a newline, then `</script>`. `check-site.py` finds blocks by that shape and reports "no JSON-LD" if the newlines go.
- Every page has a `WebPage`-type node (`#webpage`) that `apply-shell.py` gives `primaryImageOfPage`, `copyrightHolder`, `copyrightYear` and `datePublished` from its `PUBLISHED` table. Add new pages there.
- `404.html`, `brand-study.html` and `style-mockup.html` are noindex and stay out of the sitemap.
- Preview with `.claude/launch.json` (`python3 -m http.server 4180 --directory site/dist`).
- Pushing to `main` deploys through Vercel. The `Site` workflow checks it, and the `IndexNow` workflow tells search engines which sitemap URLs changed (`site/scripts/indexnow.py`).

## Releasing the installer

1. Bump `cli/Cargo.toml`, update `docs/ARCHITECTURE.md` §4 and the version in the README if it's named there.
2. Tag `cli-vX.Y.Z` (annotated: `git tag -a cli-vX.Y.Z -m cli-vX.Y.Z`; tag signing is on) and push the tag.
3. In `homebrew-tap`, point `Formula/creative-mcp.rb` at the new tag's tarball and its sha256, then run `brew install --build-from-source` and `brew test` on it.

## Rules

- **Never commit personal data or secrets:** no IP addresses, no GA4 Measurement Protocol API secret (it is server-only; see [ANALYTICS.md](ANALYTICS.md)), no tokens, no personal contact details beyond what the About page already publishes.
- **Claims follow the status vocabulary:** Documented, Implemented, Tested, Validated. Only Validated means it ran in the real app. Planned work is labelled Planned.
- **Pages keep WCAG 2.2 AA:** contrast, target size, keyboard access and heading order.
- **No AI attribution lines** in commit messages or pull requests.
- Commit and push only when the owner asks. Posting to social accounts, Search Console, GA4 settings and npm or Homebrew account actions are the owner's.

## Where the work is tracked

Open work lives in GitHub issues, not in chat history:

- Here: installer (Windows detection #5, the Premiere UXP plugin without the UXP Developer Tool #17), catalogue detection rules (#4), and site follow-ups (GA4 key events #1, post-launch reviews #2, share previews #3, VoiceOver pass #12).
- adobe-mcp: the 0.3.1 release blocked on npm trusted publishing (#433), the Premiere plugin installer draft (#434), the Photoshop round 2 tickets (epic #407).

[ARCHITECTURE.md §7](ARCHITECTURE.md#7-tracked-work) keeps the cross-repo list.
