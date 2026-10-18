'use strict';

// mdBook submits editable snippets as binaries and omits code hidden outside Ace.
// Exercise listings run the snippet plus explicitly attached, editable tests.
(() => {
    const initialized = Symbol.for('refactoring.playground-tests.initialized');
    if (document[initialized]) return;
    document[initialized] = true;

    const testEditors = new WeakMap();
    const editorOwners = new WeakMap();
    const playgroundUrl = 'https://play.rust-lang.org/execute';
    const activeRuns = new WeakMap();

    function editableCode(playground) {
        const code = playground.querySelector('code');
        if (!code || !code.classList.contains('editable')) {
            return null;
        }
        return window.ace ? window.ace.edit(code).getValue() : code.textContent;
    }

    function createTestEditor(playground) {
        const script = [playground, playground.parentElement]
            .map(element => element?.nextElementSibling)
            .find(element => element?.matches('script.playground-tests'));
        if (!script) return;

        const details = document.createElement('details');
        details.className = 'playground-test-editor';
        const summary = document.createElement('summary');
        summary.textContent = 'Tests and callers';
        const label = document.createElement('label');
        label.textContent = 'Edit tests and callers to match your API:';
        const textarea = document.createElement('textarea');
        textarea.value = script.textContent.trim();
        textarea.rows = Math.min(24, Math.max(6, textarea.value.split('\n').length));
        textarea.spellcheck = false;
        label.append(textarea);
        details.append(summary, label);
        playground.after(details);
        testEditors.set(playground, textarea);
        editorOwners.set(textarea, playground);
    }

    function editionFor(playground) {
        const code = playground.querySelector('code');
        for (const className of code.classList) {
            if (className.startsWith('edition')) {
                return className.slice('edition'.length);
            }
        }
        return '2021';
    }

    function resultBlock(playground) {
        let result = playground.querySelector('.result');
        if (!result) {
            result = document.createElement('code');
            result.className = 'result hljs';
            playground.append(result);
        }
        return result;
    }

    function highlightOutput(result) {
        const lines = result.textContent.split('\n');
        result.replaceChildren();

        function append(text, className) {
            const span = document.createElement('span');
            span.className = className;
            span.textContent = text;
            result.append(span);
            return span;
        }

        for (const [index, line] of lines.entries()) {
            if (index > 0) result.append('\n');
            const source = line.match(/^(\s*\d+\s*\| ?)(.*)$/);
            if (source) {
                append(source[1], 'diagnostic-location');
                const code = append(source[2], 'language-rust');
                if (window.hljs) window.hljs.highlightBlock(code);
            } else if (/^\s*(?:error(?:\[E\d+\])?:|test result: FAILED|.*\.\.\. FAILED$)/.test(line)) {
                append(line, 'diagnostic-error');
            } else if (/^\s*warning(?:\[.*?\])?:/.test(line)) {
                append(line, 'diagnostic-warning');
            } else if (/^\s*(?:--> |::: |\||= (?:note|help):|(?:note|help):)/.test(line)) {
                append(line, 'diagnostic-location');
            } else if (/^(?:Tests passed\.|Code compiles\.|test result: ok\.|test .* \.\.\. ok$)/.test(line)) {
                append(line, 'diagnostic-success');
            } else {
                result.append(line);
            }
        }
    }

    async function runOnPlayground(source, playground, runsTests, signal) {
        const response = await fetch(playgroundUrl, {
            headers: {
                'Content-Type': 'application/json',
            },
            method: 'POST',
            mode: 'cors',
            signal,
            body: JSON.stringify({
                channel: source.includes('#![feature') ? 'nightly' : 'stable',
                mode: 'debug',
                edition: editionFor(playground),
                crateType: runsTests ? 'bin' : 'lib',
                tests: runsTests,
                code: source,
                backtrace: false,
            }),
        });

        if (!response.ok) {
            throw new Error(`HTTP ${response.status}`);
        }

        return response.json();
    }

    function outputText(output) {
        const streams = output.success
            ? [output.stdout, output.stderr]
            : [output.stderr, output.stdout];
        return streams.filter(Boolean).join('\n').trim();
    }

    async function execute(playground) {
        if (activeRuns.has(playground)) {
            return;
        }
        const code = editableCode(playground);
        if (code === null) {
            return;
        }

        const textarea = testEditors.get(playground);
        const runsTests = Boolean(textarea);
        const result = resultBlock(playground);
        const button = playground.querySelector('.play-button');
        const controller = new AbortController();
        const timeout = window.setTimeout(() => controller.abort(), 20_000);
        activeRuns.set(playground, controller);

        result.textContent = runsTests ? 'Running tests...' : 'Checking code...';
        result.classList.remove('result-no-output');
        if (button) {
            button.disabled = true;
        }

        try {
            const output = await runOnPlayground(
                runsTests ? `${code}\n\n${textarea.value}\n` : code,
                playground,
                runsTests,
                controller.signal,
            );
            const text = outputText(output);

            if (output.success && runsTests) {
                result.textContent = text ? `Tests passed.\n\n${text}` : 'Tests passed.';
            } else {
                result.textContent = text || (output.success ? 'Code compiles.' : 'Execution failed.');
            }
            highlightOutput(result);
            result.classList.toggle('result-no-output', output.success && text === '');
        } catch (error) {
            const message = error.name === 'AbortError' ? 'request timed out' : error.message;
            result.textContent = `Playground communication: ${message}`;
            result.classList.remove('result-no-output');
        } finally {
            window.clearTimeout(timeout);
            if (activeRuns.get(playground) === controller) {
                activeRuns.delete(playground);
                if (button) {
                    button.disabled = false;
                }
            }
        }
    }

    function playgroundFromEvent(event) {
        const target = event.target instanceof Element ? event.target : null;
        const playground = editorOwners.get(target) || target?.closest('pre.playground');
        if (!playground || editableCode(playground) === null) {
            return null;
        }
        return playground;
    }

    document.querySelectorAll('pre.playground').forEach(playground => {
        if (editableCode(playground) === null) {
            return;
        }

        createTestEditor(playground);
        const button = playground.querySelector('.play-button');
        if (!button) {
            return;
        }

        const runsTests = testEditors.has(playground);
        button.title = runsTests ? 'Run tests' : 'Check code';
        button.setAttribute('aria-label', button.title);
    });

    document.addEventListener('click', event => {
        const target = event.target instanceof Element ? event.target : null;
        if (!target?.closest('.play-button')) {
            return;
        }

        const playground = playgroundFromEvent(event);
        if (!playground) {
            return;
        }

        event.preventDefault();
        event.stopImmediatePropagation();
        execute(playground);
    }, true);

    document.addEventListener('keydown', event => {
        if (!event.ctrlKey || event.key !== 'Enter') {
            return;
        }

        const playground = playgroundFromEvent(event);
        if (!playground) {
            return;
        }

        event.preventDefault();
        event.stopImmediatePropagation();
        execute(playground);
    }, true);
})();
