#!/usr/bin/env python3
"""Regenerate dist/sitemap.xml, dist/llms.txt and dist/llms-full.txt from the published pages,
and copy the series catalogue (../registry) into dist/.

Run after changing page content: python3 scripts/generate-discovery.py
lastmod comes from each page's last git commit, so commit content changes before regenerating.
"""
import html
import re
import shutil
import subprocess
from datetime import date
from html.parser import HTMLParser
from pathlib import Path

SITE = "https://creativemcp.com"
ROOT = Path(__file__).resolve().parent.parent
DIST = ROOT / "dist"
EXCLUDED = {"brand-study.html", "style-mockup.html", "404.html"}

# llms.txt sections, in reading order. Every public page must appear in exactly one.
SECTIONS = [
    ("Start here", ["index.html", "projects.html", "how-it-works.html", "start.html", "about.html"]),
    ("Documentation", ["docs/index.html", "docs/getting-started.html", "docs/content.html", "docs/api.html", "docs/cli.html", "docs/architecture.html", "docs/contributing.html"]),
    ("Implementation guides", ["apps.html", "docs/implementations/indesign.html", "docs/implementations/illustrator.html",
                               "docs/implementations/photoshop.html", "docs/implementations/aftereffects.html",
                               "docs/implementations/premiere.html", "docs/implementations/suite.html",
                               "docs/implementations/core.html", "docs/implementations/blender.html"]),
    ("Optional", ["capability-atlas.html", "features.html", "workflows.html"]),
]

LLMS_HEADER = """# Creative MCP

> Creative MCP is an open collection of local Model Context Protocol (MCP) servers that connect AI agents and MCP clients to creative software: Adobe MCP for InDesign, Illustrator, Photoshop, After Effects and Premiere Pro, and Blender MCP for Blender. Servers run on the user's own machine over stdio; nothing is hosted.

Key facts:
- Adobe MCP: five app-specific Node.js servers (Node.js 18+, macOS or Windows) plus a Suite coordinator for two documented Illustrator-to-InDesign workflows. npm: `adobe-mcp-suite`; Homebrew: `brew install zachshallbetter/tap/adobe-mcp`. Source: https://github.com/zachshallbetter/adobe-mcp (MIT).
- Blender MCP: a Rust MCP server and CLI with a companion Blender add-on (Blender 4.0+). Homebrew: `brew install zachshallbetter/tap/blender-mcp` (v0.5.0). Source: https://github.com/zachshallbetter/blender-mcp-rs (MIT OR Apache-2.0).
- The projects are independent. A client can load both, but moving files between Adobe apps and Blender is a user-managed handoff.
- Capability status is reported as Documented, Implemented, Tested or Validated; those levels are distinct and a lower level never implies a higher one.
- Created by Zach Shallbetter (https://zachshallbetter.com/).
- Website, docs and the series catalogue (https://creativemcp.com/registry.json) are open source at https://github.com/zachshallbetter/creative-mcp.
- Each tool has its own repository and release cycle; they share the Homebrew tap `zachshallbetter/tap`. One installer sets up the series: `brew install zachshallbetter/tap/creative-mcp` then `creative-mcp setup` (detects apps and MCP clients, installs, registers, verifies; macOS fully supported). The installer is built from the catalogue.

The full text of every page below is in [llms-full.txt](https://creativemcp.com/llms-full.txt).
"""

SKIP_TAGS = {"script", "style", "svg", "button", "noscript", "template", "header", "footer", "select", "input", "label"}
BLOCK_TAGS = {"p", "div", "section", "article", "aside", "figure", "figcaption", "blockquote", "dl", "dt", "dd", "details", "summary", "nav", "ul", "ol", "table", "thead", "tbody", "tr"}


class MainText(HTMLParser):
    """Collect <main> as lightweight Markdown."""

    def __init__(self, base):
        super().__init__(convert_charrefs=True)
        self.base, self.out, self.skip, self.in_main, self.in_pre = base, [], 0, False, False
        self.lists, self.href, self.link_text = [], None, []

    def emit(self, text):
        (self.link_text if self.href is not None else self.out).append(text)

    def handle_starttag(self, tag, attrs):
        a = dict(attrs)
        if tag == "main":
            self.in_main = True
            return
        if not self.in_main:
            return
        if self.skip or tag in SKIP_TAGS or a.get("aria-hidden") == "true" or "hidden" in a:
            if tag not in {"br", "img", "input", "meta", "link"}:
                self.skip += 1
            return
        if re.fullmatch(r"h[1-6]", tag):
            self.emit("\n\n" + "#" * int(tag[1]) + " ")
        elif tag in ("ul", "ol"):
            self.lists.append(tag)
        elif tag == "li":
            self.emit("\n" + "  " * (len(self.lists) - 1) + "- ")
        elif tag == "pre":
            self.in_pre = True
            self.emit("\n\n```\n")
        elif tag == "code" and not self.in_pre:
            self.emit("`")
        elif tag == "a" and a.get("href") and not a["href"].startswith("#"):
            self.href = a["href"] if a["href"].startswith("http") else SITE + a["href"] if a["href"].startswith("/") else self.base + a["href"]
            self.link_text = []
        elif tag in ("td", "th"):
            self.emit(" | ")
        elif tag == "br":
            self.emit("\n")
        elif tag == "img" and a.get("alt"):
            self.emit(f"[Image: {a['alt']}]")
        elif tag in BLOCK_TAGS:
            self.emit("\n\n")

    def handle_endtag(self, tag):
        if tag == "main":
            self.in_main = False
            return
        if not self.in_main:
            return
        if self.skip:
            self.skip -= 1
            return
        if tag in ("ul", "ol") and self.lists:
            self.lists.pop()
            self.emit("\n")
        elif tag == "pre":
            self.in_pre = False
            self.emit("\n```\n\n")
        elif tag == "code" and not self.in_pre:
            self.emit("`")
        elif tag == "a" and self.href is not None:
            text = "".join(self.link_text).strip()
            href, self.href = self.href, None
            self.out.append(f"[{text}]({href})" if text else "")
        elif re.fullmatch(r"h[1-6]", tag) or tag in BLOCK_TAGS:
            self.emit("\n")

    def handle_data(self, data):
        if self.in_main and not self.skip:
            self.emit(data if self.in_pre else re.sub(r"\s+", " ", data))

    def markdown(self):
        text = "".join(self.out)
        text = re.sub(r"[ \t]+\n", "\n", text)
        text = re.sub(r"\n[ \t]+(?=[^\s-])", "\n", text)
        return re.sub(r"\n{3,}", "\n\n", text).strip()


def page_url(path):
    if path == "index.html":
        return f"{SITE}/"
    if path.endswith("/index.html"):
        return f"{SITE}/{path[:-len('index.html')]}"
    return f"{SITE}/{path}"


def meta(source, pattern):
    match = re.search(pattern, source, re.S)
    return html.unescape(match.group(1)).strip() if match else ""


def main_text(source, path):
    parser = MainText(page_url(path).rsplit("/", 1)[0] + "/")
    parser.feed(source)
    return parser.markdown()


def git(*args):
    return subprocess.run(["git", *args], cwd=ROOT, capture_output=True, text=True).stdout


def last_modified(path, current_text):
    """Date of the last commit that changed the page's visible main content (head-only edits don't count)."""
    history = [line.split() for line in git("log", "--format=%H %cs", "--", f"dist/{path}").splitlines()]
    if not history:
        return None
    if main_text(git("show", f"{history[0][0]}:./dist/{path}"), path) != current_text:
        print(f"warning: {path} has uncommitted content changes; using today's date")
        return date.today().isoformat()
    for (commit, day), parent in zip(history, history[1:] + [None]):
        older_text = main_text(git("show", f"{parent[0]}:./dist/{path}"), path) if parent else None
        if older_text != main_text(git("show", f"{commit}:./dist/{path}"), path):
            return day
    return history[-1][1]


def publish_registry():
    """Serve the series catalogue at /registry.json (source of truth: ../registry in this repo)."""
    for name in ("registry.json", "registry.schema.json"):
        source = ROOT.parent / "registry" / name
        if source.exists():
            shutil.copyfile(source, DIST / name)


def main():
    publish_registry()
    pages = sorted(p.relative_to(DIST).as_posix() for p in DIST.rglob("*.html") if p.relative_to(DIST).as_posix() not in EXCLUDED)
    listed = [p for _, group in SECTIONS for p in group]
    missing, extra = set(pages) - set(listed), set(listed) - set(pages)
    if missing or extra:
        raise SystemExit(f"Update SECTIONS: unlisted pages {sorted(missing)}, missing files {sorted(extra)}")

    info = {}
    for path in pages:
        source = (DIST / path).read_text()
        if re.search(r'name="robots" content="[^"]*noindex', source):
            raise SystemExit(f"{path} is noindex; add it to EXCLUDED")
        title = re.sub(r"\s+[—–-]\s+Creative MCP( Docs)?$", "", meta(source, r"<title>(.*?)</title>"))
        text = main_text(source, path)
        info[path] = {"url": page_url(path), "title": title, "description": meta(source, r'name="description" content="([^"]*)"'),
                      "text": text, "lastmod": last_modified(path, text)}
        # Keep the page's own modified dates (head-only, so lastmod is unaffected) in step with its content date.
        if info[path]["lastmod"]:
            synced = re.sub(r'("dateModified": ")[^"]*(")', rf'\g<1>{info[path]["lastmod"]}\g<2>', source)
            synced = re.sub(r'(<meta property="article:modified_time" content=")[^"]*(")', rf'\g<1>{info[path]["lastmod"]}\g<2>', synced)
            if synced != source:
                (DIST / path).write_text(synced)

    sitemap = ['<?xml version="1.0" encoding="UTF-8"?>', '<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">']
    for path in pages:
        lastmod = f"<lastmod>{info[path]['lastmod']}</lastmod>" if info[path]["lastmod"] else ""
        sitemap.append(f"  <url><loc>{info[path]['url']}</loc>{lastmod}</url>")
    sitemap.append("</urlset>")
    (DIST / "sitemap.xml").write_text("\n".join(sitemap) + "\n")

    llms = [LLMS_HEADER]
    for heading, group in SECTIONS:
        llms.append(f"## {heading}\n")
        llms += [f"- [{info[p]['title']}]({info[p]['url']}): {info[p]['description']}" for p in group]
        llms.append("")
    (DIST / "llms.txt").write_text("\n".join(llms))

    full = [LLMS_HEADER.split("The full text")[0].rstrip(), ""]
    for _, group in SECTIONS:
        for p in group:
            full += ["---", "", f"Source: {info[p]['url']}", "", info[p]["text"], ""]
    (DIST / "llms-full.txt").write_text("\n".join(full))
    print(f"Wrote sitemap.xml ({len(pages)} URLs), llms.txt and llms-full.txt ({sum(len(info[p]['text']) for p in pages):,} chars)")


if __name__ == "__main__":
    main()
