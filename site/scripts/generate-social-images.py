#!/usr/bin/env python3
"""Render a 1200x630 social card for every public page and point its og:/twitter: tags at it.

Run after changing a page title or adding a page: python3 scripts/generate-social-images.py
Needs Google Chrome (headless) and macOS `sips` for JPEG conversion.
"""
import hashlib
import html
import json
import re
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path

SITE = "https://creativemcp.com"
ROOT = Path(__file__).resolve().parent.parent
DIST = ROOT / "dist"
OUT = DIST / "assets" / "social"
CHROME = "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome"
AUTHOR = "Zach Shallbetter"

# accent: brushes to layer on the right, in paint order; badges: app chips shown over them.
ACCENTS = {
    "brand": ["brand-violet", "brand-coral", "brand-magenta"],
    "magenta": ["brand-magenta", "brand-violet", "brand-coral"],
    "indesign": ["indesign", "brand-violet", "brand-magenta"],
    "illustrator": ["illustrator", "brand-coral", "brand-magenta"],
    "photoshop": ["photoshop", "brand-violet", "brand-coral"],
    "aftereffects": ["after-effects", "brand-magenta", "brand-violet"],
    "premiere": ["premiere-pro", "photoshop", "brand-violet"],
    "blender": ["brand-coral", "illustrator", "photoshop"],
    "adobe": ["indesign", "photoshop", "illustrator"],
}
# Official app marks in assets/logos/ (Adobe's 2025 app icons from adobe.com; the Blender logo).
# Trademarks of Adobe and the Blender Foundation, used to identify the apps (see NOTICE.md).
BADGES = {"indesign": "indesign", "illustrator": "illustrator", "photoshop": "photoshop",
          "aftereffects": "after-effects", "premiere": "premiere-pro", "blender": "blender"}
ALL_ADOBE = ["indesign", "illustrator", "photoshop", "aftereffects", "premiere"]

# path: (image slug, eyebrow, title, subtitle, accent, badges)
CARDS = {
    "index.html": ("home", "Creative tools, connected", "Make more with the tools you <em>already use.</em>",
                   "Local MCP servers that connect AI agents to Adobe apps and Blender.", "brand", []),
    "projects.html": ("projects", "Projects", "One protocol. Different creative <em>worlds.</em>",
                      "Adobe MCP and Blender MCP: independent, open implementations for the apps you already use.", "brand", ALL_ADOBE + ["blender"]),
    "how-it-works.html": ("how-it-works", "How it works", "From idea to output in <em>three steps.</em>",
                          "Choose an implementation, connect your MCP client, keep creating. Everything runs locally.", "magenta", []),
    "start.html": ("start", "Get started", "Start with the tools <em>you have.</em>",
                   "Pick an implementation and client to get exact setup commands, including Homebrew.", "brand", []),
    "about.html": ("about", "About the creator", "Zach Shallbetter",
                   "Artist, designer and creative technologist in Spokane, Washington. Creator of Creative MCP.", "brand", []),
    "apps.html": ("adobe-mcp", "Adobe MCP", "Every Adobe app gets its own <em>MCP server.</em>",
                  "InDesign, Illustrator, Photoshop, After Effects and Premiere Pro, plus a coordinator for cross-app handoffs.", "adobe", ALL_ADOBE),
    "features.html": ("features", "Adobe MCP · Feature inventory", "Search every documented <em>Adobe feature.</em>",
                      "Feature-level inventories for five Adobe apps, with automation routes and evidence.", "adobe", ALL_ADOBE),
    "workflows.html": ("workflows", "Workflows", "Practical handoffs between <em>creative apps.</em>",
                       "Adobe MCP and Blender MCP examples, with supported steps and manual handoffs clearly marked.", "magenta", []),
    "capability-atlas.html": ("capability-atlas", "Capability Atlas", "What automation can reach in <em>five Adobe apps.</em>",
                              "12,014 documented features, their automation routes, MCP coverage and the evidence behind each claim.", "adobe", ALL_ADOBE),
    "privacy.html": ("privacy", "Privacy", "What this site <em>measures.</em>",
                     "Google Analytics on the website only. The local tools send nothing to this site.", "brand", []),
    "docs/index.html": ("docs", "Documentation", "Find the right <em>guide.</em>",
                        "Setup, the MCP tool interface, configuration and a guide for every implementation.", "magenta", []),
    "docs/getting-started.html": ("docs-getting-started", "Docs · Getting started", "Get set <em>up.</em>",
                                  "Install the local servers, connect your MCP client and run a first workflow.", "brand", []),
    "docs/content.html": ("docs-content", "Docs · Concepts", "Content &amp; capability <em>guide</em>",
                          "What each server works on, and how a feature, a route, a tool and a validated workflow differ.", "magenta", []),
    "docs/api.html": ("docs-api", "Docs · Reference", "MCP API <em>reference</em>",
                      "Discovery, tool metadata, calls, results and execution boundaries for the Adobe MCP servers.", "magenta", []),
    "docs/cli.html": ("docs-cli", "Docs · Reference", "CLI &amp; <em>configuration</em>",
                      "Install and launch commands, Homebrew, Suite app selection and configuration variables.", "magenta", []),
    "docs/architecture.html": ("docs-architecture", "Docs · The series", "Architecture &amp; <em>distribution</em>",
                               "Separate repositories, one Homebrew tap, and the conventions every new tool follows.", "brand", ALL_ADOBE + ["blender"]),
    "docs/contributing.html": ("docs-contributing", "Docs · Contributing", "Make a useful tool easier to <em>reach.</em>",
                               "How to contribute an implementation or workflow, with scope and evidence kept clear.", "magenta", []),
    "docs/implementations/indesign.html": ("docs-indesign", "Implementation guide", "InDesign",
                                           "Layouts, typography, tables, styles, long documents and export over MCP.", "indesign", ["indesign"]),
    "docs/implementations/illustrator.html": ("docs-illustrator", "Implementation guide", "Illustrator",
                                              "Artboards, paths, layers, type, color and artwork export over MCP.", "illustrator", ["illustrator"]),
    "docs/implementations/photoshop.html": ("docs-photoshop", "Implementation guide", "Photoshop",
                                            "Documents, layers, selections, adjustments, filters and export over MCP.", "photoshop", ["photoshop"]),
    "docs/implementations/aftereffects.html": ("docs-aftereffects", "Implementation guide", "After Effects",
                                               "Projects, compositions, keyframes, effects and rendering over MCP.", "aftereffects", ["aftereffects"]),
    "docs/implementations/premiere.html": ("docs-premiere", "Implementation guide", "Premiere Pro",
                                           "Projects, sequences, timeline edits, markers and exports over MCP.", "premiere", ["premiere"]),
    "docs/implementations/suite.html": ("docs-suite", "Implementation guide", "Suite <em>coordinator</em>",
                                        "Two documented Illustrator-to-InDesign workflows behind one MCP server.", "adobe", ["illustrator", "indesign"]),
    "docs/implementations/core.html": ("docs-core", "Implementation guide", "Shared <em>core</em>",
                                       "The common infrastructure behind every Adobe MCP server.", "adobe", ALL_ADOBE),
    "docs/implementations/blender.html": ("docs-blender", "Implementation guide", "Blender MCP",
                                          "A Rust MCP server and companion add-on for scenes, objects, materials and 3D pipelines.", "blender", ["blender"]),
}

TEMPLATE = """<!doctype html><html><head><meta charset="utf-8"><style>
@font-face {{ font-family: Inter; src: url("{dist}/assets/fonts/inter-latin-var.woff2") format("woff2"); font-weight: 100 900; }}
* {{ box-sizing: border-box; }}
html, body {{ margin: 0; width: 1200px; height: 630px; overflow: hidden; }}
body {{ position: relative; background: #f4f0e8; color: #171717; font-family: Inter, sans-serif; }}
.grid {{ position: absolute; inset: 0; background-image: linear-gradient(rgba(23,23,23,.055) 1px, transparent 1px), linear-gradient(90deg, rgba(23,23,23,.055) 1px, transparent 1px); background-size: 42px 42px; -webkit-mask-image: linear-gradient(90deg, transparent 38%, #000 82%); }}
.edge {{ position: absolute; left: 0; right: 0; bottom: 0; height: 10px; background: linear-gradient(90deg, #7444d8, #cf43ad 50%, #f06430); }}
.art {{ position: absolute; right: 0; top: 0; width: 470px; height: 620px; }}
.art img.brush {{ position: absolute; mix-blend-mode: multiply; opacity: .86; }}
.art .b0 {{ width: 470px; right: -40px; top: 92px; transform: rotate(-8deg); }}
.art .b1 {{ width: 360px; right: 40px; top: 320px; transform: rotate(12deg); opacity: .62; }}
.art .b2 {{ width: 240px; right: 290px; top: 470px; transform: rotate(-10deg) scaleX(-1); opacity: .55; }}
.badges {{ position: absolute; right: 64px; top: 58px; display: flex; gap: 10px; }}
.badge {{ display: block; width: 64px; height: 64px; border-radius: 15px; box-shadow: 0 10px 24px rgba(39,31,22,.18); }}
.badge.tile {{ padding: 10px; background: #fffdf8; border: 1.5px solid rgba(23,23,23,.12); object-fit: contain; }}
.portrait {{ position: absolute; right: 64px; top: 64px; width: 400px; height: 470px; border-radius: 22px; overflow: hidden; box-shadow: 0 24px 50px rgba(39,31,22,.25); }}
.portrait img {{ width: 100%; height: 100%; object-fit: cover; object-position: 50% 30%; }}
.copy {{ position: absolute; left: 72px; top: 64px; bottom: 74px; width: {copy_width}px; display: flex; flex-direction: column; }}
.brand {{ display: flex; align-items: center; gap: 14px; font-weight: 700; font-size: 28px; letter-spacing: -.02em; }}
.brand img {{ width: 50px; height: 50px; }}
.eyebrow {{ margin-top: 44px; display: flex; align-items: center; gap: 12px; color: #4d4b46; font: 600 19px ui-monospace, "SF Mono", Menlo, monospace; letter-spacing: .1em; text-transform: uppercase; }}
.eyebrow i {{ width: 12px; height: 12px; border-radius: 3px; background: #f06430; }}
h1 {{ margin: 18px 0 0; font-weight: 800; letter-spacing: -.055em; line-height: .98; font-size: 84px; text-wrap: balance; }}
h1 em {{ font-style: normal; background: linear-gradient(90deg, #7444d8, #cf43ad 55%, #f06430); -webkit-background-clip: text; background-clip: text; color: transparent; }}
p {{ margin: 22px 0 0; color: #4d4b46; font-size: 27px; line-height: 1.32; font-weight: 500; text-wrap: pretty; }}
.foot {{ margin-top: auto; padding-top: 26px; display: flex; gap: 18px; align-items: center; font: 600 18px ui-monospace, "SF Mono", Menlo, monospace; letter-spacing: .06em; color: #171717; }}
.foot span {{ color: #6b675f; }}
</style></head><body><div class="grid"></div>{art}
<div class="copy"><div class="brand"><img src="{dist}/brand-symbol.svg" alt="">Creative MCP</div>
<div class="eyebrow"><i></i>{eyebrow}</div><h1 id="t">{title}</h1><p id="s">{subtitle}</p>
<div class="foot">{url}<span>by {author}</span></div></div><div class="edge"></div>
<script>
  // Shrink the title (max three lines), then the subtitle, until the whole column fits above the footer.
  const t = document.getElementById("t"), s = document.getElementById("s"), copy = document.querySelector(".copy");
  const overflowing = () => copy.scrollHeight > copy.clientHeight + 1;
  document.fonts.ready.then(() => {{
    let size = 84, sub = 27;
    while ((t.getBoundingClientRect().height > size * 0.98 * 3 + 4 || overflowing()) && size > 56) {{ size -= 2; t.style.fontSize = size + "px"; }}
    while (overflowing() && sub > 21) {{ sub -= 1; s.style.fontSize = sub + "px"; }}
  }});
</script></body></html>"""


def render_html(path, card):
    slug, eyebrow, title, subtitle, accent, badges = card
    dist = DIST.as_uri()
    if path == "about.html":
        art = (f'<div class="art"><img class="brush b0" src="{dist}/assets/brushes/brand-violet.png" alt="">'
               f'<img class="brush b1" src="{dist}/assets/brushes/brand-coral.png" alt="">'
               f'<img class="brush b2" src="{dist}/assets/brushes/brand-magenta.png" alt=""></div>'
               f'<div class="portrait"><img src="{dist}/assets/zach-shallbetter-portrait.webp" alt=""></div>')
        copy_width = 620
    else:
        brushes = "".join(f'<img class="brush b{i}" src="{dist}/assets/brushes/{b}.png" alt="">' for i, b in enumerate(ACCENTS[accent]))
        # Adobe icons are their own rounded tiles; the Blender logo sits on a paper tile to match.
        chips = "".join(f'<img class="badge{" tile" if b == "blender" else ""}" src="{dist}/assets/logos/{BADGES[b]}.svg" alt="">' for b in badges)
        art = f'<div class="art">{brushes}</div>' + (f'<div class="badges">{chips}</div>' if chips else "")
        copy_width = 690
    url = "creativemcp.com"
    return TEMPLATE.format(dist=dist, art=art, copy_width=copy_width, eyebrow=eyebrow, title=title,
                           subtitle=subtitle, url=html.escape(url), author=AUTHOR)


def card_url(slug):
    # A content hash in the URL makes LinkedIn, X and others fetch a changed card instead of their cached copy.
    digest = hashlib.sha256((OUT / f"{slug}.jpg").read_bytes()).hexdigest()[:10]
    return f"{SITE}/assets/social/{slug}.jpg?v={digest}"


def version_references():
    """Point every reference to a card (og:image, JSON-LD logos and images) at its current versioned URL."""
    pattern = re.compile(re.escape(SITE) + r"/assets/social/([\w-]+)\.jpg(?:\?v=\w+)?")
    for page in DIST.rglob("*.html"):
        s = page.read_text()
        updated = pattern.sub(lambda m: card_url(m.group(1)) if (OUT / f"{m.group(1)}.jpg").exists() else m.group(0), s)
        if updated != s:
            page.write_text(updated)


def alt_text(card):
    _, eyebrow, title, subtitle, _, _ = card
    plain = html.unescape(re.sub(r"<[^>]+>", "", title))
    return f"Creative MCP, {html.unescape(eyebrow)}: {plain} {subtitle} By {AUTHOR}."


def update_page(path, card):
    page = DIST / path
    s = page.read_text()
    image = card_url(card[0])
    alt = html.escape(alt_text(card), quote=True)
    s = re.sub(r'(<meta property="og:image" content=")[^"]*', rf"\g<1>{image}", s)
    s = re.sub(r'(<meta name="twitter:image" content=")[^"]*', rf"\g<1>{image}", s)
    s = re.sub(r'(<meta property="og:image:alt" content=")[^"]*', lambda m: m.group(1) + alt, s)
    s = re.sub(r'(<meta name="twitter:image:alt" content=")[^"]*', lambda m: m.group(1) + alt, s)
    s = re.sub(r'(<meta property="og:image:width" content=")[^"]*', r"\g<1>1200", s)
    s = re.sub(r'(<meta property="og:image:height" content=")[^"]*', r"\g<1>630", s)
    if 'og:image:type' not in s:
        s = s.replace('<meta property="og:image:width"', '<meta property="og:image:type" content="image/jpeg"><meta property="og:image:width"', 1)
    s = s.replace('<meta name="twitter:card" content="summary">', '<meta name="twitter:card" content="summary_large_image">')
    s = re.sub(r'("primaryImageOfPage": \{\s*"@type": "ImageObject",\s*"url": ")[^"]*', rf"\g<1>{image}", s)
    page.write_text(s)


def main():
    missing = [p for p in CARDS if not (DIST / p).exists()]
    if missing:
        raise SystemExit(f"CARDS lists missing pages: {missing}")
    public = {p.relative_to(DIST).as_posix() for p in DIST.rglob("*.html")} - {"404.html", "brand-study.html", "style-mockup.html"}
    if public - set(CARDS):
        raise SystemExit(f"Add a card for: {sorted(public - set(CARDS))}")
    OUT.mkdir(parents=True, exist_ok=True)
    only = set(sys.argv[1:])
    with tempfile.TemporaryDirectory() as tmp:
        for path, card in CARDS.items():
            if only and card[0] not in only:
                continue
            src = Path(tmp) / f"{card[0]}.html"
            png = Path(tmp) / f"{card[0]}.png"
            src.write_text(render_html(path, card))
            for attempt in range(3):  # headless Chrome occasionally exits early; retry with the same isolated profile
                done = subprocess.run([CHROME, "--headless=new", "--disable-gpu", "--hide-scrollbars", "--allow-file-access-from-files",
                                       f"--user-data-dir={tmp}/chrome-profile", "--no-first-run", "--force-device-scale-factor=1",
                                       "--virtual-time-budget=4000", "--window-size=1200,630", f"--screenshot={png}", src.as_uri()],
                                      capture_output=True)
                if done.returncode == 0 and png.exists():
                    break
            else:
                raise SystemExit(f"Chrome could not render {card[0]}: {done.stderr.decode()[-400:]}")
            subprocess.run(["sips", "-s", "format", "jpeg", "-s", "formatOptions", "86", str(png), "--out", str(OUT / f"{card[0]}.jpg")],
                           check=True, capture_output=True)
            update_page(path, card)
            print(f"{card[0]}.jpg  ←  {path}")
    version_references()
    old = OUT / "creative-mcp-og.jpg"
    if old.exists() and not only:
        old.unlink()


if __name__ == "__main__":
    main()
