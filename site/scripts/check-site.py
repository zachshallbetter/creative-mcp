#!/usr/bin/env python3
"""Check that every indexable page, and everything that describes it, agrees.

Run after apply-shell.py and generate-discovery.py: python3 scripts/check-site.py
Exits 1 and lists each problem; prints the page inventory when everything passes.

The inventory is the sitemap. For each page it checks:
- the HTML file exists, is indexable and its canonical URL is itself
- title, description (110-160 characters), og:title/og:description/twitter:* agree with them
- og:image and twitter:image are the same versioned card, the file exists, is a 1200x630 JPEG,
  its hash matches the ?v= in the URL, and it has alt text
- JSON-LD parses, every referenced @id is defined on the page, and the page node names an image
- the page is listed in llms.txt and has a Markdown copy
Pages outside the sitemap (404, design mockups) must be noindex.
"""
import hashlib
import html
import json
import re
import struct
import sys
from pathlib import Path

SITE = "https://creativemcp.com"
DIST = Path(__file__).resolve().parent.parent / "dist"
LD = re.compile(r'<script type="application/ld\+json">\n(.*?)\n</script>', re.S)


def meta(source, attr, name):
    m = re.search(rf'<meta {attr}="{re.escape(name)}" content="([^"]*)"', source)
    return html.unescape(m.group(1)) if m else None


def jpeg_size(data):
    """(width, height) from a JPEG's SOF marker."""
    i = 2
    while i < len(data):
        if data[i] != 0xFF:
            i += 1
            continue
        marker = data[i + 1]
        if marker in (0xC0, 0xC1, 0xC2):
            h, w = struct.unpack(">HH", data[i + 5:i + 9])
            return w, h
        i += 2 + struct.unpack(">H", data[i + 2:i + 4])[0]
    return None


def ids(value, defined, referenced):
    if isinstance(value, dict):
        if "@id" in value:
            (referenced if set(value) == {"@id"} else defined).add(value["@id"])
        for v in value.values():
            ids(v, defined, referenced)
    elif isinstance(value, list):
        for v in value:
            ids(v, defined, referenced)


def page_file(url):
    path = url[len(SITE):].lstrip("/")
    return DIST / (path + "index.html" if path == "" or path.endswith("/") else path)


def check_page(url, llms, problems):
    file = page_file(url)
    rel = file.relative_to(DIST).as_posix()
    say = lambda msg: problems.append(f"{rel}: {msg}")
    if not file.exists():
        say("in the sitemap but the file is missing")
        return
    s = file.read_text()
    head = s.split("</head>", 1)[0]
    if re.search(r'name="robots" content="[^"]*noindex', head):
        say("in the sitemap but marked noindex")
    canonical = re.search(r'<link rel="canonical" href="([^"]+)"', head)
    if not canonical or canonical.group(1) != url:
        say(f"canonical is {canonical.group(1) if canonical else 'missing'}, expected {url}")

    title = re.search(r"<title>(.*?)</title>", head, re.S)
    title = html.unescape(title.group(1).strip()) if title else None
    description = meta(head, "name", "description")
    if not title:
        say("no <title>")
    if not description:
        say("no meta description")
    elif not 110 <= len(description) <= 160:
        say(f"description is {len(description)} characters (aim for 110-160)")
    for attr, name, want in [("property", "og:title", title), ("name", "twitter:title", title),
                             ("property", "og:description", description), ("name", "twitter:description", description),
                             ("property", "og:url", url)]:
        got = meta(head, attr, name)
        if got is None:
            say(f"no {name}")
        elif want and got != want:
            say(f"{name} differs from the page: {got!r}")

    og_image, tw_image = meta(head, "property", "og:image"), meta(head, "name", "twitter:image")
    if not og_image:
        say("no og:image")
    else:
        if tw_image != og_image:
            say("twitter:image differs from og:image")
        if meta(head, "name", "twitter:card") != "summary_large_image":
            say("twitter:card is not summary_large_image")
        if not meta(head, "property", "og:image:alt"):
            say("og:image has no alt text")
        m = re.match(re.escape(SITE) + r"/(assets/social/[\w-]+\.jpg)\?v=(\w+)$", og_image)
        if not m:
            say(f"og:image is not a versioned social card: {og_image}")
        else:
            card = DIST / m.group(1)
            if not card.exists():
                say(f"social card {m.group(1)} is missing")
            else:
                data = card.read_bytes()
                if hashlib.sha256(data).hexdigest()[:10] != m.group(2):
                    say(f"{m.group(1)} changed since its ?v= hash; run generate-social-images.py")
                if jpeg_size(data) != (1200, 630):
                    say(f"{m.group(1)} is {jpeg_size(data)}, not 1200x630")
                for attr, name, want in [("property", "og:image:width", "1200"), ("property", "og:image:height", "630"),
                                         ("property", "og:image:type", "image/jpeg")]:
                    if meta(head, attr, name) != want:
                        say(f"{name} is {meta(head, attr, name)!r}, expected {want}")

    ld = LD.search(s)
    if not ld:
        say("no JSON-LD")
    else:
        try:
            graph = json.loads(ld.group(1))["@graph"]
        except (ValueError, KeyError) as e:
            say(f"JSON-LD does not parse: {e}")
        else:
            defined, referenced = set(), set()
            ids(graph, defined, referenced)
            for missing in sorted(referenced - defined):
                say(f"JSON-LD refers to {missing} but doesn't define it")
            page_nodes = [n for n in graph if n.get("url") == url and n.get("@type") not in ("Organization", "Person", "SoftwareApplication")]
            if not page_nodes:
                say("JSON-LD has no node for this page")
            elif not any("primaryImageOfPage" in n or "image" in n for n in page_nodes):
                say("JSON-LD page node has no image")

    if f"]({url})" not in llms:
        say("not listed in llms.txt")
    md = file.with_suffix(".md") if not rel.endswith("index.html") else file.with_name("index.md")
    if not md.exists():
        say(f"no Markdown copy ({md.relative_to(DIST)})")


def main():
    sitemap = (DIST / "sitemap.xml").read_text()
    urls = re.findall(r"<loc>([^<]+)</loc>", sitemap)
    llms = (DIST / "llms.txt").read_text()
    problems = []
    for url in urls:
        check_page(url, llms, problems)
    listed = {page_file(u).resolve() for u in urls}
    for file in sorted(DIST.rglob("*.html")):
        if file.resolve() in listed:
            continue
        head = file.read_text().split("</head>", 1)[0]
        if not re.search(r'name="robots" content="[^"]*noindex', head):
            problems.append(f"{file.relative_to(DIST)}: not in the sitemap and not marked noindex")
    if problems:
        print(f"{len(problems)} problem{'s' if len(problems) != 1 else ''}:")
        for p in problems:
            print(f"  ✗ {p}")
        sys.exit(1)
    print(f"All {len(urls)} sitemap pages check out (canonical, title, description, social card, JSON-LD, llms.txt, Markdown).")
    others = sorted(f.relative_to(DIST).as_posix() for f in DIST.rglob("*.html") if f.resolve() not in listed)
    print(f"Not indexed, and marked noindex: {', '.join(others)}")


if __name__ == "__main__":
    main()
