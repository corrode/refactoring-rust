'use strict';

// mdBook submits editable snippets as binaries and omits code hidden outside Ace.
// Exercise listings need the opposite: visible code plus hidden tests, run with `cargo test`.
(() => {
    const playgroundUrl = 'https://play.rust-lang.org/execute';

    function editableCode(playground) {
        const code = playground.querySelector('code');
        if (!code || !code.classList.contains('editable')) {
            return null;
        }
        return window.ace ? window.ace.edit(code).getValue() : code.textContent;
    }

    function testsFor(playground) {
        const tests = playground.nextElementSibling;
        if (!tests || !tests.classList.contains('playground-tests')) {
            return null;
        }
        return tests.textContent.trim();
    }

    function fallbackTestsFor(playground) {
        const tests = playground.nextElementSibling;
        const fallback = tests?.nextElementSibling;
        if (!fallback || !fallback.classList.contains('playground-tests-fallback')) {
            return null;
        }
        return fallback.textContent.trim();
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
            result.className = 'result hljs language-bash';
            playground.append(result);
        }
        return result;
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
        return output.success
            ? output.stdout.trim()
            : [output.stderr, output.stdout].filter(Boolean).join('\n').trim();
    }

    async function execute(playground) {
        const code = editableCode(playground);
        if (code === null) {
            return;
        }

        const tests = testsFor(playground);
        const fallbackTests = fallbackTestsFor(playground);
        const runsTests = tests !== null;
        const source = runsTests ? `${code}\n\n${tests}\n` : code;
        const result = resultBlock(playground);
        const button = playground.querySelector('.play-button');
        const controller = new AbortController();
        const timeout = window.setTimeout(() => controller.abort(), 20_000);

        result.textContent = runsTests ? 'Running tests...' : 'Checking code...';
        result.classList.remove('result-no-output');
        if (button) {
            button.disabled = true;
        }

        try {
            const primaryOutput = await runOnPlayground(
                source,
                playground,
                runsTests,
                controller.signal,
            );
            let output = primaryOutput;

            if (!primaryOutput.success && fallbackTests !== null) {
                result.textContent = 'Trying tests for the refactored API...';
                output = await runOnPlayground(
                    `${code}\n\n${fallbackTests}\n`,
                    playground,
                    true,
                    controller.signal,
                );
            }

            let text = outputText(output);
            if (!primaryOutput.success && !output.success && fallbackTests !== null) {
                text = `Current API tests:\n${outputText(primaryOutput)}\n\nRefactored API tests:\n${text}`;
            }

            result.textContent = text || (runsTests ? 'Tests passed.' : 'Code compiles.');
            result.classList.toggle('result-no-output', output.success && text === '');
        } catch (error) {
            const message = error.name === 'AbortError' ? 'request timed out' : error.message;
            result.textContent = `Playground communication: ${message}`;
            result.classList.remove('result-no-output');
        } finally {
            window.clearTimeout(timeout);
            if (button) {
                button.disabled = false;
            }
        }
    }

    function playgroundFromEvent(event) {
        const target = event.target instanceof Element ? event.target : null;
        const playground = target?.closest('pre.playground');
        if (!playground || editableCode(playground) === null) {
            return null;
        }
        return playground;
    }

    document.querySelectorAll('pre.playground').forEach(playground => {
        if (editableCode(playground) === null) {
            return;
        }

        const button = playground.querySelector('.play-button');
        if (!button) {
            return;
        }

        const runsTests = testsFor(playground) !== null;
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
