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
| T3 noindex and canonical agree | SATISFIED | Every public page has a self-canonical. The 404 page carries `noindex` (meta and `X-Robots-Tag`), has no canonical and is not in the sitemap. The design mockups are no longer deployed (moved to `site/design/` on 2026-11-14). |
| T4 server HTML has title, description, h1, main content, canonical, social metadata, JSON-LD | SATISFIED | All 25 sitemap pages, checked by `site/scripts/check-site.py` (also run in CI by the `Site` workflow). The feature inventory on `features.html` loads JSON on demand, but its summary and the Capability Atlas data are in the HTML. |
| T5 persistent URL per page, crawlable anchors | SATISFIED | All navigation is plain `<a href>`. A link check over every `href`/`src` found 0 missing targets. |
| T6 sitemap lists only canonical 200 URLs with truthful lastmod | SATISFIED | `site/scripts/generate-discovery.py` builds the sitemap from the public pages. `lastmod` is the full timestamp of the last commit that changed the page's visible main content (so same-day edits are detected by the IndexNow workflow), so `<head>`-only edits don't bump it. |
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
| E6 / E7 feature eligibility and observed appearance | SATISFIED (eligibility) / GRAY (appearance) | Article and Breadcrumbs are eligible. SoftwareApplication is deliberately **ineligible**: Google requires `aggregateRating` or `review` for software rich results, and DPP E3 forbids inventing them, so it stays as plain Schema.org markup. **Decision (#10, 2026-11-03):** add the truthful free `offers` (`price` 0, USD) with `softwareVersion`, `operatingSystem`, `downloadUrl` and `installUrl` to Adobe MCP, Blender MCP and the `creative-mcp` installer. These facts are kept in one table in `site/scripts/apply-shell.py`. Ratings and reviews are added only if they come from real, independent users and are shown on the page. No appearance in search is claimed until observed. |
| Titles and descriptions | SATISFIED | 25 unique titles and descriptions, each 110-160 characters and matching its Open Graph and Twitter copies (`check-site.py`). The API, CLI and content docs descriptions were rewritten as real summaries instead of truncated opening sentences. |
| Social previews | SATISFIED (markup) / GRAY (render) | Each of the 25 public pages has its own 1200×630 card (`assets/social/<page>.jpg`, 114–226KB): brand row, section eyebrow, page title, a short subtitle, three app-specific brush strokes, the official app icons for the apps the page covers (Adobe's icons from adobe.com, the Blender logo; `assets/logos/`), the URL and the author credit. The About card uses the portrait. Every page has `og:image` with type, size and descriptive alt text, a matching `twitter:image` and `summary_large_image`, and the same image as `primaryImageOfPage` in JSON-LD. Cards are generated by `site/scripts/generate-social-images.py`, which adds a content hash to every card URL (`?v=…`) so platforms that cache images by URL (LinkedIn, X) fetch a changed card. Check with each platform's preview tool after deploy. |
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
| Answer monitoring (fixed prompt panel) | GRAY | Five fixed prompts (listed in [#2](https://github.com/zachshallbetter/creative-mcp/issues/2)), run at each post-launch review. First run 2026-11-13: see [Post-launch reviews](#post-launch-reviews). |

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

`site/vercel.json` sets `X-Content-Type-Options`, `Referrer-Policy`, `X-Frame-Options` and `Permissions-Policy`, a 7-day cache on `/assets/`, `X-Robots-Tag: noindex` on the 404 page, and correct content types for `llms*.txt` and the manifest. HSTS is already served by Vercel. A Content-Security-Policy (2026-11-03) allows scripts only from the site, five SHA-256 hashes for the inline theme and page scripts, and Google Tag Manager; styles from the site plus inline `style` attributes; images, connections and beacons only from the site and Google Analytics' documented origins; and no plugins, framing by other sites, `<base>` changes or off-site form posts. `site/scripts/check-site.py` fails when a page has an inline script whose hash isn't in the policy. Before deploying, every page was loaded in headless Chrome under the exact header with no violations; a control page with an unlisted inline script was blocked.

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
| Response headers and status codes | Observed live (before `privacy.html` was added, so 24): sitemap URLs return 200; `/index.html` and `/docs/index.html` return 308 to their directory URLs; unknown paths return the custom 404; security headers, the 7-day asset cache and `X-Robots-Tag: noindex` on mockups are present. |
| Social cards as crawlers see them | Fetched with a `facebookexternalhit` user agent (before `privacy.html`, so 24): all pages expose a distinct `og:image` that returns 200 and `twitter:card=summary_large_image`. |
| Google Search Console | Domain property `creativemcp.com` verified. `sitemap.xml` submitted 2026-11-03 and resubmitted after the page-completeness pass: status **Success**, 25 URLs discovered. URL Inspection: `/`, `/start.html`, `/apps.html`, `/docs/getting-started.html` and `/docs/implementations/blender.html` are indexed (Blender guide: Breadcrumbs valid); recrawl requested for all five. `/privacy.html` was discovered but not yet indexed; indexing requested. Search Console is linked to the GA4 web stream. |
| Bing Webmaster Tools | `sitemap.xml` submitted 2026-11-03; status Processing. |
| IndexNow | Key file `/b972b7f21695dfb5963f98720e14cd33.txt` at the site root. `site/scripts/indexnow.py` notifies Bing, Yandex, Naver, Seznam and the other IndexNow engines; the `IndexNow` workflow runs it on every push that changes the sitemap, after the deploy is live, with only the pages whose `lastmod` changed. All 24 URLs submitted by hand 2026-11-03: HTTP 202 (received, key validation pending). The workflow has not run yet; its first run is the next sitemap change. |
| GA4 on production | `page_view` reaches `G-JBJ1NMP96V` without `debug_mode`. Enhanced measurement, internal and developer traffic filters (Active), 14-month retention, 13 custom dimensions and 1 custom metric are configured. |
| Accessibility on production | axe-core WCAG 2.2 AA: 0 violations on five key pages at 1280px and 375px. |
| Rich Results Test | Run 2026-11-03 (see E5/E6): Article and Breadcrumbs valid; SoftwareApplication ineligible by design. |
| Social-network preview tools | **Facebook Sharing Debugger** (2026-11-03): all 25 pages scraped, 0 issues in the Batch Invalidator; each page shows its own card, title and description. The only note is the optional `fb:app_id`, which needs a Meta app and is not required for link previews. **LinkedIn Post Inspector** (2026-11-03): all 25 pages scraped; large card, title, description and author (Zach Shallbetter) detected. It reported no publication date, so `datePublished` and `article:published_time` were added from the original site's history. **X** (2026-11-03): a real post of the home page (https://x.com/zachshallbetter/status/2117777531021783538) shows the large card with the page's own image, title and "From creativemcp.com". **Slack, iMessage:** need a real share; GRAY until one is observed. |
| Indexing, queries, AI answer panel | GRAY. Reviewed at 7, 28 and 90 days; observations are in [Post-launch reviews](#post-launch-reviews). |

## Post-launch reviews

Observations, not targets ([#2](https://github.com/zachshallbetter/creative-mcp/issues/2)). Verdicts above stay GRAY until all three reviews are recorded.

### 7 days (recorded 2026-11-13, ten days after launch)

| Source | Observation |
| --- | --- |
| Google Search Console, indexing (report updated 2026-11-08) | 26 indexed: all 25 sitemap pages plus `humans.txt`. 10 not indexed, none of them pages: 3 "Page with redirect" (the `http://` and `www.` variants, as intended) and 7 "Crawled, currently not indexed" (`/data/*.json`, `favicon.ico`, and a `www` URL with a tracking parameter from May 2026). |
| Google Search Console, performance (data to 2026-11-10) | Last 7 days: 1 click, 126 impressions, CTR 0.8%, average position 9.7. Last 28 days: 1 click, 136 impressions. Top pages by impressions: `/` (47, position 3.8), `/capability-atlas.html` (39, 5.6), `/about.html` (17, 3.3); the one click went to `/workflows.html`. Top queries: "mcp creative" (7 impressions, position 2.7), "adobe creative mcp" (3, 7.0), then single impressions for Illustrator, InDesign and Blender MCP queries at positions 26–89. |
| Bing | Webmaster Tools not checked (needs the owner's sign-in). Public search: no creativemcp.com page is in Bing's index; `site:creativemcp.com` returns an unrelated domain, and "Creative MCP Zach Shallbetter" finds only the GitHub repos, zachshallbetter.com and MCP directories. All five IndexNow submissions since launch returned HTTP 200 and the key file is served. |
| GA4 (16 Oct–12 Nov, internal and developer traffic excluded) | 72 sessions: Direct 61, Referral 6, Organic Search 4, Organic Social 1. `copy_command` has been received (1); `setup_ready` has not ([#1](https://github.com/zachshallbetter/creative-mcp/issues/1)). |

AI answer panel, 2026-11-13, Google Search (US English, signed in) and Claude (`claude-opus-5-5`, no search):

| Prompt | Google AI Overview | creativemcp.com cited or ranked | Accurate about Creative MCP |
| --- | --- | --- | --- |
| What is Creative MCP? | Describes the generic idea and Adobe's own Creative Cloud MCP integrations announced at Adobe MAX (Adobe Newsroom, TechCrunch 2026-11-10) | No; not on the first page | Not about this project |
| How do I connect Claude to InDesign? | Points to Adobe's connector help pages and a third-party plugin | No | Not about this project |
| Is there an MCP server for Blender written in Rust? | Cites a different `blender-mcp-rs` (crate on docs.rs and lib.rs, GitHub JLFN) | No | Names another project with the same name |
| Who makes Adobe MCP? | Adobe's own announcement (Adobe Newsroom, TechCrunch, CNET) | No | Not about this project |
| How do I install Creative MCP with Homebrew? | Correct commands (`brew tap`, `brew trust`, `brew install zachshallbetter/tap/creative-mcp`, `creative-mcp setup`), credited to Zach Shallbetter, citing the `homebrew-tap` GitHub repo | Yes: `/start.html` is the first web result and `/how-it-works.html` the fourth | Yes |

Claude without search knows none of these facts: its training data predates the launch.

Two things changed around the project since launch: Adobe now ships its own "Creative Cloud MCP" integrations, which take over the generic queries, and another project publishes a crate named `blender-mcp-rs`.

## Open work

Each item still GRAY or pending above is tracked as an issue:

| Issue | Covers |
| --- | --- |
| [#1](https://github.com/zachshallbetter/creative-mcp/issues/1) | GA4 key events `setup_ready` and `copy_command` |
| [#2](https://github.com/zachshallbetter/creative-mcp/issues/2) | Indexing, queries and the AI-answer panel at 7, 28 and 90 days (P07, ASI) |
| [#3](https://github.com/zachshallbetter/creative-mcp/issues/3) | Social-network preview tools |
| [#4](https://github.com/zachshallbetter/creative-mcp/issues/4) | Unverified InDesign and Premiere Pro detection rules |

## Maintenance

After editing `registry/registry.json`, run `python3 site/scripts/generate-catalogue.py` first: it renders the Projects grid, the series table, the setup builder's options and `data/setup.json` from the catalogue (CI fails if they are stale). After adding or editing a page, run `python3 site/scripts/apply-shell.py`, then `generate-discovery.py`, then `python3 site/scripts/check-site.py`, which must pass (CI runs it on every push that touches `site/`). It keeps every page's shared head (theme colours, robots, Markdown alternate, no-JavaScript styles), self-contained JSON-LD, docs topic list and footer consistent. Commit the content and the regenerated `sitemap.xml` (with each page's social image), `llms.txt`, `llms-full.txt` and the per-page `.md` copies. Pushing that commit to `main` triggers the `IndexNow` workflow, which waits for Vercel to serve the new sitemap and submits the changed URLs. To submit by hand: `python3 site/scripts/indexnow.py --since <rev>` (or `--all`; add `--dry-run` to preview). When a page is added or its title changes, add or edit its entry in `CARDS` in `site/scripts/generate-social-images.py` and run that script too (it refuses to run if a public page has no card).

## Page-level completeness (2026-11-03)

Every public page is meant to stand on its own when shared, crawled or read in isolation:

- **Description:** 110 to 160 characters, written to be read on its own; `og:description` and `twitter:description` match it.
- **Social card:** a page-specific 1200×630 image with alt text; also listed in the image sitemap.
- **JSON-LD:** the page node plus every node it references (site, publisher, author, the software or dataset it is about); no dangling `@id`. Content pages name their copyright holder. The feature inventory is described as a `Dataset` with one JSON download per app.
- **Readers:** guides are wrapped in `<article>` with the breadcrumb outside it, so Safari and Firefox reader views start at the title (checked with Mozilla Readability).
- **Without JavaScript:** content is readable and dark mode follows the system. The setup builder, checklist and feature inventory are replaced by notes that link to their written equivalents; the theme toggle and copy buttons are hidden.
- **Machine-readable copies:** `/path.md` for every page (front matter with title, description, URL, author, modified date and copyright), served as `text/markdown` with a canonical `Link` header to the HTML page.
- **Extra files:** `/humans.txt`, `/.well-known/security.txt` (GitHub private vulnerability reporting, enabled on all three repositories), `/robots.txt`, `/privacy.html`.
- **Accessibility:** axe-core WCAG 2.0/2.1/2.2 A and AA rules, light and dark themes, 375 px: 0 violations on all 25 pages. Forced-colours and print styles are included.
- **One inventory check:** `site/scripts/check-site.py` takes the sitemap as the page inventory and checks, for each page: the file exists and is indexable; the canonical URL is itself; title and description exist and the description is 110-160 characters; `og:` and `twitter:` copies match them; `og:image` and `twitter:image` are the same versioned card, which exists, is a 1200×630 JPEG whose hash matches its `?v=` and has alt text; JSON-LD parses, has no dangling `@id` and the page node names an image; the page is in `llms.txt` and has a Markdown copy. HTML files outside the sitemap must be `noindex` (today: `404.html`). Result on 2026-11-03: all 25 pass. It found that the home and About page nodes named no image; `apply-shell.py` now adds the page's card as `primaryImageOfPage` (or `image` on the `WebSite` node).

## WCAG 2.0 A and AA, by criterion (2026-11-03)

Scope: all 25 sitemap pages plus `404.html`, light and dark themes. "axe" means axe-core 4.10 with the wcag2a, wcag2aa, wcag21aa and wcag22aa tags, run at 320 px (and earlier at 375 px). Automated checks don't establish conformance on their own, so each row says what else was done.

| Criterion | Method | Result |
| --- | --- | --- |
| 1.1.1 Non-text content | axe; manual review of decorative brushes (`alt=""`), icons and social-card alt text | Pass |
| 1.3.1 Info and relationships | axe; heading order, landmarks, breadcrumbs as `nav`, data tables with headers, labelled form controls | Pass |
| 1.3.2 Meaningful sequence | Reader view (Mozilla Readability) and keyboard order | Pass |
| 1.4.3 Contrast (minimum) | axe, light and dark | Pass |
| 1.4.4 Resize text | Every computed font size doubled (text-only 200%) on home, Projects, Adobe MCP, Features, Atlas, Get started, Getting started and CLI at 1280 px: no text clipped, no sideways scroll. Cropping on home is limited to decorative brush art | Pass |
| 2.1.1 Keyboard / 2.1.2 No trap | Tab through the home page (22 stops), the setup builder, the docs search and the Atlas tabs | Pass |
| 2.2.2 Pause, stop, hide | Home animations stop under `prefers-reduced-motion`; nothing auto-updates | Pass |
| 2.4.1 Bypass blocks | Skip link on every page, first in tab order | Pass |
| 2.4.2 Page titled / 2.4.4 Link purpose | `check-site.py` (unique titles); axe | Pass |
| 2.4.3 Focus order / 2.4.7 Focus visible | Keyboard pass; one 2 px focus ring for every focusable element (`:where(...):focus-visible`) | Pass |
| 3.1.1 Language of page | `lang="en"` on every page (axe) | Pass |
| 3.2.1 / 3.2.2 On focus, on input | Builder and filters change only their own output; no navigation on focus or input | Pass |
| 3.3.1 / 3.3.2 Errors, labels | Docs search, feature search and route filter are labelled; no forms submit | Pass |
| 4.1.1 Parsing / 4.1.2 Name, role, value | axe; theme toggle exposes `aria-pressed`; tabs use `role="tab"` with `aria-selected` | Pass |
| Status messages (2.1 4.1.3) | Docs search count, setup output and readiness summary are `aria-live="polite"`; the feature count is now an explicit `role="status"` live region | Pass |
| 1.4.10 Reflow (2.1) | All 26 pages at 320 px: no sideways scroll (the home page brush layer is now clipped by `body`) | Pass |

Not done: a live screen-reader session (VoiceOver) through the docs search, feature filters, setup builder and readiness checklist. The semantics those depend on are checked above, but how they are announced has not been heard. Tracked in [#12](https://github.com/zachshallbetter/creative-mcp/issues/12).

## Assets and attribution (2026-11-03)

| Asset | Source | Licence or credit |
| --- | --- | --- |
| Brush artwork (`assets/brushes/`), brush-loop symbol and icons (`brand-symbol.svg`, `assets/icons/`), portrait, social cards | Made for this site by Zach Shallbetter | © 2026 Zach Shallbetter, all rights reserved (`NOTICE.md`); not covered by the MIT licence |
| Inter (`assets/fonts/inter-latin-var.woff2`) | rsms/inter | SIL Open Font License; the licence ships beside the font (`inter-LICENSE.txt`) |
| Adobe app icons (`assets/logos/*.svg`, except Blender) | adobe.com/content/dam/cc/icons, 2025 icons | Trademarks of Adobe, used only to identify the apps; stated in `NOTICE.md` |
| Blender logo (`assets/logos/blender.svg`) | Wikimedia Commons, Blender_logo_no_text.svg | Trademark of the Blender Foundation, used only to identify the app; stated in `NOTICE.md` |

No other third-party images, scripts or fonts are served. Analytics loads Google's `gtag.js` from Google (see `privacy.html`).

## Browser branding decisions (2026-11-03)

- **Theme colour:** two `theme-color` metas (light `#f4f0e8`, dark `#111417`) follow the system, and `theme.js` updates them when the visitor picks a theme. The web manifest can hold only one `theme_color`, so it keeps the light value. That is acceptable because the manifest uses `display: browser` and the page metas take precedence in browsers.
- **Safari pinned tab:** no `mask-icon`. Safari 15 and later show the SVG favicon for pinned tabs, so a separate monochrome icon would only serve older versions.
- **humans.txt:** kept, because it gives accurate credits and contact routes. `robots.txt`, `sitemap.xml`, `llms.txt` and `llms-full.txt` remain the canonical discovery files.
