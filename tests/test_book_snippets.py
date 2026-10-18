"""Run with python3 -m unittest discover -s tests -p 'test_book_snippets.py'."""

import html
import importlib.util
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile
import unittest
from unittest.mock import patch


SPEC = importlib.util.spec_from_file_location(
    "check_book_snippets", Path(__file__).resolve().parents[1] / "scripts/check_book_snippets.py")
checker = importlib.util.module_from_spec(SPEC)
sys.modules[SPEC.name] = checker
SPEC.loader.exec_module(checker)


def block(code="fn value() -> u8 { 1 }", suite=None, metadata="", classes="language-rust editable"):
    rendered = f'<pre class="playground"><code class="{classes}">{html.escape(code)}</code></pre>'
    if suite is not None:
        rendered += f'<script type="text/plain" class="playground-tests" {metadata}>{suite}</script>'
    return rendered


def extract(contents, require=True):
    return checker.extract_snippets(checker.parse_html(f"<main>{contents}</main>"), "fixture.html", require)


class ParserTests(unittest.TestCase):
    def test_inline_entities_and_raw_script_exact_source(self):
        code = 'fn text() -> &\'static str { "<&>" }\n'
        suite = '\n #[test] fn check() { assert_eq!(text(), "<&>"); }\n '
        snippets, errors = extract(block(code, suite))
        self.assertEqual(errors, [])
        self.assertEqual(snippets[0].source, code + "\n\n" + suite.strip() + "\n")
        self.assertEqual(snippets[0].edition, "2021")
        snippets, _ = extract(block("", 'fn raw() { let s = "&amp;"; }'))
        self.assertIn('"&amp;"', snippets[0].suite)

    def test_hidden_markup_is_text_not_doctest_rewritten(self):
        markup = '<pre class="playground"><code class="editable edition2024"><span class="boring">fn hidden() {}\n</span>#[test]\nfn visible() {}</code></pre>'
        snippets, errors = extract(markup, False)
        self.assertEqual(errors, [])
        self.assertEqual(snippets[0].code, "fn hidden() {}\n#[test]\nfn visible() {}")
        self.assertEqual(snippets[0].edition, "2024")

    def test_two_suites_do_not_cross_associate(self):
        snippets, errors = extract(block("fn a() {}", "suite a") + block("fn b() {}", "suite b"))
        self.assertEqual(errors, [])
        self.assertEqual([s.suite for s in snippets], ["suite a", "suite b"])

    def test_parent_sibling_and_comments_match_browser(self):
        markup = '<div>' + block() + '</div><!-- comment --> text <script class="playground-tests"> tests </script>'
        snippets, errors = extract(markup)
        self.assertEqual(errors, [])
        self.assertEqual(snippets[0].suite, "tests")

    def test_element_interrupts_association(self):
        snippets, errors = extract(block() + '<p>gap</p><script class="playground-tests">tests</script>')
        self.assertIsNone(snippets[0].suite)
        self.assertTrue(any("missing" in e for e in errors))
        self.assertTrue(any("orphan" in e for e in errors))

    def test_duplicate_orphan_fallback_and_unknown_scripts(self):
        _, errors = extract(block(suite="one") + '<script class="playground-tests">two</script>'
                            '<script class="playground-tests-fallback">fallback</script>'
                            '<script class="playground-tests-other">unknown</script>')
        self.assertEqual(len(errors), 3)
        self.assertTrue(any("orphan/duplicate" in e for e in errors))

    def test_shared_wrapper_suite_is_rejected(self):
        _, errors = extract('<div>' + block() + block() + '</div><script class="playground-tests">tests</script>')
        self.assertTrue(any("duplicate suite association" in e for e in errors))

    def test_direct_suite_precedes_wrapper_suite(self):
        snippets, errors = extract('<div>' + block(suite="direct") + '</div><script class="playground-tests">wrapper</script>')
        self.assertEqual(snippets[0].suite, "direct")
        self.assertTrue(any("orphan" in e for e in errors))

    def test_missing_suite_policy_and_noneditable(self):
        snippets, errors = extract(block())
        self.assertIn("missing", errors[0])
        self.assertIsNone(snippets[0].suite)
        self.assertEqual(extract(block(), False)[1], [])
        self.assertEqual(extract(block(classes="language-rust")), ([], []))

    def test_only_main_is_checked(self):
        root = checker.parse_html(block(suite="outside") + "<main>" + block(suite="inside") + "</main>")
        snippets, errors = checker.extract_snippets(root, "fixture.html")
        self.assertEqual(errors, [])
        self.assertEqual([s.suite for s in snippets], ["inside"])

    def test_metadata_and_duplicate_names(self):
        snippets, errors = extract(block(suite="tests", metadata='data-expected-failures="tests::a\n tests::b"'))
        self.assertEqual(errors, [])
        self.assertEqual(snippets[0].expected, {"tests::a", "tests::b"})
        self.assertTrue(extract(block(suite="tests", metadata='data-expected-failures="a a"'))[1])

    def test_book_aliases_nested_index_and_empty_build(self):
        with tempfile.TemporaryDirectory() as temp:
            directory = Path(temp)
            self.assertTrue(checker.load_book(directory)[1])
            page = "<main>" + block(suite="tests") + "</main>"
            for name in ("index.html", "first.html", "print.html"):
                (directory / name).write_text(page)
            (directory / "chapter").mkdir()
            (directory / "chapter/index.html").write_text(page)
            snippets, errors = checker.load_book(directory)
            self.assertEqual(errors, [])
            self.assertEqual(len(snippets), 2)
            (directory / "index.html").write_text("<main>unique" + block(suite="tests") + "</main>")
            self.assertEqual(len(checker.load_book(directory)[0]), 3)

    def test_timeout_is_actionable(self):
        with patch.object(checker.subprocess, "run", side_effect=subprocess.TimeoutExpired("rustc", 1)):
            with self.assertRaisesRegex(checker.CheckError, "timed out"):
                checker.command(["rustc"], 1)


@unittest.skipUnless(shutil.which("rustc"), "rustc required for integration fixtures")
class RustTests(unittest.TestCase):
    def check(self, suite, code="fn value() -> u8 { 1 }", expected=()):
        return checker.check_snippet(checker.Snippet("fixture:1", code, "2021", suite, frozenset(expected)), 20)

    def test_passing_suite_and_library(self):
        self.assertEqual(self.check('#[test] fn check() { assert_eq!(value(), 1); }'), 1)
        self.assertEqual(self.check(None), 0)

    def test_should_panic_is_a_passing_test(self):
        self.assertEqual(self.check('#[test] #[should_panic] fn panics() { panic!("expected"); }'), 1)

    def test_compiled_api_mismatch_never_expected(self):
        with self.assertRaisesRegex(checker.CheckError, "compilation failed"):
            self.check('#[test] fn check() { value(2); }', expected=["check"])

    def test_expected_failure_exact_names(self):
        suite = 'mod tests { #[test] fn fails() { assert!(false); } #[test] fn passes() {} }'
        self.assertEqual(self.check(suite, expected=["tests::fails"]), 2)
        with self.assertRaisesRegex(checker.CheckError, "unexpected failures"):
            self.check(suite)
        with self.assertRaisesRegex(checker.CheckError, "unexpected successes.*tests::passes"):
            self.check(suite, expected=["tests::fails", "tests::passes"])
        with self.assertRaisesRegex(checker.CheckError, "unknown expected failure"):
            self.check(suite, expected=["fails"])

    def test_ignored_and_zero_tests(self):
        with self.assertRaisesRegex(checker.CheckError, "ignored tests"):
            self.check('#[test] #[ignore] fn skipped() {}')
        with self.assertRaisesRegex(checker.CheckError, "zero tests"):
            self.check("")

    def test_early_success_exit_is_not_a_passing_suite(self):
        with self.assertRaisesRegex(checker.CheckError, "not all listed tests|summary"):
            self.check('#[test] fn exits() { std::process::exit(0); }')


if __name__ == "__main__":
    unittest.main()
