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

    function attachedTestCandidates(playground) {
        const tests = playground.nextElementSibling;
        if (!tests || !tests.classList.contains('playground-tests')) {
            return [];
        }

        const candidates = [tests.textContent.trim()];
        const fallback = tests.nextElementSibling;
        if (fallback?.classList.contains('playground-tests-fallback')) {
            candidates.push(fallback.textContent.trim());
        }
        return candidates;
    }

    function testCandidates(playground, code) {
        const attached = attachedTestCandidates(playground);
        if (attached.length > 0) {
            return attached;
        }

        const functionNames = Array.from(
            code.matchAll(/\b(?:pub\s+)?fn\s+([A-Za-z_][A-Za-z0-9_]*)/g),
            match => match[1],
        );
        if (functionNames.length === 0) {
            return [];
        }

        const candidates = Array.from(document.querySelectorAll(
            'script.playground-tests, script.playground-tests-fallback',
        ))
            .map(tests => tests.textContent.trim())
            .filter(tests => functionNames.some(name => {
                const call = new RegExp(`\\b${name}\\s*\\(`);
                return call.test(tests);
            }));

        return Array.from(new Set(candidates));
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

        const candidates = testCandidates(playground, code);
        const runsTests = candidates.length > 0;
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
            const outputs = [];

            if (runsTests) {
                for (const [index, tests] of candidates.entries()) {
                    if (index > 0) {
                        result.textContent = 'Trying tests for a compatible API...';
                    }
                    const output = await runOnPlayground(
                        `${code}\n\n${tests}\n`,
                        playground,
                        true,
                        controller.signal,
                    );
                    outputs.push(output);
                    if (output.success) {
                        break;
                    }
                }
            } else {
                outputs.push(await runOnPlayground(
                    code,
                    playground,
                    false,
                    controller.signal,
                ));
            }

            const output = outputs[outputs.length - 1];
            let text = outputText(output);
            if (!output.success && outputs.length > 1) {
                text = outputs
                    .map((attempt, index) => `Test set ${index + 1}:\n${outputText(attempt)}`)
                    .join('\n\n');
            }

            if (output.success && runsTests) {
                result.textContent = text ? `Tests passed.\n\n${text}` : 'Tests passed.';
            } else {
                result.textContent = text || 'Code compiles.';
            }
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

        const runsTests = testCandidates(playground, editableCode(playground)).length > 0;
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
