'use strict';

const assert = require('node:assert/strict');
const { readFileSync } = require('node:fs');
const { test } = require('node:test');
const vm = require('node:vm');

const source = readFileSync(new URL('../book/theme/js/playground-tests.js', `file://${__filename}`), 'utf8');

// Only the DOM surface used by the runner, with real parent/sibling relationships.
class Element {
    constructor(tagName) {
        this.tagName = tagName.toUpperCase();
        this.children = [];
        this.parentElement = null;
        this.className = '';
        this.attributes = {};
        this.value = '';
        this.listeners = new Map();
        if (this.tagName === 'DETAILS') this.open = false;
        this.classList = {
            contains: name => this.className.split(/\s+/).includes(name),
            remove: name => { this.className = this.className.split(/\s+/).filter(item => item !== name).join(' '); },
            toggle: (name, enabled) => {
                this.classList.remove(name);
                if (enabled) this.className += ` ${name}`;
            },
            [Symbol.iterator]: () => this.className.split(/\s+/)[Symbol.iterator](),
        };
    }
    append(...children) {
        for (const child of children) {
            if (child instanceof Element && child.parentElement) {
                const siblings = child.parentElement.children;
                siblings.splice(siblings.indexOf(child), 1);
            }
            this.children.push(child);
            if (child instanceof Element) child.parentElement = this;
        }
    }
    after(element) {
        const siblings = this.parentElement.children;
        siblings.splice(siblings.indexOf(this) + 1, 0, element);
        element.parentElement = this.parentElement;
    }
    get nextElementSibling() {
        const siblings = this.parentElement?.children || [];
        return siblings.slice(siblings.indexOf(this) + 1).find(child => child instanceof Element) || null;
    }
    set textContent(text) { this.replaceChildren(...(String(text) ? [String(text)] : [])); }
    get textContent() { return this.children.map(child => typeof child === 'string' ? child : child.textContent).join(''); }
    replaceChildren(...children) {
        for (const child of this.children) if (child instanceof Element) child.parentElement = null;
        this.children = [];
        this.append(...children);
    }
    setAttribute(name, value) { this.attributes[name] = value; }
    addEventListener(type, listener) {
        this.listeners.set(type, [...(this.listeners.get(type) || []), listener]);
    }
    remove() {
        if (!this.parentElement) return;
        const siblings = this.parentElement.children;
        siblings.splice(siblings.indexOf(this), 1);
        this.parentElement = null;
    }
    focus() {
        let root = this;
        while (root.parentElement) root = root.parentElement;
        root.activeElement = this;
    }
    matches(selector) {
        return selector.split(',').some(part => {
            const [tag, ...classes] = part.trim().split('.');
            return (!tag || tag.toUpperCase() === this.tagName) && classes.every(name => this.classList.contains(name));
        });
    }
    closest(selector) { return this.matches(selector) ? this : this.parentElement?.closest(selector) || null; }
    querySelectorAll(selector) {
        return this.children.flatMap(child => child instanceof Element
            ? [...(child.matches(selector) ? [child] : []), ...child.querySelectorAll(selector)] : []);
    }
    querySelector(selector) { return this.querySelectorAll(selector)[0] || null; }
}

function element(tag, className = '', text = '') {
    const node = new Element(tag);
    node.className = className;
    node.textContent = text;
    return node;
}

function harness({ ace = true } = {}) {
    const document = element('main');
    const listeners = new Map();
    document.createElement = tag => element(tag);
    document.addEventListener = (type, listener, capture) => {
        assert.equal(capture, true);
        listeners.set(type, [...(listeners.get(type) || []), listener]);
    };
    const requests = [];
    const timers = new Map();
    const aceValues = new Map();
    const aceEditors = new Map();
    const aceEditCalls = [];
    const highlighted = [];
    let respond = async () => ({ success: true, stdout: 'test result: ok.', stderr: '' });
    let timerId = 0;
    const window = {
        ace: ace ? {
            edit(code) {
                aceEditCalls.push(code);
                if (!aceEditors.has(code)) {
                    const initialValue = code.textContent;
                    const input = element('textarea', 'ace_text-input');
                    aceEditors.set(code, {
                        options: { theme: 'ace/theme/dawn', fontFamily: 'monospace' },
                        getValue: () => aceValues.get(code) ?? initialValue,
                        setValue: value => aceValues.set(code, value),
                        setOptions(options) { Object.assign(this.options, options); },
                        getOption(name) { return this.options[name]; },
                        getTheme() { return this.options.theme; },
                        setTheme(theme) { this.options.theme = theme; },
                        textInput: {
                            getElement() {
                                if (!input.parentElement) {
                                    const wrapper = element('div', 'ace_input-wrapper');
                                    wrapper.append(input);
                                    code.append(wrapper);
                                }
                                return input;
                            },
                        },
                        commands: {
                            added: [],
                            addCommand(command) { this.added.push(command); },
                        },
                        resizeCalls: [],
                        resize(force) { this.resizeCalls.push(force); },
                    });
                }
                return aceEditors.get(code);
            },
        } : undefined,
        hljs: { highlightBlock: code => highlighted.push(code.textContent) },
        setTimeout: (callback, delay) => { assert.equal(delay, 20_000); timers.set(++timerId, callback); return timerId; },
        clearTimeout: id => timers.delete(id),
    };
    const context = vm.createContext({
        document, Element, AbortController, window,
        fetch: async (url, options) => {
            requests.push({ url, ...options, body: JSON.parse(options.body) });
            const output = await respond(options);
            return { ok: true, json: async () => output };
        },
    });
    return {
        document, window, requests, timers, aceValues, aceEditors, aceEditCalls, highlighted,
        respond: fn => { respond = fn; },
        init: () => vm.runInContext(source, context),
        block({ suite, wrapped = false, editable = true, button = true } = {}) {
            const pre = element('pre', 'playground');
            const code = element('code', editable ? 'editable edition2024' : '', 'fn shared() {}');
            pre.append(code);
            if (button) pre.append(element('button', 'play-button'));
            if (wrapped) {
                const wrapper = element('div');
                wrapper.append(pre);
                document.append(wrapper);
            } else document.append(pre);
            let script;
            if (suite !== undefined) {
                script = element('script', 'playground-tests', suite);
                script.setAttribute('data-expected-failures', '1');
                document.append(script);
            }
            return { pre, code, script };
        },
        toggle(details, open) {
            details.open = open;
            return this.fire('toggle', details);
        },
        async fire(type, target, extra = {}) {
            const event = {
                target, ...extra, prevented: false, stopped: false,
                preventDefault() { this.prevented = true; },
                stopImmediatePropagation() { this.stopped = true; },
            };
            for (const listener of [...(listeners.get(type) || []), ...(target.listeners.get(type) || [])]) {
                listener(event);
                if (event.stopped) break;
            }
            await new Promise(resolve => setImmediate(resolve));
            return event;
        },
    };
}

test('attached callers are editable, accessible, adjacent, and submitted verbatim with current Ace code', async () => {
    const h = harness();
    const { pre, code, script } = h.block({ suite: '  #[test]\nfn old() { shared(); }\n ' });
    h.init();
    const details = pre.nextElementSibling;
    assert.equal(details.tagName, 'DETAILS');
    assert.equal(details.querySelector('summary').textContent, 'Tests and local setup');
    const textarea = details.querySelector('textarea');
    assert.equal(textarea.parentElement.tagName, 'LABEL');
    assert.match(textarea.parentElement.textContent, /Edit tests and callers/);
    assert.equal(textarea.value, script.textContent.trim());
    assert.equal(textarea.rows, 6);
    assert.equal(textarea.spellcheck, false);
    assert.ok(!textarea.readOnly && !textarea.disabled);
    h.aceValues.set(code, 'fn changed(arg: u8) {}');
    textarea.value = '\n#[test]\nfn edited() { changed(42); }\n';
    await h.fire('click', pre.querySelector('.play-button'));
    assert.equal(h.requests.length, 1);
    assert.equal(h.requests[0].body.code, `fn changed(arg: u8) {}\n\n${textarea.value}\n`);
    assert.equal(h.requests[0].body.tests, true);
    assert.equal(h.requests[0].body.edition, '2024');
    assert.equal(h.requests[0].body.crateType, 'bin');
    assert.equal(script.textContent, '  #[test]\nfn old() { shared(); }\n ');
    assert.equal(script.attributes['data-expected-failures'], '1');
    assert.equal(h.timers.size, 0);
});

for (const wrapped of [false, true]) {
    test(`adjacent local setup moves below editable tests without duplication or payload changes (wrapped: ${wrapped})`, async () => {
        const h = harness();
        const { pre, code, script } = h.block({ wrapped, suite: '#[test]\nfn original() {}' });
        const local = element('div', 'playground-local-setup');
        const heading = element('strong', '', 'Run locally');
        const shell = element('pre');
        shell.append(element('code', 'language-sh', 'cargo test --example local_only'));
        local.append(heading, shell);
        script.after(local);
        h.init();
        const details = pre.nextElementSibling;
        const label = details.querySelector('label');
        const textarea = label.querySelector('textarea');
        assert.equal(details.querySelector('summary').textContent, 'Tests and local setup');
        assert.deepEqual(details.children, [details.querySelector('summary'), label, local]);
        assert.deepEqual(local.children, [heading, shell]);
        assert.equal(local.parentElement, details);
        assert.equal(script.nextElementSibling, null);
        assert.equal(textarea.value, script.textContent);
        textarea.value = '\n#[test]\nfn edited() { changed(); }\n ';
        h.aceValues.set(code, 'fn changed() {}\n');
        h.init();
        assert.equal(h.document.querySelectorAll('details.playground-test-editor').length, 1);
        assert.deepEqual(h.document.querySelectorAll('div.playground-local-setup'), [local]);
        assert.equal(label.nextElementSibling, local);
        await h.fire('click', pre.querySelector('.play-button'));
        assert.equal(h.requests.length, 1);
        assert.equal(h.requests[0].body.code, `fn changed() {}\n\n\n${textarea.value}\n`);
        assert.doesNotMatch(h.requests[0].body.code, /Run locally|cargo test|local_only/);
    });
}

for (const placement of ['nonadjacent', 'wrong tag', 'wrong class', 'other block']) {
    test(`does not adopt ${placement} local instructions`, () => {
        const h = harness();
        const first = h.block({ suite: 'first tests' });
        const local = element(placement === 'wrong tag' ? 'section' : 'div',
            placement === 'wrong class' ? 'other-local-setup' : 'playground-local-setup', 'Run locally');
        let other;
        if (placement === 'nonadjacent') h.document.append(element('p', '', 'Intervening content'));
        if (placement === 'other block') other = h.block({ suite: 'other tests' });
        h.document.append(local);
        h.init();
        const details = first.pre.nextElementSibling;
        assert.deepEqual(details.children.map(child => child.tagName), ['SUMMARY', 'LABEL']);
        assert.equal(local.parentElement, other ? other.pre.nextElementSibling : h.document);
        assert.equal(h.document.querySelectorAll('textarea').length, other ? 2 : 1);
    });
}

for (const [name, output] of [
    ['assertion', { success: false, stdout: 'test example ... FAILED\ntest result: FAILED.', stderr: "thread panicked: assertion failed" }],
    ['compile', { success: false, stdout: '', stderr: 'error[E0308]: mismatched types\n  --> src/lib.rs:2:1\n2 | changed();' }],
    ['empty', { success: false, stdout: '', stderr: '' }],
]) {
    test(`${name} failure stays visible and never retries fallback tests`, async () => {
        const h = harness();
        const { pre } = h.block({ suite: 'first suite' });
        h.document.append(element('script', 'playground-tests-fallback', 'fallback suite'));
        h.block({ suite: 'another suite' });
        h.respond(async () => output);
        h.init();
        await h.fire('click', pre.querySelector('.play-button'));
        assert.equal(h.requests.length, 1);
        assert.equal(h.requests[0].body.code, 'fn shared() {}\n\nfirst suite\n');
        const result = pre.querySelector('.result');
        assert.ok(!result.classList.contains('result-no-output'));
        assert.match(result.textContent, name === 'empty' ? /Execution failed/ : name === 'compile' ? /error\[E0308\]/ : /test result: FAILED/);
        assert.doesNotMatch(result.textContent, /Tests passed|Code compiles/);
        if (name !== 'empty') assert.ok(result.querySelector('.diagnostic-error'));
        if (name === 'compile') assert.deepEqual(h.highlighted, ['changed();']);
        assert.equal(pre.querySelector('.play-button').disabled, false);
    });
}

test('plain blocks compile as libraries without adopting same-name chapter or fallback suites', async () => {
    const h = harness();
    h.document.append(element('h1', '', 'Chapter'));
    h.block({ suite: '#[test] fn check() { shared(); }' });
    const { pre, code } = h.block();
    h.document.append(element('script', 'playground-tests-fallback', '#[test] fn check() { shared(); }'));
    const other = h.block();
    h.document.append(element('div', 'playground-tests', 'not a script'));
    h.respond(async () => ({ success: true, stdout: '', stderr: '' }));
    h.init();
    for (const block of [pre, other.pre]) {
        assert.notEqual(block.nextElementSibling.tagName, 'DETAILS');
        assert.equal(block.querySelector('.play-button').title, 'Check code');
        await h.fire('click', block.querySelector('.play-button'));
    }
    assert.equal(h.requests.length, 2);
    for (const request of h.requests) {
        assert.equal(request.body.code, code.textContent);
        assert.equal(request.body.crateType, 'lib');
        assert.equal(request.body.tests, false);
    }
    assert.match(pre.querySelector('.result').textContent, /Code compiles/);
});

test('wrapped attachment, textarea Ctrl+Enter ownership, and repeated initialization do not duplicate UI or requests', async () => {
    const h = harness();
    const first = h.block({ wrapped: true, suite: 'first' });
    h.block({ suite: 'second' });
    h.init();
    const textarea = first.pre.nextElementSibling.querySelector('textarea');
    textarea.value = 'edited';
    h.init();
    assert.equal(h.document.querySelectorAll('details.playground-test-editor').length, 2);
    assert.equal(textarea.value, 'edited');
    const event = await h.fire('keydown', textarea, { ctrlKey: true, key: 'Enter' });
    assert.ok(event.prevented && event.stopped);
    assert.equal(h.requests.length, 1);
    assert.equal(h.requests[0].body.code, 'fn shared() {}\n\nedited\n');
    assert.ok(first.pre.querySelector('.result'));
});

test('empty edited suite stays attached and original snippet Ctrl+Enter runs it', async () => {
    const h = harness();
    const { pre, code } = h.block({ suite: 'old tests' });
    h.init();
    pre.nextElementSibling.querySelector('textarea').value = '';
    await h.fire('keydown', code, { ctrlKey: true, key: 'Enter' });
    assert.equal(h.requests[0].body.code, 'fn shared() {}\n\n\n');
    assert.equal(h.requests[0].body.tests, true);
});

test('in-flight runs lock both entry points, timeout is visible, and retry unlocks', async () => {
    const h = harness();
    const { pre } = h.block({ suite: 'tests' });
    h.respond(options => new Promise((resolve, reject) => {
        options.signal.addEventListener('abort', () => reject(Object.assign(new Error('aborted'), { name: 'AbortError' })));
    }));
    h.init();
    const button = pre.querySelector('.play-button');
    await h.fire('click', button);
    assert.equal(button.disabled, true);
    await h.fire('keydown', pre.nextElementSibling.querySelector('textarea'), { ctrlKey: true, key: 'Enter' });
    assert.equal(h.requests.length, 1);
    for (const callback of h.timers.values()) callback();
    await new Promise(resolve => setImmediate(resolve));
    assert.match(pre.querySelector('.result').textContent, /request timed out/);
    assert.equal(button.disabled, false);
    assert.equal(h.timers.size, 0);
    h.respond(async () => ({ success: true, stdout: 'test result: ok.', stderr: '' }));
    await h.fire('click', button);
    assert.equal(h.requests.length, 2);
    assert.ok(pre.querySelector('.diagnostic-success'));
});

test('non-editable blocks are ignored and attachments without buttons still get an editor', () => {
    const h = harness();
    const ignored = h.block({ editable: false, suite: 'ignored' });
    const editable = h.block({ button: false, suite: 'editable' });
    h.init();
    assert.equal(ignored.pre.nextElementSibling, ignored.script);
    assert.equal(editable.pre.nextElementSibling.tagName, 'DETAILS');
});

test('test editors initialize lazily and retain current textarea edits on first opening', async () => {
    const h = harness();
    const first = h.block({ suite: '  #[test]\nfn original() { shared(); }\n ' });
    const second = h.block({ suite: 'other tests' });
    h.init();
    const details = first.pre.nextElementSibling;
    const textarea = details.querySelector('textarea');
    assert.equal(details.open, false);
    assert.equal(details.querySelector('pre.playground-tests-code'), null);
    assert.deepEqual([...h.aceEditors.keys()], [first.code, second.code]);
    assert.equal(h.window.editors, undefined);
    const callsBeforeOpening = h.aceEditCalls.length;
    textarea.value = '\n#[test]\nfn edited_before_opening() { shared(); }\n ';
    await h.toggle(details, false);
    assert.equal(h.aceEditCalls.length, callsBeforeOpening);
    await h.toggle(details, true);
    const testPre = details.querySelector('pre.playground-tests-code');
    const editor = h.aceEditors.get(testPre.querySelector('code'));
    assert.equal(details.querySelector('label').nextElementSibling, testPre);
    assert.equal(editor.getValue(), textarea.value);
    assert.equal(textarea.parentElement, null);
    assert.equal(details.querySelector('textarea.playground-tests-source'), null);
    assert.equal(second.pre.nextElementSibling.querySelector('pre.playground-tests-code'), null);
    assert.ok(second.pre.nextElementSibling.querySelector('textarea.playground-tests-source'));
    assert.equal(h.aceEditors.size, 3);
    assert.deepEqual(Array.from(h.window.editors), [editor]);
    assert.equal(h.aceEditors.get(first.code).getValue(), 'fn shared() {}');
    assert.equal(first.script.textContent, '  #[test]\nfn original() { shared(); }\n ');
    assert.equal(h.requests.length, 0);
});

test('Rust editors preserve accessible labels and inherit and register for snippet theme and font updates', async () => {
    const h = harness();
    const blocks = [
        { ...h.block({ suite: 'first tests' }), theme: 'ace/theme/tomorrow_night', fontFamily: '"Fira Code", monospace' },
        { ...h.block({ suite: 'second tests' }), theme: 'ace/theme/dawn', fontFamily: 'Consolas, monospace' },
    ];
    h.init();
    const snippets = blocks.map(block => h.aceEditors.get(block.code));
    h.window.editors = [...snippets];
    for (const [index, block] of blocks.entries()) {
        block.details = block.pre.nextElementSibling;
        block.inputId = block.details.querySelector('textarea').id;
        assert.equal(block.inputId, `playground-tests-${index + 1}`);
        assert.equal(block.details.querySelector('label').htmlFor, block.inputId);
        snippets[index].setOptions({ theme: block.theme, fontFamily: block.fontFamily });
    }
    const testEditors = [];
    for (const block of [...blocks].reverse()) {
        await h.toggle(block.details, true);
        const testPre = block.details.querySelector('pre.playground-tests-code');
        const editor = h.aceEditors.get(testPre.querySelector('code'));
        testEditors.push(editor);
        assert.deepEqual(editor.options, {
            mode: 'ace/mode/rust',
            theme: block.theme,
            fontFamily: block.fontFamily,
            fontSize: 'inherit',
            highlightActiveLine: false,
            showPrintMargin: false,
            showLineNumbers: true,
            showGutter: true,
            minLines: 6,
            maxLines: 24,
            tabSize: 4,
            useSoftTabs: true,
        });
        const label = block.details.querySelector('label');
        const input = editor.textInput.getElement();
        assert.equal(input.id, block.inputId);
        assert.equal(label.htmlFor, input.id);
        assert.equal(input.attributes['aria-label'], 'Edit tests and callers to match your API:');
        assert.equal(label.querySelector('textarea'), null);
        assert.equal(input.closest('details.playground-test-editor'), block.details);
    }
    assert.equal(h.document.querySelectorAll('textarea').length, 2);
    assert.deepEqual(Array.from(h.window.editors), [...snippets, ...testEditors]);
    for (const editor of h.window.editors) {
        editor.setTheme('ace/theme/ayu_mirage');
        editor.setOptions({ fontFamily: '"Source Code Pro", monospace' });
    }
    for (const editor of testEditors) {
        assert.equal(editor.getTheme(), 'ace/theme/ayu_mirage');
        assert.equal(editor.getOption('fontFamily'), '"Source Code Pro", monospace');
    }
});

test('reopening resizes without recreating editors, losing edits, or duplicating hints and local setup', async () => {
    const h = harness();
    const { pre, script } = h.block({ suite: 'original tests' });
    const local = element('div', 'playground-local-setup', 'Run locally');
    script.after(local);
    h.init();
    const details = pre.nextElementSibling;
    await h.toggle(details, true);
    const testPre = details.querySelector('pre.playground-tests-code');
    const testCode = testPre.querySelector('code');
    const editor = h.aceEditors.get(testCode);
    const hint = details.querySelector('p.playground-test-editor-hint');
    assert.ok(hint);
    assert.deepEqual(editor.resizeCalls, [true]);
    editor.setValue('\n#[test]\nfn edited() { shared(); }\n');
    for (let reopening = 0; reopening < 2; reopening++) {
        h.init();
        await h.toggle(details, false);
        assert.equal(editor.resizeCalls.length, reopening + 1);
        await h.toggle(details, true);
    }
    assert.deepEqual(editor.resizeCalls, [true, true, true]);
    assert.equal(h.aceEditCalls.filter(code => code === testCode).length, 1);
    assert.equal(h.aceEditors.size, 2);
    assert.equal(editor.getValue(), '\n#[test]\nfn edited() { shared(); }\n');
    assert.equal(editor.commands.added.length, 1);
    assert.deepEqual(Array.from(h.window.editors), [editor]);
    assert.deepEqual(details.children, [details.querySelector('summary'), details.querySelector('label'), testPre, hint, local]);
    assert.equal(h.document.querySelectorAll('details.playground-test-editor').length, 1);
    assert.equal(script.textContent, 'original tests');
    assert.equal(h.requests.length, 0);
});

for (const trigger of ['Run', 'nested Ace Ctrl+Enter']) {
    test(`${trigger} submits current Ace snippet and tests exactly once to their own playground`, async () => {
        const h = harness();
        const blocks = [h.block({ wrapped: true, suite: 'first original suite' }), h.block({ suite: 'second original suite' })];
        const originalScripts = blocks.map(block => block.script.textContent);
        h.init();
        for (const block of blocks) {
            block.details = block.pre.nextElementSibling;
            const textarea = block.details.querySelector('textarea');
            await h.toggle(block.details, true);
            block.editor = h.aceEditors.get(block.details.querySelector('pre.playground-tests-code').querySelector('code'));
            textarea.value = 'stale detached textarea';
        }
        h.init();
        for (const [index, block] of [blocks[1], blocks[0], blocks[1]].entries()) {
            const snippet = `fn changed_${index}(value: u8) {}`;
            const suite = `\n#[test]\nfn edited_${index}() { changed_${index}(${index}); }\n `;
            h.aceValues.set(block.code, snippet);
            block.editor.setValue(suite);
            const target = trigger === 'Run' ? block.pre.querySelector('.play-button') : block.editor.textInput.getElement();
            if (trigger !== 'Run') {
                assert.equal(target.closest('pre.playground'), null);
                assert.equal(target.closest('details.playground-test-editor'), block.details);
            }
            const event = await h.fire(trigger === 'Run' ? 'click' : 'keydown', target, { ctrlKey: true, key: 'Enter' });
            assert.ok(event.prevented && event.stopped);
            assert.equal(h.requests.length, index + 1);
            assert.equal(h.requests[index].body.code, `${snippet}\n\n${suite}\n`);
            assert.equal(h.requests[index].body.tests, true);
            assert.equal(h.requests[index].body.crateType, 'bin');
            assert.equal(block.editor.getValue(), suite);
            assert.equal(h.aceEditors.get(block.code).getValue(), snippet);
            assert.ok(block.pre.querySelector('.result'));
            if (index === 0) assert.equal(blocks[0].pre.querySelector('.result'), null);
        }
        assert.deepEqual(blocks.map(block => block.script.textContent), originalScripts);
    });
}

test('an empty edited Ace suite stays attached without restoring original or fallback tests', async () => {
    const h = harness();
    const { pre, code, script } = h.block({ suite: 'original tests' });
    h.document.append(element('script', 'playground-tests-fallback', 'fallback tests'));
    h.block({ suite: 'another suite' });
    h.init();
    const details = pre.nextElementSibling;
    await h.toggle(details, true);
    const editor = h.aceEditors.get(details.querySelector('pre.playground-tests-code').querySelector('code'));
    editor.setValue('');
    h.aceValues.set(code, 'fn changed() {}');
    await h.fire('click', pre.querySelector('.play-button'));
    assert.equal(h.requests.length, 1);
    assert.equal(h.requests[0].body.code, 'fn changed() {}\n\n\n');
    assert.equal(h.requests[0].body.tests, true);
    assert.equal(h.requests[0].body.crateType, 'bin');
    assert.equal(editor.getValue(), '');
    assert.equal(script.textContent, 'original tests');
});

test('Escape returns focus to the disclosure summary without changing tests or closing the editor', async () => {
    const h = harness();
    const { pre } = h.block({ suite: 'original tests' });
    h.init();
    const details = pre.nextElementSibling;
    await h.toggle(details, true);
    const testPre = details.querySelector('pre.playground-tests-code');
    const editor = h.aceEditors.get(testPre.querySelector('code'));
    const hint = testPre.nextElementSibling;
    assert.ok(hint.matches('p.playground-test-editor-hint'));
    assert.equal(hint.textContent, 'Press Escape to leave the editor.');
    const command = editor.commands.added.find(command => command.name === 'leave-test-editor');
    assert.ok(command);
    assert.deepEqual({ ...command.bindKey }, { win: 'Esc', mac: 'Esc' });
    editor.setValue('edited tests');
    const input = editor.textInput.getElement();
    input.focus();
    assert.equal(h.document.activeElement, input);
    command.exec(editor);
    assert.equal(h.document.activeElement, details.querySelector('summary'));
    assert.equal(details.open, true);
    assert.equal(editor.getValue(), 'edited tests');
    assert.equal(h.requests.length, 0);
});

test('without Ace, the textarea remains editable across openings and both run entry points use its current value', async () => {
    const h = harness({ ace: false });
    const { pre, code, script } = h.block({ suite: 'original tests' });
    const other = h.block({ suite: 'other tests' });
    h.init();
    const details = pre.nextElementSibling;
    const textarea = details.querySelector('textarea.playground-tests-source');
    textarea.value = '\n#[test]\nfn edited_before_opening() { changed(1); }\n';
    code.textContent = 'fn changed(value: u8) {}';
    await h.toggle(details, true);
    assert.equal(details.querySelector('textarea.playground-tests-source'), textarea);
    assert.ok(!textarea.readOnly && !textarea.disabled);
    assert.equal(details.querySelector('label').htmlFor, textarea.id);
    await h.fire('click', pre.querySelector('.play-button'));
    assert.equal(h.requests.length, 1);
    assert.equal(h.requests[0].body.code, `${code.textContent}\n\n${textarea.value}\n`);
    assert.equal(h.requests[0].body.tests, true);
    await h.toggle(details, false);
    h.init();
    await h.toggle(details, true);
    assert.equal(details.querySelector('textarea.playground-tests-source'), textarea);
    textarea.value = '\n#[test]\nfn edited_again() { changed(2); }\n';
    const event = await h.fire('keydown', textarea, { ctrlKey: true, key: 'Enter' });
    assert.ok(event.prevented && event.stopped);
    assert.equal(h.requests.length, 2);
    assert.equal(h.requests[1].body.code, `${code.textContent}\n\n${textarea.value}\n`);
    assert.equal(h.requests[1].body.tests, true);
    assert.equal(h.requests[1].body.crateType, 'bin');
    assert.equal(h.aceEditors.size, 0);
    assert.equal(h.window.editors, undefined);
    assert.equal(details.querySelector('pre.playground-tests-code'), null);
    assert.equal(details.querySelector('p.playground-test-editor-hint'), null);
    assert.equal(other.pre.querySelector('.result'), null);
    assert.equal(h.document.querySelectorAll('textarea').length, 2);
    assert.equal(script.textContent, 'original tests');
});
