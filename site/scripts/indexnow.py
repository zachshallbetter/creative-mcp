#!/usr/bin/env python3
"""Tell IndexNow search engines (Bing, Yandex, Naver, Seznam and others) which pages changed.

Submits the sitemap URLs whose <loc> or <lastmod> differ between a git revision and the working tree:
    python3 scripts/indexnow.py --since HEAD~1
    python3 scripts/indexnow.py --all            # every sitemap URL
    python3 scripts/indexnow.py --dry-run --all  # print, don't submit

Run it after the deploy is live: engines fetch the pages (and the key file) when notified.
--wait polls the live sitemap until it matches the local one first. https://www.indexnow.org/documentation
"""
import argparse
import json
import re
import subprocess
import sys
import time
import urllib.error
import urllib.request
from pathlib import Path

SITE = "https://creativemcp.com"
KEY = "b972b7f21695dfb5963f98720e14cd33"
ENDPOINT = "https://api.indexnow.org/indexnow"
ROOT = Path(__file__).resolve().parent.parent
SITEMAP = ROOT / "dist" / "sitemap.xml"


def entries(xml):
    """{loc: lastmod} for each <url> in a sitemap."""
    return {loc: lastmod for loc, lastmod in re.findall(r"<loc>([^<]+)</loc>(?:<lastmod>([^<]+)</lastmod>)?", xml)}


def fetch(url):
    request = urllib.request.Request(url, headers={"User-Agent": "creative-mcp-indexnow"})
    with urllib.request.urlopen(request, timeout=30) as response:
        return response.read().decode()


def changed_since(rev):
    old = subprocess.run(["git", "show", f"{rev}:./dist/sitemap.xml"], cwd=ROOT, capture_output=True, text=True)
    if old.returncode:
        sys.exit(f"cannot read dist/sitemap.xml at {rev}: {old.stderr.strip()}")
    before, now = entries(old.stdout), entries(SITEMAP.read_text())
    return [loc for loc, lastmod in now.items() if before.get(loc) != lastmod]


def wait_for_deploy(seconds):
    local = SITEMAP.read_text()
    deadline = time.monotonic() + seconds
    while True:
        try:
            if fetch(f"{SITE}/sitemap.xml") == local:
                return
        except urllib.error.URLError:
            pass
        if time.monotonic() > deadline:
            sys.exit(f"live sitemap still differs from dist/sitemap.xml after {seconds}s; not submitting")
        time.sleep(15)


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    which = parser.add_mutually_exclusive_group(required=True)
    which.add_argument("--since", metavar="REV", help="submit URLs whose sitemap entry changed since this git revision")
    which.add_argument("--all", action="store_true", help="submit every sitemap URL")
    parser.add_argument("--wait", type=int, default=0, metavar="SECONDS", help="first wait up to this long for the deploy to go live")
    parser.add_argument("--dry-run", action="store_true")
    args = parser.parse_args()

    urls = list(entries(SITEMAP.read_text())) if args.all else changed_since(args.since)
    if not urls:
        print("No sitemap changes; nothing to submit.")
        return
    print("\n".join(urls))
    if args.dry_run:
        print(f"Dry run: {len(urls)} URLs not submitted.")
        return

    if args.wait:
        wait_for_deploy(args.wait)
    key_location = f"{SITE}/{KEY}.txt"
    if fetch(key_location).strip() != KEY:
        sys.exit(f"{key_location} does not serve the key; deploy dist/{KEY}.txt first")

    body = json.dumps({"host": SITE.split("//")[1], "key": KEY, "keyLocation": key_location, "urlList": urls}).encode()
    request = urllib.request.Request(ENDPOINT, data=body, method="POST",
                                     headers={"Content-Type": "application/json; charset=utf-8"})
    try:
        with urllib.request.urlopen(request, timeout=30) as response:
            status = response.status
    except urllib.error.HTTPError as error:
        sys.exit(f"IndexNow rejected the submission: HTTP {error.code} {error.reason}")
    # 200: accepted. 202: accepted, key validation pending (normal on the first submission).
    print(f"Submitted {len(urls)} URLs to IndexNow: HTTP {status}")


if __name__ == "__main__":
    main()
