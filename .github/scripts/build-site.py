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
import posixpath
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
DEFAULT_BASE_URL = "https://winappmigrator.com"
SITEMAP_NS = "http://www.sitemaps.org/schemas/sitemap/0.9"

# Mermaid draws the two diagrams in the wiki. Pinned, and checked by the browser against this hash, which was
# compared with the file in npm's own package for this version.
MERMAID_URL = "https://cdn.jsdelivr.net/npm/mermaid@12.2.0/dist/mermaid.min.js"
MERMAID_SRI = "sha384-8xUhuLNMjW/cI2Dacg3DR4WliYznvIg0LVG1stctThKD/YVrJFI9TNCFxft5NYgp"

# The only other places a page may load anything from, as its Content-Security-Policy says. The wiki's screenshots come
# from the repository, and from jsDelivr only that one file of Mermaid's, since jsDelivr serves whatever anyone publishes.
ALLOWED_SOURCES = ("https://raw.githubusercontent.com/", MERMAID_URL)

# A search result shows about this many characters of a description.
DESCRIPTION_MAX = 160
DESCRIPTION_MIN = 80

# Translations, one folder per language in site/i18n: strings.json (the words of the page frame), index.html (the landing
# page), and these pages of the guide; the rest of the guide is in English. en-US holds only the English strings.
I18N = "i18n"
ENGLISH = "en-US"
TRANSLATED_PAGES = ("Getting-Started", "Installing-on-the-New-PC", "Troubleshooting")
RIGHT_TO_LEFT = ("ar", "fa", "ur", "he")
TRANSLATION_ISSUE = (
    "https://github.com/{repository}/issues/new?template=translation-problem.yml&labels=translation&language={tag}"
)
RELATIVE_URL = re.compile(r'(\s(?:href|src|srcset)=")([^"]*)(")')
SCHEME = re.compile(r"^[a-z][a-z0-9+.-]*:", re.IGNORECASE)
WORD_SLOT = re.compile(r"\{[a-z_]+\}")

MERMAID_FENCE = re.compile(r"^```mermaid[ \t]*\r?\n(.*?)^```[ \t]*$", re.MULTILINE | re.DOTALL)
MERMAID_PLACEHOLDER = "WAMMERMAIDBLOCK{}"
MERMAID_PARAGRAPH = re.compile(r"<p[^>]*>WAMMERMAIDBLOCK(\d+)</p>")
FIRST_H1 = re.compile(r"<h1[^>]*>(.*?)</h1>", re.DOTALL)
PARAGRAPH = re.compile(r"<p\b[^>]*>(.*?)</p>", re.DOTALL)
TAG = re.compile(r"<[^>]+>")
CODE_LANGUAGE = re.compile(r'<pre lang="([^"]*)"')
CODE_BLOCK = re.compile(r"<pre(?=[\s>])(?![^>]*\btabindex=)")


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
    # A code block's language is a programming language, not one a screen reader can speak in, so it is not given as
    # lang; and a block that scrolls sideways has to take the keyboard's focus to be scrolled without a mouse.
    rendered = CODE_LANGUAGE.sub(r'<pre data-lang="\1"', rendered)
    rendered = CODE_BLOCK.sub('<pre tabindex="0"', rendered)

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


def language_of(tag: str) -> str:
    return tag.split("-", 1)[0]


def direction_of(tag: str) -> str:
    return "rtl" if language_of(tag) in RIGHT_TO_LEFT else "ltr"


def read_strings(source: Path, tag: str, english: dict[str, str] | None) -> dict[str, str]:
    """A language's words for the page frame, checked against English's: the same keys, the same {slots}."""
    path = source / I18N / tag / "strings.json"
    try:
        strings = json.loads(path.read_text(encoding="utf-8"))
    except (OSError, ValueError) as error:
        raise BuildError(f"{path}: {error}") from error
    if not isinstance(strings, dict) or not all(isinstance(v, str) for v in strings.values()):
        raise BuildError(f"{path}: must map names to text")
    if english is not None:
        if set(strings) != set(english):
            raise BuildError(f"{path}: {sorted(set(strings) ^ set(english))} differ from {ENGLISH}")
        for key, value in english.items():
            if sorted(WORD_SLOT.findall(value)) != sorted(WORD_SLOT.findall(strings[key])):
                raise BuildError(f"{path}: {key} must keep {WORD_SLOT.findall(value)}")
    return strings


def fill_words(text: str, **values: str) -> str:
    for name, value in values.items():
        text = text.replace("{" + name + "}", value)
    return text


def translations(source: Path) -> list[str]:
    folder = source / I18N
    if not folder.is_dir():
        return []
    return sorted(p.name for p in folder.iterdir() if p.is_dir() and p.name != ENGLISH)


def relocate(document: str, kept: set[str]) -> str:
    """A page one folder down keeps its links to the pages translated beside it; everything else, the English pages, the
    stylesheet, and the images, is up a folder."""

    def fix(match: re.Match[str]) -> str:
        url = match.group(2)
        if not url or url == "./" or url.startswith(("#", "../", "/")) or SCHEME.match(url):
            return match.group(0)
        if url.split("#", 1)[0].split("?", 1)[0] in kept:
            return match.group(0)
        return f"{match.group(1)}../{url}{match.group(3)}"

    return RELATIVE_URL.sub(fix, document)


def page_url(base_url: str, tag: str, page: str | None) -> str:
    folder = "" if tag == ENGLISH else f"{tag}/"
    return f"{base_url}/{folder}{'' if page is None else urllib.parse.quote(page)}"


def alternate_links(base_url: str, page: str | None, tags: list[str]) -> str:
    """Where the same page is in each other language, for search engines. Nothing for a page in English alone."""
    if len(tags) < 2:
        return ""
    links = [
        f'  <link rel="alternate" hreflang="{tag}" href="{html.escape(page_url(base_url, tag, page))}">' for tag in tags
    ]
    links.append(f'  <link rel="alternate" hreflang="x-default" href="{html.escape(page_url(base_url, ENGLISH, page))}">')
    return "\n".join(links)


def language_menu(current: str, page: str | None, tags: list[str], names: dict[str, str], label: str) -> str:
    """Every language the page is in, each named in itself and marked as that language, as links that need no script."""
    if len(tags) < 2:
        return ""
    up = "" if current == ENGLISH else "../"
    items = []
    for tag in tags:
        target = ("" if tag == ENGLISH else f"{tag}/") + ("" if page is None else page)
        href = up + target if (up + target) else "./"
        here = ' aria-current="page"' if tag == current else ""
        items.append(
            f'          <li><a href="{html.escape(href)}" lang="{language_of(tag)}" hreflang="{tag}"'
            f' dir="{direction_of(tag)}"{here}>{html.escape(names[tag])}</a></li>'
        )
    return (
        f'        <details class="languages"><summary>{html.escape(label)}</summary><ul>\n'
        + "\n".join(items)
        + "\n        </ul></details>"
    )


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
    # on the web host, and not only on a case-sensitive disk.
    files = {path.relative_to(out).as_posix() for path in out.rglob("*") if path.is_file()}
    pages: dict[str, LinkCollector] = {}
    for name in sorted(f for f in files if f.endswith(".html")):
        collector = LinkCollector()
        collector.feed((out / name).read_text(encoding="utf-8"))
        pages[name] = collector

    problems: list[str] = []
    for name, collector in pages.items():
        folder = posixpath.dirname(name)
        for attribute, link in collector.links:
            target = link
            base = folder
            if target.startswith(base_url + "/") or target == base_url:
                rest = target[len(base_url) :].lstrip("/")
                target = rest if rest and not rest.startswith("#") else "./" + rest
                base = ""
            elif re.match(r"^[a-z][a-z0-9+.-]*:|^//", target, re.IGNORECASE):
                if attribute != "href" and not target.startswith(ALLOWED_SOURCES):
                    problems.append(f"{name}: {link} is loaded from a host the pages' Content-Security-Policy blocks")
                continue
            path, _, fragment = target.partition("#")
            path = path.split("?", 1)[0]
            if not path:
                page: str | None = name
            else:
                joined = posixpath.normpath(posixpath.join(base, path))
                if joined == ".." or joined.startswith("../") or joined.startswith("/"):
                    problems.append(f"{name}: {link} leads out of the site")
                    continue
                folder_index = "index.html" if joined == "." else f"{joined}/index.html"
                if path.endswith("/") or joined == ".":
                    candidates = [folder_index]
                else:
                    candidates = [f"{joined}.html", joined, folder_index]
                found = next((candidate for candidate in candidates if candidate in files), None)
                if found is None:
                    problems.append(f"{name}: {link} leads to no page or file")
                    continue
                page = found if found in pages else None
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


# Search engines, and the crawlers of AI search and assistants, are all welcome: the site is the project's guide.
AI_CRAWLERS = (
    "GPTBot", "OAI-SearchBot", "ChatGPT-User", "ClaudeBot", "Claude-SearchBot", "Claude-User", "PerplexityBot",
    "Perplexity-User", "Google-Extended", "Applebot-Extended", "Amazonbot", "DuckAssistBot", "Meta-ExternalAgent",
    "MistralAI-User",
)


def write_robots(path: Path, base_url: str) -> None:
    lines = ["User-agent: *", "Allow: /", "", "# AI search and agent crawlers are welcome. Plain-text guides: "
             "/llms.txt and /llms-full.txt"]
    lines += [f"User-agent: {name}" for name in AI_CRAWLERS]
    lines += ["Allow: /", "", f"Sitemap: {base_url}/sitemap.xml", ""]
    path.write_text("\n".join(lines), encoding="utf-8")


def write_llms(
    out: Path,
    template: Path,
    common: dict[str, str],
    guide: list[tuple[str, str, str, str]],
    languages: list[str],
    names: dict[str, str],
) -> None:
    """llms.txt, what the project is and where its guide's pages are, and llms-full.txt, the whole guide as written."""
    summary = string.Template(template.read_text(encoding="utf-8")).substitute(
        {key: html.unescape(value) for key, value in common.items()}
    )
    pages = "\n".join(f"- [{title}]({url}): {description}" for title, url, description, _ in guide)
    spoken = ", ".join(names[tag] for tag in languages)
    (out / "llms.txt").write_text(
        f"{summary.rstrip()}\n\n## Languages\n\nThe app, its installers and the website's key pages are in: {spoken}.\n\n"
        f"## Guide\n\n{pages}\n",
        encoding="utf-8",
    )
    full = "\n\n".join(f"<!-- {url} -->\n\n{text.strip()}" for _, url, _, text in guide)
    (out / "llms-full.txt").write_text(f"{summary.rstrip()}\n\n{full}\n", encoding="utf-8")


# On the English guide's pages only: the translated pages are not cluttered with words in another language.
SAK_PROMO = """      <aside class="promo-strip" aria-label="From the same developer">
        <img src="sak-utility.png" alt="" width="48" height="48" loading="lazy">
        <p><strong>S.A.K. Utility</strong>, from the same developer: a free, portable toolkit for PC technicians, with
          profile backup, a partition manager, diagnostics and app deployment, and nothing to install.</p>
        <a class="button" href="https://sakutility.com/">Visit sakutility.com</a>
      </aside>
"""


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
        if item.is_file() and item.suffix in (".css", ".js", ".txt", ".png"):
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
        "mermaid_url": MERMAID_URL,
    }

    english_words = read_strings(source, ENGLISH, None)
    languages = translations(source)
    words = {ENGLISH: english_words} | {tag: read_strings(source, tag, english_words) for tag in languages}
    names = {tag: words[tag]["language_name"] for tag in words}
    for tag in languages:
        if not (source / I18N / tag / "index.html").is_file():
            raise BuildError(f"site/{I18N}/{tag} has no index.html, its landing page")
    # Which languages each translated page is in, English first.
    page_languages = {
        page: [ENGLISH] + [tag for tag in languages if (source / I18N / tag / f"{page}.md").is_file()]
        for page in TRANSLATED_PAGES
    }
    landing_languages = [ENGLISH, *languages]

    def frame(tag: str, updated: str, english_page: str) -> dict[str, str]:
        """The words of a page's frame in a language, with its notice when it is a translation."""
        said = words[tag]
        notice = ""
        if tag != ENGLISH:
            report = TRANSLATION_ISSUE.format(repository=args.repository, tag=tag)
            notice = '      <p class="mt-notice">' + fill_words(
                said["notice"], english=html.escape(english_page), report=html.escape(report)
            ) + "</p>"
        return {
            "lang": language_of(tag),
            "dir": direction_of(tag),
            "notice": notice,
            "s_skip": html.escape(said["s_skip"]),
            "s_site": html.escape(said["s_site"]),
            "s_docs": html.escape(said["s_docs"]),
            "s_download": html.escape(said["s_download"]),
            "s_all_pages": html.escape(said["s_all_pages"]),
            "s_documentation": html.escape(said["s_documentation"]),
            "s_updated": fill_words(html.escape(said["s_updated"]), date=updated, version=html.escape(version)),
            "s_built": fill_words(
                said["s_built"],
                wiki=f"https://github.com/{args.repository}/wiki",
                license=f"https://github.com/{args.repository}/blob/main/LICENSE",
            ),
        }

    sitemap: dict[str, str] = {}
    landing_changed = git_date(root, "site/index.html") or dt.datetime.now(dt.timezone.utc)
    sitemap[base_url + "/"] = iso(max(landing_changed, released))
    (out / "index.html").write_text(
        fill(
            source / "index.html",
            **common,
            languages=language_menu(ENGLISH, None, landing_languages, names, english_words["language"]),
            alternates=alternate_links(base_url, None, landing_languages),
        ),
        encoding="utf-8",
    )
    (out / "404.html").write_text(fill(source / "404.html", **common), encoding="utf-8")

    def scripts_for(rendered: str) -> str:
        scripts = '<script defer src="site.js"></script>'
        if 'class="mermaid"' in rendered:
            scripts = (
                f'<script defer src="{MERMAID_URL}" integrity="{MERMAID_SRI}" crossorigin="anonymous"></script>\n'
                f"  {scripts}"
            )
        return scripts

    def titled(rendered: str, where: str) -> tuple[str, str]:
        heading = FIRST_H1.search(rendered)
        if heading is None:
            raise BuildError(f"{where} has no # heading to title it")
        description = describe(rendered)
        if not description:
            raise BuildError(f"{where} has no paragraph to describe it")
        return plain(heading.group(1)), description

    pages = sorted(p for p in wiki.glob("*.md") if not p.name.startswith("_"))
    guide: list[tuple[str, str, str, str]] = []
    for page in pages:
        name = page.stem
        rendered = render_markdown(page.read_text(encoding="utf-8"))
        heading, description = titled(rendered, page.name)
        title = "WinAppMigrator Documentation" if heading == "WinAppMigrator" else f"{heading} – WinAppMigrator"
        changed = git_date(wiki, page.name) or dt.datetime.now(dt.timezone.utc)
        url = page_url(base_url, ENGLISH, name)
        current = f'href="{html.escape(name)}"'
        nav = sidebar.replace(current, f'{current} aria-current="page"')
        in_languages = page_languages.get(name, [ENGLISH])
        document = fill(
            source / "page.html",
            **common,
            **frame(ENGLISH, changed.date().isoformat(), name),
            title=html.escape(title),
            description=html.escape(description),
            canonical=html.escape(url),
            alternates=alternate_links(base_url, name, in_languages),
            languages=language_menu(ENGLISH, name, in_languages, names, english_words["language"]),
            content=rendered,
            sidebar=nav,
            footer=footer,
            scripts=scripts_for(rendered),
            promo=SAK_PROMO,
        )
        (out / f"{name}.html").write_text(document, encoding="utf-8")
        sitemap[url] = iso(changed)
        guide.append((heading, url, description, page.read_text(encoding="utf-8")))

    for tag in languages:
        folder = out / tag
        folder.mkdir()
        here = source / I18N / tag
        translated = [page for page in TRANSLATED_PAGES if (here / f"{page}.md").is_file()]
        kept = set(translated)
        said = words[tag]
        landing = fill(
            here / "index.html",
            **common,
            languages=language_menu(tag, None, landing_languages, names, said["language"]),
            alternates=alternate_links(base_url, None, landing_languages),
        )
        (folder / "index.html").write_text(relocate(landing, kept), encoding="utf-8")
        landing_changed = git_date(root, f"site/{I18N}/{tag}/index.html") or dt.datetime.now(dt.timezone.utc)
        sitemap[page_url(base_url, tag, None)] = iso(max(landing_changed, released))

        rendered_pages = {page: render_markdown((here / f"{page}.md").read_text(encoding="utf-8")) for page in translated}
        titles = {page: titled(rendered, f"site/{I18N}/{tag}/{page}.md") for page, rendered in rendered_pages.items()}
        for page, rendered in rendered_pages.items():
            heading, description = titles[page]
            changed = git_date(root, f"site/{I18N}/{tag}/{page}.md") or dt.datetime.now(dt.timezone.utc)
            nav = "\n".join(
                f'        <a href="{html.escape(other)}"'
                + (' aria-current="page"' if other == page else "")
                + f">{html.escape(titles[other][0])}</a>"
                for other in translated
            )
            nav += f'\n        <a href="../Home" lang="en" hreflang="en">{html.escape(said["full_guide"])}</a>'
            document = fill(
                source / "page.html",
                **common,
                **frame(tag, changed.date().isoformat(), f"../{page}"),
                title=html.escape(f"{heading} – WinAppMigrator"),
                description=html.escape(description),
                canonical=html.escape(page_url(base_url, tag, page)),
                alternates=alternate_links(base_url, page, page_languages[page]),
                languages=language_menu(tag, page, page_languages[page], names, said["language"]),
                content=rendered,
                sidebar=nav,
                footer=footer,
                scripts=scripts_for(rendered),
                promo="",
            )
            (folder / f"{page}.html").write_text(relocate(document, kept), encoding="utf-8")
            sitemap[page_url(base_url, tag, page)] = iso(changed)

    write_sitemap(out / "sitemap.xml", sitemap)
    write_robots(out / "robots.txt", base_url)
    write_llms(out, source / "llms.txt.in", common, guide, landing_languages, names)

    problems = check_links(out, base_url)
    if problems:
        raise BuildError("broken links:\n  " + "\n  ".join(problems))

    if args.changed_out is not None:
        changed_urls = [loc for loc, lastmod in sitemap.items() if before.get(loc) != lastmod]
        args.changed_out.write_text("".join(f"{url}\n" for url in changed_urls), encoding="utf-8")
        print(f"{len(changed_urls)} of {len(sitemap)} pages are new or changed since the live site")

    print(
        f"Built {len(pages)} wiki pages and the landing page for v{version} into {out}, with {len(languages)} "
        "translations of the landing page and its key pages"
    )


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
