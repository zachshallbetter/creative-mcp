# Digital Presence Protocol: site conformance record

Protocol: Digital Presence Protocol 0.3.0 (`_protocols/digital-presence-protocol`), modules P01–P07, DPP-PRP and DPP-DAP.
Entity: Creative MCP (`https://creativemcp.com/#organization`), with Adobe MCP and Blender MCP as software entities and Zach Shallbetter as creator.
Surface scope: the static site in `site/dist/` as served from creativemcp.com. Out of scope: GitHub, npm, social profiles, third-party indexes.
Evaluation: 2026-11-03, local preview plus the live site's response headers. Evaluator: Claude Code, at the owner's request.

Level reached: **Implemented and mechanically tested** for the items marked SATISFIED. Nothing here is **empirically validated**: indexing, rich results, AI citations and reach have not been observed (DPP-DAP gate, P07 state machine). Third-party index and AI retrieval status is GRAY until observed.

Verdicts use DPP terms: SATISFIED, VIOLATED, GRAY (unresolved evidence), NOT_APPLICABLE.

## P01: crawl, render and index

| Gate | Verdict | Evidence |
| --- | --- | --- |
| T1 HTTP, TLS and redirects | SATISFIED | `http://` and `www.` return 308 to `https://creativemcp.com/`. `site/vercel.json` adds 308s from `/index.html` and `/docs/index.html` to their directory URLs. |
| T2 robots.txt doesn't block important pages | SATISFIED | `robots.txt` allows everything and points to the sitemap. |
| T3 noindex and canonical agree | SATISFIED | Every public page has a self-canonical. The two design mockups and the 404 page carry `noindex` (meta and `X-Robots-Tag`), have no canonical and are not in the sitemap. |
| T4 server HTML has title, description, h1, main content, canonical, social metadata, JSON-LD | SATISFIED | All 23 public pages, checked by script. The feature inventory on `features.html` loads JSON on demand, but its summary and the Capability Atlas data are in the HTML. |
| T5 persistent URL per page, crawlable anchors | SATISFIED | All navigation is plain `<a href>`. A link check over every `href`/`src` found 0 missing targets. |
| T6 sitemap lists only canonical 200 URLs with truthful lastmod | SATISFIED | `site/scripts/generate-discovery.py` builds the sitemap from the public pages. `lastmod` is the date of the last commit that changed the page's visible main content, so `<head>`-only edits don't bump it. |
| T7 no infinite crawl spaces | NOT_APPLICABLE | No pagination or facets. |
| T8 media has descriptive context | SATISFIED | Every content `<img>` has alt text; decorative images use `alt=""`. The social image has `og:image:alt`. |
| T9 mobile, accessibility and performance regressions checked | SATISFIED (lab) / GRAY (field) | axe-core WCAG 2.2 AA plus best-practice rules: 0 violations across 19 pages × light/dark × 1280px/375px. No real-user Core Web Vitals yet; GA4 is collecting from 2026-11-03. |
| T10 no private or staging URL indexed | SATISFIED | Mockups are `noindex` and excluded from the sitemap and `llms.txt`. |

## P03 and PRP: entity, schema and search appearance

| Gate | Verdict | Evidence |
| --- | --- | --- |
| E1 identity reconciled | SATISFIED | Every page's JSON-LD uses the same IDs: `#website`, `#organization`, `about.html#zach-shallbetter`, `apps.html#adobe-mcp`, `docs/implementations/blender.html#blender-mcp`. `sameAs` points only to owned repos, npm and the creator's own profiles. |
| E2 schema matches visible content | SATISFIED | BreadcrumbList matches the visible breadcrumb nav on docs pages. SoftwareApplication facts (requirements, licences, OS) restate what the pages say. |
| E3 no misleading markup | SATISFIED | No Offer, AggregateRating, Review or FAQ markup. Licences come from the GitHub repos (Adobe MCP: MIT; Blender MCP: MIT OR Apache-2.0). |
| E4 canonical and @id consistent | SATISFIED | Page `@id` = canonical + `#webpage`. |
| E5 validator passes | SATISFIED | Schema.org Markup Validator (live, 2026-11-03): 0 errors and 0 warnings on a docs page, Projects and About, after splitting docs pages into WebPage + TechArticle. Google Rich Results Test on `/docs/implementations/blender.html`: Article valid with no issues (after switching `dateModified` to full ISO 8601 timestamps with a time zone), Breadcrumbs valid. |
| E6 / E7 feature eligibility and observed appearance | SATISFIED (eligibility) / GRAY (appearance) | Article and Breadcrumbs are eligible. SoftwareApplication is deliberately **ineligible**: Google requires `aggregateRating` or `review` for software rich results, and DPP E3 forbids inventing them, so it stays as plain Schema.org markup. No appearance in search is claimed until observed. |
| Titles and descriptions | SATISFIED | 23 unique titles and descriptions. The API, CLI and content docs descriptions were rewritten as real summaries instead of truncated opening sentences. |
| Social previews | SATISFIED (markup) / GRAY (render) | Each of the 24 public pages has its own 1200×630 card (`assets/social/<page>.jpg`, 114–226KB): brand row, section eyebrow, page title, a short subtitle, an app-specific brush accent and app badges, the URL and the author credit. The About card uses the portrait. Every page has `og:image` with type, size and descriptive alt text, a matching `twitter:image` and `summary_large_image`, and the same image as `primaryImageOfPage` in JSON-LD. Cards are generated by `site/scripts/generate-social-images.py`. Check with each platform's preview tool after deploy. |
| Planned vs shipped (ASI 8.2) | SATISFIED | `docs/architecture.html` labels the installer **Implemented and tested (v0.1.0)** and states its limits: full support on macOS only, minimal Linux detection, no Windows detection. Commands not built yet are listed as not built in `docs/ARCHITECTURE.md`. |
| Authorship | SATISFIED | Every page has `<meta name="author" content="Zach Shallbetter">`, `<link rel="author">` to the About page, and a visible footer byline. Every JSON-LD graph names the Person (`about.html#zach-shallbetter`) as `author`/`creator` of the page and site. Docs pages use `og:type=article` with `article:author` and an `article:modified_time` synced to the content date. The social image and its alt text credit Zach Shallbetter. |
| Icons | SATISFIED | `favicon.ico` (16/32/48px), an SVG favicon that switches to lighter stops in dark mode, a 180px Apple touch icon on the brand paper colour, 192/512px manifest icons, separate maskable icons with the mark inside the safe zone, `application-name` and `apple-mobile-web-app-title`, and an Organization `logo` in JSON-LD. |

## P04 and ASI: AI retrieval and answer visibility

| Item | Verdict | Evidence |
| --- | --- | --- |
| Key facts as server-rendered HTML text | SATISFIED | No critical fact exists only in canvas, images or script output. |
| Machine-readable discovery routes (DAP) | SATISFIED | `sitemap.xml`; `llms.txt` (curated index, linked from every page with `rel="alternate"`); `llms-full.txt` (Markdown text of every public page's main content). These are access routes only: DPP states they don't guarantee inclusion in AI answers. |
| No hidden or machine-only instructions | SATISFIED | No hidden prompt text. `llms.txt` contains only facts that are also visible on the pages. |
| Crawler policy | SATISFIED (as declared) | All crawlers allowed, AI crawlers included. Change `robots.txt` if the licensing decision changes. |
| Answer monitoring (fixed prompt panel) | GRAY | Not started. Suggested panel: "What is Creative MCP?", "How do I connect Claude to InDesign?", "Is there an MCP server for Blender written in Rust?", "Who makes Adobe MCP?" |

## Accessibility (P01 T9, P02 C10)

| Item | Verdict | Evidence |
| --- | --- | --- |
| Contrast (WCAG 1.4.3) | SATISFIED | `--ink-3` darkened to #6b675f (4.95:1 on paper) and `--warn` to #965400. Accent-coloured text is mixed 35% toward the ink colour, and badge backgrounds come from the theme panel colour, so both themes pass 4.5:1. |
| Target size (2.5.8) | SATISFIED | Footer links are at least 24px tall. |
| Keyboard access to scrolling regions | SATISFIED | Every `<pre>` and scrolling table wrapper is focusable; table regions are labelled from their headings. |
| Heading hierarchy | SATISFIED | One h1 per page and no skipped levels. Visually hidden h2s were added on Projects and How it works. |
| Landmarks and ARIA | SATISFIED | Notes inside `<main>` no longer use `<aside>`. Docs search no longer claims combobox/listbox semantics it doesn't implement: it is a labelled search field with a live result count and plain links. Docs breadcrumbs are `<nav aria-label="Breadcrumb">` with `aria-current`. |
| Colour-blind safety | SATISFIED | Atlas automation-route bars and legend swatches use a distinct pattern per route plus hover titles with counts; the bars also have `aria-label`s. Checked under a deuteranopia simulation. App identity always comes with a text name. |
| Reader views | SATISFIED | Content is in `<main>` with semantic headings, paragraphs and lists. `llms-full.txt` provides a text-only rendering. |
| Theme | SATISFIED | Follows `prefers-color-scheme` unless the visitor has chosen a theme. |

## DAP: access and headers

`site/vercel.json` sets `X-Content-Type-Options`, `Referrer-Policy`, `X-Frame-Options` and `Permissions-Policy`, a 7-day cache on `/assets/`, `X-Robots-Tag: noindex` on the mockups and 404 page, and correct content types for `llms*.txt` and the manifest. HSTS is already served by Vercel. A Content-Security-Policy is not set: it would need nonces or hashes for the inline theme script and gtag, so it's left as a follow-up.

## Distribution: Homebrew (DIS)

| Item | Verdict | Evidence |
| --- | --- | --- |
| Homebrew: Adobe MCP | SATISFIED | Tap https://github.com/zachshallbetter/homebrew-tap is published (2026-11-03). `brew install zachshallbetter/tap/adobe-mcp` from GitHub installs 0.3.0 and links 7 commands. `brew test` and `brew audit --strict --online` pass. Shown on the home page, How it works, Projects, Adobe MCP, the setup builder (Homebrew is the default on macOS and Linux), Getting started, CLI & configuration, the Suite and Blender guides, and `llms.txt`. The tap README credits the author, the formula homepages point to creativemcp.com, and the repo has topics. |
| Homebrew: Blender MCP | SATISFIED | blender-mcp-rs `v0.5.0` released 2026-11-03 (CI green on Linux, macOS and Windows; licence files added). `brew install zachshallbetter/tap/blender-mcp` builds the release tarball; `brew test` and the strict online audit pass. |
| Old in-repo formula | SATISFIED | adobe-mcp PR #430 (merged 2026-11-03, `a23db8d`) deprecates `Formula/adobe-mcp-suite.rb` (0.2.1) in favour of the tap. The tap formula deliberately has no cross-tap `conflicts_with`: Homebrew 7 refuses to load an untrusted tap it references. |

## P07: post-publication checks

Deployed 2026-11-03 from `zachshallbetter/creative-mcp` (Vercel, Root Directory `site`).

| Check | Result |
| --- | --- |
| Response headers and status codes | Observed live: 24 sitemap URLs return 200; `/index.html` and `/docs/index.html` return 308 to their directory URLs; unknown paths return the custom 404; security headers, the 7-day asset cache and `X-Robots-Tag: noindex` on mockups are present. |
| Social cards as crawlers see them | Fetched with a `facebookexternalhit` user agent: all 24 pages expose a distinct `og:image` that returns 200 and `twitter:card=summary_large_image`. |
| Google Search Console | Domain property `creativemcp.com` verified. `sitemap.xml` submitted 2026-11-03: status **Success**, 24 URLs discovered. Search Console is linked to the GA4 web stream. |
| Bing Webmaster Tools | `sitemap.xml` submitted 2026-11-03; status Processing. |
| GA4 on production | `page_view` reaches `G-JBJ1NMP96V` without `debug_mode`. Enhanced measurement, internal and developer traffic filters (Active), 14-month retention, 13 custom dimensions and 1 custom metric are configured. |
| Accessibility on production | axe-core WCAG 2.2 AA: 0 violations on five key pages at 1280px and 375px. |
| Rich Results Test | Run 2026-11-03 (see E5/E6): Article and Breadcrumbs valid; SoftwareApplication ineligible by design. |
| Social-network preview tools | Not run. Facebook and LinkedIn require signing in; X has no public validator. Crawler-level checks above pass. GRAY until a share is observed. |
| Indexing, queries, AI answer panel | GRAY. Re-review at 7, 28 and 90 days and record observations, not targets. |

## Maintenance

After changing page content, commit it, then run `python3 site/scripts/generate-discovery.py` and commit the regenerated `sitemap.xml`, `llms.txt` and `llms-full.txt`. When a page is added or its title changes, add or edit its entry in `CARDS` in `site/scripts/generate-social-images.py` and run that script too (it refuses to run if a public page has no card).
