#!/usr/bin/env python3
"""Keep the shared page shell consistent across every page in dist/.

Run after adding or editing a page, before generate-discovery.py: python3 scripts/apply-shell.py
It is idempotent. It manages:
- <head>: light and dark theme-color, color-scheme, robots directives, the Markdown alternate,
  humans.txt, the no-JavaScript stylesheet and og:locale
- JSON-LD: every page carries the WebSite, Organization and author nodes it references, so a page
  shared or crawled on its own is fully described; content pages name their copyright holder
- docs: one topic list on every guide, with the current page marked; guides wrapped in <article>
  so reader views start at the title
- the footer attribution line
"""
import copy
from datetime import datetime, timezone
import json
import re
from pathlib import Path

SITE = "https://creativemcp.com"
ROOT = Path(__file__).resolve().parent.parent
DIST = ROOT / "dist"
NOINDEX = {"brand-study.html", "style-mockup.html", "404.html"}
PERSON = f"{SITE}/about.html#zach-shallbetter"
LD = re.compile(r'(<script type="application/ld\+json">\n)(.*?)(\n</script>)', re.S)
YEAR = 2026
# When each page first went live, from the commit history of the original site repository
# (creative-mcp-site-archive). Pages added later are dated by apply-shell the first time it runs on them.
PUBLISHED = {
    "index.html": "2026-10-19T22:04:59-07:00",
    "docs/architecture.html": "2026-11-03T03:30:41-08:00",
    "privacy.html": "2026-11-03T14:32:20-08:00",
    "about.html": "2026-11-02T17:05:54-08:00",
    "apps.html": "2026-11-02T17:05:54-08:00",
    "capability-atlas.html": "2026-11-02T17:05:54-08:00",
    "docs/api.html": "2026-11-02T17:05:54-08:00",
    "docs/cli.html": "2026-11-02T17:05:54-08:00",
    "docs/content.html": "2026-11-02T17:05:54-08:00",
    "docs/contributing.html": "2026-11-02T17:05:54-08:00",
    "docs/getting-started.html": "2026-11-02T17:05:54-08:00",
    "docs/implementations/aftereffects.html": "2026-11-02T17:05:54-08:00",
    "docs/implementations/blender.html": "2026-11-02T17:05:54-08:00",
    "docs/implementations/core.html": "2026-11-02T17:05:54-08:00",
    "docs/implementations/illustrator.html": "2026-11-02T17:05:54-08:00",
    "docs/implementations/indesign.html": "2026-11-02T17:05:54-08:00",
    "docs/implementations/photoshop.html": "2026-11-02T17:05:54-08:00",
    "docs/implementations/premiere.html": "2026-11-02T17:05:54-08:00",
    "docs/implementations/suite.html": "2026-11-02T17:05:54-08:00",
    "docs/index.html": "2026-11-02T17:05:54-08:00",
    "features.html": "2026-11-02T17:05:54-08:00",
    "how-it-works.html": "2026-11-02T17:05:54-08:00",
    "projects.html": "2026-11-02T17:05:54-08:00",
    "start.html": "2026-11-02T17:05:54-08:00",
    "workflows.html": "2026-11-02T17:05:54-08:00",
}

THEME = ('<meta name="theme-color" content="#f4f0e8" media="(prefers-color-scheme: light)">'
         '<meta name="theme-color" content="#111417" media="(prefers-color-scheme: dark)">'
         '<meta name="color-scheme" content="light dark">')
ROBOTS = '<meta name="robots" content="index, follow, max-image-preview:large, max-snippet:-1, max-video-preview:-1">'
# Without JavaScript the theme toggle, copy buttons and interactive builders can't work; hide them and show
# the <noscript> fallbacks instead. Dark mode still follows the system through site-tokens.css.
NOSCRIPT = '<noscript><style>.theme-toggle, button.copy, .needs-js { display: none !important; }</style></noscript>'
BYLINE = (f'<p class="footer-byline">Created by <a href="/about.html" rel="author">Zach Shallbetter</a>. '
          f'Code is <a href="https://github.com/zachshallbetter/creative-mcp/blob/main/LICENSE">MIT licensed</a>; '
          f'site content and images © {YEAR} Zach Shallbetter. '
          f'<a href="/privacy.html">Privacy</a> · <a href="/humans.txt">humans.txt</a></p>')

DOC_TABS = [
    ("/docs/", "Docs home"), ("/docs/getting-started.html", "Getting started"), ("/docs/content.html", "Content"),
    ("/docs/api.html", "MCP API"), ("/docs/cli.html", "CLI &amp; config"), ("/docs/architecture.html", "Architecture"),
    ("/docs/implementations/indesign.html", "InDesign"), ("/docs/implementations/illustrator.html", "Illustrator"),
    ("/docs/implementations/photoshop.html", "Photoshop"), ("/docs/implementations/aftereffects.html", "After Effects"),
    ("/docs/implementations/premiere.html", "Premiere Pro"), ("/docs/implementations/suite.html", "Suite"),
    ("/docs/implementations/core.html", "Shared core"), ("/docs/implementations/blender.html", "Blender MCP"),
    ("/docs/contributing.html", "Contribute"),
]
EVENTS = {"/docs/": "nav_docs", "/docs/getting-started.html": "cta_get_started", "/docs/implementations/blender.html": "cta_blender_mcp"}
# Guides that get the shared topic list, and the element it follows when the page doesn't have one yet.
TABBED = [p.lstrip("/") for p, _ in DOC_TABS if p not in ("/docs/", "/docs/getting-started.html")]


def url_path(path):
    return "/" + (path[: -len("index.html")] if path.endswith("index.html") else path)


def markdown_path(path):
    return "/" + path[: -len(".html")] + ".md"


def shared_nodes():
    """Every node with an @id, keyed by @id, taking the fullest definition found on any page."""
    nodes = {}
    for file in sorted(DIST.rglob("*.html")):
        match = LD.search(file.read_text())
        for node in json.loads(match.group(2))["@graph"] if match else []:
            if "@id" in node and len(json.dumps(node)) > len(json.dumps(nodes.get(node["@id"], {}))):
                nodes[node["@id"]] = node
    return nodes


def references(value):
    if isinstance(value, dict):
        if set(value) == {"@id"}:
            yield value["@id"]
        for item in value.values():
            yield from references(item)
    elif isinstance(value, list):
        for item in value:
            yield from references(item)


def update_head(path, source):
    head, body = source.split("</head>", 1)
    if 'media="(prefers-color-scheme: light)"' not in head:
        head = re.sub(r'<meta name="theme-color" content="[^"]*">', THEME, head, count=1)
    if NOSCRIPT not in head:
        head = head.replace("<title>", NOSCRIPT + "<title>", 1)
    if path not in NOINDEX:
        if 'name="robots"' not in head:
            head = head.replace('<meta name="viewport" content="width=device-width, initial-scale=1">',
                                '<meta name="viewport" content="width=device-width, initial-scale=1">' + ROBOTS, 1)
        extra = [f'<link rel="alternate" type="text/markdown" href="{markdown_path(path)}" title="This page as Markdown">',
                 '<link rel="author" type="text/plain" href="/humans.txt">']
        missing = "\n".join(tag for tag in extra if tag not in head)
        if missing:
            head = re.sub(r'(<link rel="alternate" type="text/plain" href="/llms.txt"[^>]*>)', lambda m: m.group(1) + "\n" + missing, head, count=1)
        if 'property="og:locale"' not in head:
            head = head.replace('<meta property="og:site_name" content="Creative MCP">',
                                '<meta property="og:site_name" content="Creative MCP"><meta property="og:locale" content="en_US">', 1)
    return head + "</head>" + body


def update_graph(path, source, shared):
    match = LD.search(source)
    if not match or path in NOINDEX:
        return source
    data = json.loads(match.group(2))
    nodes = data["@graph"]
    # Use the fullest author node, then add every node the page refers to but doesn't define
    # (the site, the publisher, the software a guide is about), so the page stands on its own.
    nodes[:] = [copy.deepcopy(shared[PERSON]) if n.get("@id") == PERSON else n for n in nodes]
    defined = {n.get("@id") for n in nodes}
    pending = [f"{SITE}/#website", f"{SITE}/#organization", *references(nodes)]
    while pending:
        node_id = pending.pop(0)
        if node_id not in defined and node_id in shared:
            node = copy.deepcopy(shared[node_id])
            nodes.append(node)
            defined.add(node_id)
            pending += references(node)
    page_id = SITE + url_path(path)
    for node in nodes:
        is_page = node.get("url") == page_id and node.get("@type") not in ("Organization", "Person", "SoftwareApplication")
        if is_page or node.get("@type") == "TechArticle":
            node.setdefault("copyrightHolder", {"@id": PERSON})
            node.setdefault("copyrightYear", YEAR)
            # First publication: dates from the site's history (PUBLISHED); a new page is published when first processed.
            node.setdefault("datePublished", PUBLISHED.get(path) or datetime.now(timezone.utc).astimezone().isoformat(timespec="seconds"))
    text = json.dumps(data, indent=2, ensure_ascii=False)
    source = source[: match.start(2)] + text + source[match.end(2):]
    published = next((n["datePublished"] for n in nodes if "datePublished" in n), None)
    if published and 'content="article"' in source and "article:published_time" not in source:
        source = source.replace('<meta property="article:modified_time"', f'<meta property="article:published_time" content="{published}"><meta property="article:modified_time"', 1)
    return source


def doc_tabs(path):
    links = []
    for href, label in DOC_TABS:
        attrs = f' data-event-id="{EVENTS[href]}"' if href in EVENTS else ""
        if href == "/" + path:
            attrs += ' aria-current="page"'
        links.append(f'<a href="{href}"{attrs}>{label}</a>')
    return '<nav class="doc-tabs" aria-label="Documentation topics">' + "".join(links) + "</nav>"


def update_docs(path, source):
    if path not in TABBED:
        return source
    tabs = doc_tabs(path)
    if '<nav class="doc-tabs"' in source:
        source = re.sub(r'<nav class="doc-tabs"[^>]*>.*?</nav>', lambda m: tabs, source, count=1, flags=re.S)
    else:
        source = re.sub(r'(<p class="(?:story-)?lede">.*?</p>)', lambda m: m.group(1) + tabs, source, count=1, flags=re.S)
    if "doc-article" not in source:
        # Reader views and assistive tech start at the guide's title; the breadcrumb stays outside it.
        source = re.sub(r'(<main[^>]*>\s*(?:<nav class="breadcrumb".*?</nav>)?\s*)', r'\1<article class="doc-article">', source, count=1, flags=re.S)
        source = source.replace("</main>", "</article></main>", 1)
    return source


def update_footer(source):
    return re.sub(r'<p class="footer-byline">.*?</p>', lambda m: BYLINE, source, count=1, flags=re.S)


def main():
    shared = shared_nodes()
    changed = 0
    for file in sorted(DIST.rglob("*.html")):
        path = file.relative_to(DIST).as_posix()
        source = file.read_text()
        updated = update_footer(update_docs(path, update_graph(path, update_head(path, source), shared)))
        if updated != source:
            file.write_text(updated)
            changed += 1
    print(f"Updated {changed} pages")


if __name__ == "__main__":
    main()
