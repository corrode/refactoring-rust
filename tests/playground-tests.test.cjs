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

function harness() {
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
    const highlighted = [];
    let respond = async () => ({ success: true, stdout: 'test result: ok.', stderr: '' });
    let timerId = 0;
    const context = vm.createContext({
        document, Element, AbortController,
        window: {
            ace: { edit: code => ({ getValue: () => aceValues.get(code) ?? code.textContent }) },
            hljs: { highlightBlock: code => highlighted.push(code.textContent) },
            setTimeout: (callback, delay) => { assert.equal(delay, 20_000); timers.set(++timerId, callback); return timerId; },
            clearTimeout: id => timers.delete(id),
        },
        fetch: async (url, options) => {
            requests.push({ url, ...options, body: JSON.parse(options.body) });
            const output = await respond(options);
            return { ok: true, json: async () => output };
        },
    });
    return {
        document, requests, timers, aceValues, highlighted,
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
        async fire(type, target, extra = {}) {
            const event = {
                target, ...extra, prevented: false, stopped: false,
                preventDefault() { this.prevented = true; },
                stopImmediatePropagation() { this.stopped = true; },
            };
            for (const listener of listeners.get(type) || []) {
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
