#!/usr/bin/env python3
"""Check the exact editable Rust snippets sent by the book's browser runner.

Usage: python3 scripts/check_book_snippets.py book/book
Build with `mdbook build book` first. Only Python's standard library and rustc
are required. No Cargo dependencies are injected. Feature-gated snippets use
`rustup run nightly rustc`, matching the browser's nightly selection.

Every editable block requires an adjacent suite by default. --allow-missing-suites
checks unattached blocks as libraries instead (the browser's compile-only mode).
Suites may declare whitespace-separated, fully qualified test names in
 data-expected-failures; compilation failures never count as expected failures.
All HTML pages are inspected, except print.html and the root index.html when its
main content duplicates another page. Nested index.html chapters are retained.
Rendered code text is used verbatim, including any mdBook hidden-line markup's
text: no doctest wrapping, hash stripping, include expansion, or source-file
substitution is performed. Script text is raw HTML text, not entity-decoded.

This executes trusted repository code, not a sandbox. Each compile/list/run has
a timeout and all generated Rust sources and binaries live in temporary dirs.
"""

import argparse
from dataclasses import dataclass, field
from html.parser import HTMLParser
from pathlib import Path
import re
import subprocess
import sys
import tempfile


# ECMAScript String.trim whitespace (Python strip additionally removes U+0085).
JS_WHITESPACE = "\u0009\u000a\u000b\u000c\u000d\u0020\u00a0\u1680\u2000\u2001\u2002\u2003\u2004\u2005\u2006\u2007\u2008\u2009\u200a\u2028\u2029\u202f\u205f\u3000\ufeff"
VOID_TAGS = set("area base br col embed hr img input link meta param source track wbr".split())


@dataclass(eq=False)
class Element:
    tag: str
    attrs: dict
    line: int
    parent: object = None
    children: list = field(default_factory=list)

    @property
    def classes(self):
        return self.attrs.get("class", "").split()

    @property
    def text(self):
        return "".join(child.text if isinstance(child, Element) else child
                       for child in self.children)

    def walk(self):
        yield self
        for child in self.children:
            if isinstance(child, Element):
                yield from child.walk()

    def next_element(self):
        if self.parent is None:
            return None
        siblings = self.parent.children
        return next((c for c in siblings[siblings.index(self) + 1:]
                     if isinstance(c, Element)), None)

    def fingerprint(self):
        return (self.tag, sorted(self.attrs.items()),
                [c.fingerprint() if isinstance(c, Element) else c for c in self.children])


class BookParser(HTMLParser):
    """Small DOM subset; script raw-text and ordinary entity decoding are native."""

    def __init__(self):
        super().__init__(convert_charrefs=True)
        self.root = Element("document", {}, 1)
        self.stack = [self.root]

    def handle_starttag(self, tag, attrs):
        node = Element(tag, {k: v or "" for k, v in attrs}, self.getpos()[0], self.stack[-1])
        self.stack[-1].children.append(node)
        if tag not in VOID_TAGS:
            self.stack.append(node)

    def handle_startendtag(self, tag, attrs):
        self.handle_starttag(tag, attrs)
        if tag not in VOID_TAGS:
            self.handle_endtag(tag)

    def handle_endtag(self, tag):
        for index in range(len(self.stack) - 1, 0, -1):
            if self.stack[index].tag == tag:
                del self.stack[index:]
                break

    def handle_data(self, data):
        self.stack[-1].children.append(data)


def parse_html(html):
    parser = BookParser()
    # Browsers normalize CRLF and CR before constructing text nodes.
    parser.feed(html.replace("\r\n", "\n").replace("\r", "\n"))
    parser.close()
    return parser.root


@dataclass
class Snippet:
    location: str
    code: str
    edition: str
    suite: str | None
    expected: frozenset = frozenset()

    @property
    def source(self):
        return self.code if self.suite is None else self.code + "\n\n" + self.suite + "\n"


def extract_snippets(root, filename, require_suites=True):
    snippets, errors = [], []
    nodes = [n for main in root.walk() if main.tag == "main" for n in main.walk()]
    scripts = [n for n in nodes if n.tag == "script" and "playground-tests" in n.classes]
    used = set()
    for node in nodes:
        if node.tag == "script" and any(c.startswith("playground-tests") and c != "playground-tests"
                                         for c in node.classes):
            errors.append(f"{filename}:{node.line}: unknown/fallback test script: {node.classes}")
    for pre in (n for n in nodes if n.tag == "pre" and "playground" in n.classes):
        code = next((n for n in pre.walk() if n.tag == "code"), None)
        if code is None or "editable" not in code.classes:
            continue
        location = f"{filename}:{pre.line}: editable block {len(snippets) + 1}"
        candidates = [pre.next_element(), pre.parent.next_element() if pre.parent else None]
        script = next((n for n in candidates if n is not None and n.tag == "script"
                       and "playground-tests" in n.classes), None)
        expected = []
        if script is not None:
            if script in used:
                errors.append(f"{location}: duplicate suite association (script line {script.line})")
            used.add(script)
            expected = script.attrs.get("data-expected-failures", "").split()
            if len(expected) != len(set(expected)):
                errors.append(f"{location}: duplicate expected failure names")
        elif require_suites:
            errors.append(f"{location}: missing adjacent script.playground-tests suite")
        edition = next((c[len("edition"):] for c in code.classes if c.startswith("edition")), "2021")
        snippets.append(Snippet(location, code.text, edition,
                                script.text.strip(JS_WHITESPACE) if script is not None else None,
                                frozenset(expected)))
    for script in scripts:
        if script not in used:
            errors.append(f"{filename}:{script.line}: orphan/duplicate playground-tests script")
    return snippets, errors


class CheckError(Exception):
    pass


def command(args, timeout):
    try:
        return subprocess.run(args, capture_output=True, text=True, timeout=timeout)
    except subprocess.TimeoutExpired as error:
        raise CheckError(f"command timed out after {timeout}s: {args}") from error
    except OSError as error:
        raise CheckError(f"cannot execute {args[0]}: {error}") from error


def output(result):
    return (result.stdout + result.stderr).strip()


def check_snippet(snippet, timeout=30):
    with tempfile.TemporaryDirectory(prefix="book-snippet-") as directory:
        source = Path(directory) / "snippet.rs"
        binary = Path(directory) / "snippet"
        source.write_text(snippet.source, encoding="utf-8")
        compiler = ["rustup", "run", "nightly", "rustc"] if "#![feature" in snippet.source else ["rustc"]
        compiled = command(compiler + [str(source), "--edition", snippet.edition,
                                      "--crate-name", "book_snippet",
                                      *(["--test"] if snippet.suite is not None else ["--crate-type", "lib"]),
                                      "-o", str(binary)], timeout)
        if compiled.returncode:
            raise CheckError("compilation failed (never an expected runtime failure):\n" + output(compiled))
        if snippet.suite is None:
            return 0
        listed = command([str(binary), "--list", "--format", "pretty"], timeout)
        if listed.returncode:
            raise CheckError("listing tests failed:\n" + output(listed))
        names = re.findall(r"^(.+): test$", listed.stdout, re.MULTILINE)
        if not names:
            raise CheckError("suite contains zero tests")
        if len(names) != len(set(names)):
            raise CheckError("suite lists duplicate test names")
        unknown = snippet.expected - set(names)
        if unknown:
            raise CheckError(f"unknown expected failure names: {sorted(unknown)}")
        result = command([str(binary), "--format", "pretty", "--color", "never", "--test-threads=1"], timeout)
        records = re.findall(r"^test (.+?)(?: - should panic)? \.\.\. (ok|FAILED|ignored)(?:[^\n]*)$", result.stdout, re.MULTILINE)
        if len(records) != len(names) or {name for name, _ in records} != set(names):
            raise CheckError("not all listed tests ran exactly once:\n" + output(result))
        ignored = {name for name, status in records if status == "ignored"}
        failed = {name for name, status in records if status == "FAILED"}
        if ignored:
            raise CheckError(f"ignored tests are not allowed: {sorted(ignored)}\n" + output(result))
        if failed != snippet.expected:
            raise CheckError(f"unexpected failures: {sorted(failed - snippet.expected)}; "
                             f"unexpected successes: {sorted(snippet.expected - failed)}\n" + output(result))
        expected_exit = 101 if failed else 0
        if result.returncode != expected_exit:
            raise CheckError(f"unexpected test process exit {result.returncode}, expected {expected_exit}:\n" + output(result))
        summaries = re.findall(r"^test result: (?:ok|FAILED)\. (\d+) passed; (\d+) failed; (\d+) ignored; (\d+) measured; (\d+) filtered out;", result.stdout, re.MULTILINE)
        if summaries != [(str(len(names) - len(failed)), str(len(failed)), "0", "0", "0")]:
            raise CheckError("missing or inconsistent test summary:\n" + output(result))
        return len(names)


def load_book(directory, require_suites=True):
    pages = {p: parse_html(p.read_text(encoding="utf-8")) for p in sorted(directory.rglob("*.html"))
             if p != directory / "print.html"}
    index = directory / "index.html"
    if index in pages:
        main = next((n for n in pages[index].walk() if n.tag == "main"), None)
        if main is not None and any(n.fingerprint() == main.fingerprint()
                                    for p, root in pages.items() if p != index
                                    for n in root.walk() if n.tag == "main"):
            del pages[index]
    snippets, errors = [], []
    for path, root in pages.items():
        found, problems = extract_snippets(root, path.relative_to(directory), require_suites)
        snippets.extend(found)
        errors.extend(problems)
    if not snippets:
        errors.append(f"{directory}: no editable playgrounds found; build the book first")
    return snippets, errors


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("output_dir", type=Path, help="built mdBook HTML directory, e.g. book/book")
    parser.add_argument("--timeout", type=float, default=30, help="seconds per compiler/test process (default: 30)")
    parser.add_argument("--allow-missing-suites", action="store_true", help="compile unattached blocks as libraries instead of rejecting them")
    args = parser.parse_args(argv)
    if args.timeout <= 0 or not args.timeout < float("inf"):
        parser.error("--timeout must be finite and positive")
    if not args.output_dir.is_dir():
        parser.error(f"not a directory: {args.output_dir}")
    try:
        snippets, errors = load_book(args.output_dir, not args.allow_missing_suites)
    except (OSError, UnicodeError) as error:
        print(f"ERROR: {error}", file=sys.stderr)
        return 1
    count = 0
    for snippet in snippets:
        try:
            tests = check_snippet(snippet, args.timeout)
            count += tests
            print(f"OK {snippet.location}: {str(tests) + ' tests' if snippet.suite is not None else 'library compiled'}")
        except CheckError as error:
            errors.append(f"{snippet.location}: {error}")
    for error in errors:
        print(f"ERROR {error}", file=sys.stderr)
    print(f"Checked {len(snippets)} editable blocks, {count} tests validated, {len(errors)} errors.")
    return int(bool(errors))


if __name__ == "__main__":
    sys.exit(main())
