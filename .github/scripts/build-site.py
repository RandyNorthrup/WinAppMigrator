#!/usr/bin/env python3
"""Build the WinAppMigrator website from the wiki and the files in site/.

Search engines do not read a GitHub wiki until its repository has 500 stars, so this publishes the same pages where
they can. Each wiki page goes through GitHub's own Markdown renderer, the one the wiki uses, so the site says exactly
what the wiki says. The landing page is site/index.html. Every link between pages, every #anchor, and every local
image is checked, and the build fails on one that leads nowhere.

Standard library only. It needs a GitHub token in GITHUB_TOKEN or GH_TOKEN for GitHub's API.

    python3 -I .github/scripts/build-site.py --wiki ../WinAppMigrator.wiki --out _site
"""

from __future__ import annotations

import argparse
import datetime as dt
import html
import json
import os
import re
import shutil
import string
import subprocess
import sys
import urllib.error
import urllib.parse
import urllib.request
import xml.etree.ElementTree as ET
from html.parser import HTMLParser
from pathlib import Path

API = "https://api.github.com"
DEFAULT_REPOSITORY = "RandyNorthrup/WinAppMigrator"
DEFAULT_BASE_URL = "https://randynorthrup.github.io/WinAppMigrator"
SITEMAP_NS = "http://www.sitemaps.org/schemas/sitemap/0.9"

# Mermaid draws the two diagrams in the wiki. Pinned, and checked by the browser against this hash, which was
# compared with the file in npm's own package for this version.
MERMAID_URL = "https://cdn.jsdelivr.net/npm/mermaid@12.2.0/dist/mermaid.min.js"
MERMAID_SRI = "sha384-8xUhuLNMjW/cI2Dacg3DR4WliYznvIg0LVG1stctThKD/YVrJFI9TNCFxft5NYgp"

# The only other hosts a page may load anything from, as its Content-Security-Policy says. The wiki's screenshots come
# from the repository; Mermaid comes from jsDelivr.
ALLOWED_SOURCES = ("https://raw.githubusercontent.com/", "https://cdn.jsdelivr.net/npm/")

# A search result shows about this many characters of a description.
DESCRIPTION_MAX = 160
DESCRIPTION_MIN = 80

MERMAID_FENCE = re.compile(r"^```mermaid[ \t]*\r?\n(.*?)^```[ \t]*$", re.MULTILINE | re.DOTALL)
MERMAID_PLACEHOLDER = "WAMMERMAIDBLOCK{}"
MERMAID_PARAGRAPH = re.compile(r"<p[^>]*>WAMMERMAIDBLOCK(\d+)</p>")
FIRST_H1 = re.compile(r"<h1[^>]*>(.*?)</h1>", re.DOTALL)
PARAGRAPH = re.compile(r"<p\b[^>]*>(.*?)</p>", re.DOTALL)
TAG = re.compile(r"<[^>]+>")


class BuildError(Exception):
    pass


def token() -> str:
    value = os.environ.get("GITHUB_TOKEN") or os.environ.get("GH_TOKEN")
    if not value:
        raise BuildError("set GITHUB_TOKEN or GH_TOKEN: GitHub's API renders the pages")
    return value


def github(path: str, body: dict | None = None) -> bytes:
    request = urllib.request.Request(
        API + path,
        data=None if body is None else json.dumps(body).encode("utf-8"),
        method="GET" if body is None else "POST",
        headers={
            "Accept": "application/vnd.github+json",
            "Authorization": f"Bearer {token()}",
            "X-GitHub-Api-Version": "2022-11-28",
            "User-Agent": "winappmigrator-site-build",
        },
    )
    try:
        with urllib.request.urlopen(request, timeout=60) as response:
            return response.read()
    except urllib.error.HTTPError as error:
        detail = error.read().decode("utf-8", "replace")[:500]
        raise BuildError(f"GitHub API {path} answered {error.code}: {detail}") from error


def render_markdown(text: str) -> str:
    """Render as GitHub renders a file in a repository or a wiki page: no hard line breaks, no issue links."""
    blocks: list[str] = []

    def hold(match: re.Match[str]) -> str:
        blocks.append(match.group(1))
        return MERMAID_PLACEHOLDER.format(len(blocks) - 1)

    text = MERMAID_FENCE.sub(hold, text)
    if "```mermaid" in text:
        raise BuildError("a ```mermaid fence that is not at the start of its line would show as code, not a diagram")
    rendered = github("/markdown", {"text": text, "mode": "markdown"}).decode("utf-8")
    # GitHub prefixes every anchor so a page cannot clash with its own; the wiki's links are written without it.
    rendered = rendered.replace('id="user-content-', 'id="').replace('name="user-content-', 'name="')
    rendered = rendered.replace('href="#user-content-', 'href="#')

    def diagram(match: re.Match[str]) -> str:
        return f'<pre class="mermaid">{html.escape(blocks[int(match.group(1))])}</pre>'

    rendered = MERMAID_PARAGRAPH.sub(diagram, rendered)
    if "WAMMERMAIDBLOCK" in rendered:
        raise BuildError("a Mermaid diagram was not put back where it was")
    return rendered


def plain(fragment: str) -> str:
    return " ".join(html.unescape(TAG.sub("", fragment)).split())


def describe(rendered: str) -> str:
    """The first paragraphs of a page, cut to what a search result shows."""
    text = ""
    for match in PARAGRAPH.finditer(rendered):
        words = plain(match.group(1))
        if words.endswith(":"):
            # It leads into a list the description leaves out.
            words = words[:-1] + "."
        if words:
            text = f"{text} {words}".strip()
        if len(text) >= DESCRIPTION_MIN:
            break
    if len(text) <= DESCRIPTION_MAX:
        return text
    cut = text[: DESCRIPTION_MAX - 1].rsplit(" ", 1)[0].rstrip(",;:")
    return cut + "…"


def git_date(repo: Path, path: str) -> dt.datetime | None:
    result = subprocess.run(
        ["git", "-C", str(repo), "log", "-1", "--format=%cI", "--", path],
        capture_output=True,
        text=True,
        check=True,
    )
    stamp = result.stdout.strip()
    return dt.datetime.fromisoformat(stamp) if stamp else None


def iso(moment: dt.datetime) -> str:
    return moment.astimezone(dt.timezone.utc).strftime("%Y-%m-%dT%H:%M:%S+00:00")


def fill(template: Path, **values: str) -> str:
    try:
        return string.Template(template.read_text(encoding="utf-8")).substitute(values)
    except (KeyError, ValueError) as error:
        raise BuildError(f"{template}: {error}") from error


class LinkCollector(HTMLParser):
    def __init__(self) -> None:
        super().__init__(convert_charrefs=True)
        self.ids: set[str] = set()
        self.links: list[tuple[str, str]] = []

    def handle_starttag(self, tag: str, attrs: list[tuple[str, str | None]]) -> None:
        for name, value in attrs:
            if value is None:
                continue
            if name == "id" or (name == "name" and tag == "a"):
                self.ids.add(value)
            elif name in ("href", "src"):
                self.links.append((name, value))
            elif name == "srcset":
                self.links.extend(
                    (name, part.strip().split(" ")[0]) for part in value.split(",") if part.strip()
                )


def check_links(out: Path, base_url: str) -> list[str]:
    # Names exactly as built, so a link in the wrong case, or one that climbs out of the site, fails here as it would
    # on GitHub Pages, and not only on a case-sensitive disk.
    files = {path.relative_to(out).as_posix() for path in out.rglob("*") if path.is_file()}
    pages: dict[str, LinkCollector] = {}
    for name in sorted(f for f in files if f.endswith(".html") and "/" not in f):
        collector = LinkCollector()
        collector.feed((out / name).read_text(encoding="utf-8"))
        pages[name] = collector

    problems: list[str] = []
    for name, collector in pages.items():
        for attribute, link in collector.links:
            target = link
            if target.startswith(base_url + "/") or target == base_url:
                rest = target[len(base_url) :].lstrip("/")
                target = rest if rest and not rest.startswith("#") else "index.html" + rest
            elif re.match(r"^[a-z][a-z0-9+.-]*:|^//", target, re.IGNORECASE):
                if attribute != "href" and not target.startswith(ALLOWED_SOURCES):
                    problems.append(f"{name}: {link} is loaded from a host the pages' Content-Security-Policy blocks")
                continue
            path, _, fragment = target.partition("#")
            path = path.split("?", 1)[0]
            if path == "./":
                path = "index.html"
            if not path:
                page: str | None = name
            elif f"{path}.html" in files:
                page = f"{path}.html"
            elif path in files:
                page = path if path in pages else None
            else:
                problems.append(f"{name}: {link} leads to no page or file")
                continue
            if fragment and page is not None and fragment not in pages[page].ids:
                problems.append(f"{name}: {link} leads to no heading or anchor in {page}")
    return problems


def read_sitemap(path: Path | None) -> dict[str, str]:
    if path is None or not path.is_file() or path.stat().st_size == 0:
        return {}
    try:
        root = ET.parse(path).getroot()
    except ET.ParseError:
        return {}
    entries: dict[str, str] = {}
    for url in root.findall(f"{{{SITEMAP_NS}}}url"):
        loc = url.findtext(f"{{{SITEMAP_NS}}}loc")
        if loc:
            entries[loc.strip()] = (url.findtext(f"{{{SITEMAP_NS}}}lastmod") or "").strip()
    return entries


def write_sitemap(path: Path, entries: dict[str, str]) -> None:
    lines = ['<?xml version="1.0" encoding="UTF-8"?>', f'<urlset xmlns="{SITEMAP_NS}">']
    for loc, lastmod in entries.items():
        lines.append(f"  <url><loc>{html.escape(loc)}</loc><lastmod>{lastmod}</lastmod></url>")
    lines.append("</urlset>")
    path.write_text("\n".join(lines) + "\n", encoding="utf-8")


def build(args: argparse.Namespace) -> None:
    root: Path = args.repo_root
    wiki: Path = args.wiki
    out: Path = args.out
    source = root / "site"
    base_url = args.base_url.rstrip("/")

    if not (wiki / "Home.md").is_file():
        raise BuildError(f"{wiki} has no Home.md: is it a clone of the wiki?")
    # Read first: the live sitemap may have been saved where the new site is about to go.
    before = read_sitemap(args.old_sitemap)

    release = json.loads(github(f"/repos/{args.repository}/releases/latest"))
    version = release["tag_name"].removeprefix("v")
    released = dt.datetime.fromisoformat(release["published_at"].replace("Z", "+00:00"))

    if out.exists():
        shutil.rmtree(out)
    (out / "images").mkdir(parents=True)

    for item in source.iterdir():
        if item.is_file() and item.suffix in (".css", ".js", ".txt"):
            shutil.copy2(item, out / item.name)
    for image in sorted((root / "docs" / "images").glob("*.png")):
        shutil.copy2(image, out / "images" / image.name)
    shutil.copy2(root / "crates" / "winappmigrator-ui" / "assets" / "winappmigrator.png", out / "icon.png")
    shutil.copy2(root / "crates" / "winappmigrator-ui" / "assets" / "winappmigrator.ico", out / "favicon.ico")

    sidebar = render_markdown((wiki / "_Sidebar.md").read_text(encoding="utf-8"))
    footer = render_markdown((wiki / "_Footer.md").read_text(encoding="utf-8"))
    common = {
        "base_url": base_url,
        "version": html.escape(version),
        "release_date": released.date().isoformat(),
        "repository": args.repository,
    }

    sitemap: dict[str, str] = {}
    landing_changed = git_date(root, "site/index.html") or dt.datetime.now(dt.timezone.utc)
    sitemap[base_url + "/"] = iso(max(landing_changed, released))
    (out / "index.html").write_text(fill(source / "index.html", **common), encoding="utf-8")
    (out / "404.html").write_text(fill(source / "404.html", **common), encoding="utf-8")

    pages = sorted(p for p in wiki.glob("*.md") if not p.name.startswith("_"))
    for page in pages:
        name = page.stem
        rendered = render_markdown(page.read_text(encoding="utf-8"))
        heading = FIRST_H1.search(rendered)
        if heading is None:
            raise BuildError(f"{page.name} has no # heading to title it")
        title = plain(heading.group(1))
        title = "WinAppMigrator documentation" if title == "WinAppMigrator" else f"{title} – WinAppMigrator"
        description = describe(rendered)
        if not description:
            raise BuildError(f"{page.name} has no paragraph to describe it")
        changed = git_date(wiki, page.name) or dt.datetime.now(dt.timezone.utc)
        url = f"{base_url}/{urllib.parse.quote(name)}"
        current = f'href="{html.escape(name)}"'
        nav = sidebar.replace(current, f'{current} aria-current="page"')
        scripts = '<script defer src="site.js"></script>'
        if 'class="mermaid"' in rendered:
            scripts = (
                f'<script defer src="{MERMAID_URL}" integrity="{MERMAID_SRI}" crossorigin="anonymous"></script>\n'
                f"  {scripts}"
            )
        document = fill(
            source / "page.html",
            **common,
            title=html.escape(title),
            description=html.escape(description),
            canonical=html.escape(url),
            content=rendered,
            sidebar=nav,
            footer=footer,
            scripts=scripts,
            updated=changed.date().isoformat(),
        )
        (out / f"{name}.html").write_text(document, encoding="utf-8")
        sitemap[url] = iso(changed)

    write_sitemap(out / "sitemap.xml", sitemap)

    problems = check_links(out, base_url)
    if problems:
        raise BuildError("broken links:\n  " + "\n  ".join(problems))

    if args.changed_out is not None:
        changed_urls = [loc for loc, lastmod in sitemap.items() if before.get(loc) != lastmod]
        args.changed_out.write_text("".join(f"{url}\n" for url in changed_urls), encoding="utf-8")
        print(f"{len(changed_urls)} of {len(sitemap)} pages are new or changed since the live site")

    print(f"Built {len(pages)} wiki pages and the landing page for v{version} into {out}")


def main() -> int:
    here = Path(__file__).resolve()
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--wiki", type=Path, required=True, help="a clone of the wiki, with its history")
    parser.add_argument("--out", type=Path, required=True, help="where to write the site; emptied first")
    parser.add_argument("--repo-root", type=Path, default=here.parents[2])
    parser.add_argument("--repository", default=os.environ.get("GITHUB_REPOSITORY", DEFAULT_REPOSITORY))
    parser.add_argument("--base-url", default=DEFAULT_BASE_URL)
    parser.add_argument("--old-sitemap", type=Path, help="the live site's sitemap.xml, to tell what changed")
    parser.add_argument("--changed-out", type=Path, help="write the URLs that are new or changed here")
    args = parser.parse_args()
    try:
        build(args)
    except BuildError as error:
        print(f"error: {error}", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    sys.exit(main())
