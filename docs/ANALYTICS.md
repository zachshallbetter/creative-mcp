# Analytics (GA4)

Property: Creative MCP · Web stream `web` → https://creativemcp.com · Measurement ID `G-JBJ1NMP96V`

## How it is wired

- Every public page loads `gtag.js` and `/analytics.js` in `<head>`. `brand-study.html` and `style-mockup.html` (noindex mockups) are excluded.
- `/events.js` turns clicks on any `[data-event-id]` element into a `creative-mcp:interaction` event and exposes `window.creativeMcpInteraction(name, params)` for components.
- `/analytics.js` forwards those to GA4. On `localhost` or with `?debug_analytics` in the URL it sets `debug_mode`, so hits show in **Admin → DebugView**.
- The Measurement Protocol API secret is server-only. It is not in this repo and must never go in `dist/`.

## Events

| Event | Fired by | Parameters |
| --- | --- | --- |
| `page_view` | gtag config, every page | — |
| `navigation_click` | `[data-event-id^="nav_"]` links | `link_id`, `link_url` |
| `cta_click` | other `[data-event-id]` links (`cta_*`, `about_*`) | `link_id`, `link_url` |
| `theme_change` | theme toggle | `link_id`, `theme` |
| `copy_command` | copy buttons on `/` and `/start.html` | `command_id`, `implementation`, `client`, `install_method` |
| `setup_select` | implementation, client and install-method selects on `/start.html` | `implementation`, `client`, `install_method` |
| `setup_ready` | every readiness box checked on `/start.html` | `implementation` |
| `search` | docs search (1 s idle) and feature inventory search | `search_term`, `search_scope`, `results_count` |
| `select_content` | docs search result click | `content_type`, `item_id`, `search_term` |
| `view_inventory` | opening an app inventory on `/features.html` | `app` |
| `filter_features` | route filter on `/features.html` | `app`, `route` |
| `select_app` | app tab on `/capability-atlas.html` | `app` |

All of these were confirmed sending to `google-analytics.com/g/collect` from a local preview.

## Property settings to apply (Admin)

**Data streams → web → Enhanced measurement (gear)**
- Page views → Show advanced settings → uncheck *Page changes based on browser history events* (the atlas tabs use `history.replaceState`, which would log fake page views).
- Site search → off (the site sends its own `search` event; there are no search query params).
- Form interactions → off (no real forms; selects on `/start.html` would add noise).
- Keep Scrolls, Outbound clicks, File downloads (starter zip) and Video on.

**Data streams → web → Configure tag settings → Show more**
- Define internal traffic → Create: name `Owner home`, `traffic_type` = `internal`, rules:
  - IP address *equals* your public IPv4 address
  - IP address *is in range (CIDR)* your IPv6 `/64` prefix (home IPv6 addresses are usually temporary and rotate; the /64 covers the whole network)
  - Keep the actual addresses in GA only. Never commit them to this public repository.
- List unwanted referrals → none needed.

**Data filters**
- *Internal Traffic* → change state from Testing to **Active**.
- *Developer Traffic* → **Active** (keeps localhost / `debug_mode` hits out of reports; DebugView still shows them).

**Data retention** → Event data retention **14 months**; keep *Reset user data on new activity* on.

**Data collection** → leave Google signals **off** (no ads use; keeps data sharing minimal).

**Custom definitions → Create custom dimension** (scope *Event*), one per parameter:
`link_id`, `link_url`, `theme`, `command_id`, `implementation`, `client`, `install_method`, `search_scope`, `app`, `route`, `content_type`, `item_id`, `search_term`.
**Create custom metric** (scope *Event*, unit Standard): `results_count`.

**Events → Key events** → New key event: `setup_ready`, `copy_command`.

**Product links (optional)**
- Search Console: link once `creativemcp.com` is verified there.
- BigQuery: only if you want raw event export.
