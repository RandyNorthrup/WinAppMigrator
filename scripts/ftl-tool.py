"""Checks and tidies WinAppMigrator's Fluent catalogs against English. Needs `pip install fluent.syntax`.

  python scripts/ftl-tool.py check crates/winappmigrator-i18n/locales <tag> [--file NAME]
  python scripts/ftl-tool.py merge crates/winappmigrator-i18n/locales <tag> [--file NAME]

check compares <tag>'s files with en-US, as the Rust catalog test does, and also warns of plural forms the language
needs and of text still in English. It exits 1 on a problem that would fail that test.

merge rewrites <tag>'s files in English's order under English's comments, dropping what English lacks. A draft may hold
only `id = text` messages, in `<tag>/<name>.ftl` or in pieces `<tag>/drafts/<stem>.<n>.ftl`, which merge then deletes.
"""

from __future__ import annotations

import argparse
import re
import sys
from pathlib import Path

from fluent.syntax import FluentParser, FluentSerializer, ast

ENGLISH = "en-US"

# CLDR cardinal categories, the union of old and new data so either fluent-bundle's or a newer one is accepted.
PLURALS = {
    "en": {"one", "other"},
    "es": {"one", "many", "other"},
    "pt": {"one", "many", "other"},
    "fr": {"one", "many", "other"},
    "de": {"one", "other"},
    "it": {"one", "many", "other"},
    "nl": {"one", "other"},
    "pl": {"one", "few", "many", "other"},
    "ru": {"one", "few", "many", "other"},
    "uk": {"one", "few", "many", "other"},
    "tr": {"one", "other"},
    "zh": {"other"},
    "ja": {"other"},
    "ko": {"other"},
    "hi": {"one", "other"},
    "bn": {"one", "other"},
    "id": {"other"},
    "vi": {"other"},
    "th": {"other"},
    "ar": {"zero", "one", "two", "few", "many", "other"},
    "fa": {"one", "other"},
    "ur": {"one", "other"},
    "he": {"one", "two", "many", "other"},
}
# Forms a count of files or things reaches and the grammar needs; "many" in es/fr/it/pt is only for millions.
NEEDED = {
    "pl": {"one", "few", "many"},
    "ru": {"one", "few", "many"},
    "uk": {"one", "few", "many"},
    "ar": {"zero", "one", "two", "few", "many"},
}
# Words that are the same in every language and may stand alone as a message.
SAME_EVERYWHERE = re.compile(r"^[\s\W\d]*$|WinAppMigrator|SHA-256|PowerShell|winget|\.NET|Visual C\+\+|Hyper-V|GitHub")


def parse(path: Path) -> ast.Resource:
    return FluentParser(with_spans=False).parse(path.read_text(encoding="utf-8"))


def variables(node, found: set[str]) -> set[str]:
    if isinstance(node, ast.VariableReference):
        found.add(node.id.name)
    elif isinstance(node, ast.BaseNode):
        for value in vars(node).values():
            if isinstance(value, list):
                for item in value:
                    variables(item, found)
            elif isinstance(value, ast.BaseNode):
                variables(value, found)
    return found


def selects(node, found: list) -> list:
    if isinstance(node, ast.SelectExpression):
        found.append(node)
    if isinstance(node, ast.BaseNode):
        for value in vars(node).values():
            if isinstance(value, list):
                for item in value:
                    selects(item, found)
            elif isinstance(value, ast.BaseNode):
                selects(value, found)
    return found


def messages(resource: ast.Resource) -> dict[str, ast.Message]:
    return {entry.id.name: entry for entry in resource.body if isinstance(entry, ast.Message)}


def parts(message: ast.Message) -> dict[str, ast.Pattern | None]:
    found = {"": message.value}
    for attribute in message.attributes:
        found[attribute.id.name] = attribute.value
    return found


def plain(pattern: ast.Pattern | None) -> str:
    if pattern is None:
        return ""
    words = []
    for element in pattern.elements:
        if isinstance(element, ast.TextElement):
            words.append(element.value)
        else:
            words.append("{}")
    return "".join(words).strip()


def check_file(english_path: Path, path: Path, tag: str) -> tuple[list[str], list[str]]:
    errors: list[str] = []
    warnings: list[str] = []
    name = path.name
    if not path.is_file():
        return [f"{name}: missing"], []
    resource = parse(path)
    for entry in resource.body:
        if isinstance(entry, ast.Junk):
            snippet = entry.content.strip().splitlines()[0][:100] if entry.content.strip() else ""
            errors.append(f"{name}: does not parse near: {snippet}")
    english = messages(parse(english_path))
    theirs = messages(resource)
    allowed = PLURALS.get(tag.split("-")[0], set())
    needed = NEEDED.get(tag.split("-")[0], set())
    untranslated = 0
    for message_id, message in english.items():
        translated = theirs.get(message_id)
        if translated is None:
            errors.append(f"{name}: {message_id} is missing")
            continue
        wanted, given = parts(message), parts(translated)
        for part, pattern in wanted.items():
            label = message_id if not part else f"{message_id}.{part}"
            if part not in given or (pattern is not None and given[part] is None):
                errors.append(f"{name}: {label} is missing")
                continue
            if pattern is None:
                continue
            want_vars, got_vars = variables(pattern, set()), variables(given[part], set())
            if want_vars != got_vars:
                errors.append(f"{name}: {label} uses {sorted(got_vars)}, not {sorted(want_vars)}")
            for select in selects(given[part], []):
                keys = set()
                defaults = 0
                for variant in select.variants:
                    defaults += variant.default
                    key = variant.key
                    if isinstance(key, ast.Identifier):
                        keys.add(key.name)
                        if allowed and key.name not in allowed and not select_is_not_plural(select, english, message_id):
                            errors.append(f"{name}: {label} has [{key.name}], which {tag} has no use for; it has {sorted(allowed)}")
                if defaults != 1:
                    errors.append(f"{name}: {label} needs exactly one *[default] variant")
                if is_plural(select) and needed - keys and keys & PLURALS["en"]:
                    warnings.append(f"{name}: {label} has no {sorted(needed - keys)} form, which {tag} needs for some counts")
            english_words = plain(pattern)
            if (
                plain(given[part]) == english_words
                and len(english_words.split()) >= 3
                and not SAME_EVERYWHERE.search(english_words)
            ):
                untranslated += 1
                if untranslated <= 15:
                    warnings.append(f"{name}: {label} is still English: {english_words[:80]}")
        for part in given:
            if part not in wanted:
                errors.append(f"{name}: {message_id}.{part} is not in English")
    for message_id in theirs:
        if message_id not in english:
            errors.append(f"{name}: {message_id} is not in English")
    if untranslated > 15:
        warnings.append(f"{name}: {untranslated} messages in all are still English")
    return errors, warnings


def is_plural(select: ast.SelectExpression) -> bool:
    keys = {variant.key.name for variant in select.variants if isinstance(variant.key, ast.Identifier)}
    return bool(keys & {"zero", "one", "two", "few", "many"}) or keys == {"other"}


def select_is_not_plural(select: ast.SelectExpression, english, message_id) -> bool:
    """A select on a word (a kind, a place) rather than a count: its keys are English's own."""
    keys = {variant.key.name for variant in select.variants if isinstance(variant.key, ast.Identifier)}
    return not bool(keys & {"zero", "one", "two", "few", "many"}) and keys != {"other"}


def drafts_of(path: Path) -> list[Path]:
    """Draft pieces for `<tag>/<name>.ftl`: `<tag>/drafts/<name-stem>.<anything>.ftl`, in name order."""
    folder = path.parent / "drafts"
    return sorted(folder.glob(f"{path.stem}.*.ftl")) if folder.is_dir() else []


def merge_file(english_path: Path, path: Path) -> list[str]:
    english = parse(english_path)
    draft = messages(parse(path)) if path.is_file() else {}
    pieces = drafts_of(path)
    for piece in pieces:
        resource = parse(piece)
        for entry in resource.body:
            if isinstance(entry, ast.Junk):
                snippet = entry.content.strip().splitlines()[0][:100] if entry.content.strip() else ""
                raise SystemExit(f"{piece.name} does not parse near: {snippet}")
        draft.update(messages(resource))
    missing = []
    body = []
    for entry in english.body:
        if isinstance(entry, ast.Message):
            translated = draft.get(entry.id.name)
            if translated is None:
                missing.append(entry.id.name)
                continue
            translated.comment = entry.comment
            body.append(translated)
        elif isinstance(entry, ast.Junk):
            raise SystemExit(f"{english_path} does not parse")
        else:
            body.append(entry)
    text = FluentSerializer(with_junk=False).serialize(ast.Resource(body))
    path.write_text(text, encoding="utf-8", newline="\n")
    for piece in pieces:
        piece.unlink()
    folder = path.parent / "drafts"
    if folder.is_dir() and not any(folder.iterdir()):
        folder.rmdir()
    return missing


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("action", choices=["check", "merge"])
    parser.add_argument("locales", type=Path)
    parser.add_argument("tag")
    parser.add_argument("--file")
    args = parser.parse_args()
    english_dir = args.locales / ENGLISH
    here = args.locales / args.tag
    names = [args.file] if args.file else sorted(path.name for path in english_dir.glob("*.ftl"))
    if args.action == "merge":
        for name in names:
            missing = merge_file(english_dir / name, here / name)
            print(f"{name}: merged" + (f", {len(missing)} still missing: {' '.join(missing[:20])}" if missing else ""))
        return 0
    all_errors, all_warnings = [], []
    for name in names:
        errors, warnings = check_file(english_dir / name, here / name, args.tag)
        all_errors += errors
        all_warnings += warnings
    extra = sorted(path.name for path in here.glob("*.ftl") if path.name not in {p.name for p in english_dir.glob("*.ftl")})
    all_errors += [f"{name}: not a file English has" for name in extra if not args.file]
    for line in all_errors:
        print("ERROR", line)
    for line in all_warnings:
        print("WARN ", line)
    print(f"{args.tag}: {len(all_errors)} errors, {len(all_warnings)} warnings")
    return 1 if all_errors else 0


if __name__ == "__main__":
    sys.exit(main())
