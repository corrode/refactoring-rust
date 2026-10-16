'use strict';

// mdBook submits editable snippets as binaries and omits code hidden outside Ace.
// Exercise listings need the opposite: visible code plus hidden tests, run with `cargo test`.
(() => {
    const playgroundUrl = 'https://play.rust-lang.org/execute';
    const activeRuns = new WeakMap();

    function editableCode(playground) {
        const code = playground.querySelector('code');
        if (!code || !code.classList.contains('editable')) {
            return null;
        }
        return window.ace ? window.ace.edit(code).getValue() : code.textContent;
    }

    function attachedTestCandidates(playground) {
        const tests = [playground, playground.parentElement]
            .map(element => element?.nextElementSibling)
            .find(element => element?.classList.contains('playground-tests'));
        if (!tests) {
            return [];
        }

        const candidates = [tests.textContent.trim()];
        const fallback = tests.nextElementSibling;
        if (fallback?.classList.contains('playground-tests-fallback')) {
            candidates.push(fallback.textContent.trim());
        }
        return candidates;
    }

    function chapterTestScripts(playground) {
        const main = playground.closest('main');
        if (!main) {
            return [];
        }

        let section = playground;
        while (section.parentElement && section.parentElement !== main) {
            section = section.parentElement;
        }

        let heading = section;
        while (heading && heading.tagName !== 'H1') {
            heading = heading.previousElementSibling;
        }
        if (!heading) {
            return [];
        }

        const scripts = [];
        for (
            let element = heading.nextElementSibling;
            element && element.tagName !== 'H1';
            element = element.nextElementSibling
        ) {
            if (element.matches('script.playground-tests, script.playground-tests-fallback')) {
                scripts.push(element);
            }
            scripts.push(...element.querySelectorAll(
                'script.playground-tests, script.playground-tests-fallback',
            ));
        }
        return scripts;
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

        const candidates = chapterTestScripts(playground)
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

        const candidates = testCandidates(playground, code);
        const runsTests = candidates.length > 0;
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
