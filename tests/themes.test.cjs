'use strict';

const assert = require('node:assert/strict');
const { existsSync, readFileSync } = require('node:fs');
const { test } = require('node:test');
const vm = require('node:vm');
const path = require('node:path');

const root = path.resolve(__dirname, '..');
const source = readFileSync(path.join(root, 'book/theme/js/themes.js'), 'utf8');
const builtIndex = path.join(root, 'book/book/index.html');
// Exercise the actual native implementation when a book build is available,
// without copying mdBook internals into a fixture that could silently drift.
const html = existsSync(builtIndex) ? readFileSync(builtIndex, 'utf8') : null;
const bookScript = html?.match(/<script src="(book-[^"]+\.js)"><\/script>/)?.[1];
const nativeSource = bookScript
    ? readFileSync(path.join(root, 'book/book', bookScript), 'utf8')
        .match(/\(function themes\(\) \{[\s\S]*?\n\}\)\(\);/)?.[0]
    : null;
if (html) assert.ok(nativeSource, 'Built mdBook theme implementation must be recognizable');
const integration = { skip: nativeSource ? false : 'Run make book-build to test native mdBook integration' };

function harness({ saved = null, dark = false, blocked = false, native = false } = {}) {
    const document = { activeElement: null };
    class Element {
        constructor(id = '', className = '') {
            this.id = id;
            this.className = className;
            this.children = [];
            this.attributes = {};
            this.style = {};
            this.listeners = {};
            this.classList = {
                contains: name => this.className.split(' ').includes(name),
                add: name => {
                    if (!this.classList.contains(name)) this.className = `${this.className} ${name}`.trim();
                },
                remove: name => { this.className = this.className.split(' ').filter(item => item !== name).join(' '); },
            };
        }
        append(child) { this.children.push(child); child.parentElement = this; }
        remove() { this.parentElement.children = this.parentElement.children.filter(child => child !== this); }
        setAttribute(name, value) { this.attributes[name] = String(value); }
        addEventListener(type, listener) { (this.listeners[type] ||= []).push(listener); }
        dispatch(type, event = {}) {
            event.target ||= this;
            event.stopImmediatePropagation ||= () => { event.stopped = true; };
            event.preventDefault ||= () => {};
            for (const listener of this.listeners[type] || []) {
                listener.call(this, event);
                if (event.stopped) return;
            }
            this.parentElement?.dispatch(type, event);
        }
        click() { this.dispatch('click'); }
        focus() { document.activeElement = this; }
        contains(node) { return node === this || this.children.some(child => child.contains(node)); }
        get previousElementSibling() {
            return this.parentElement.children[this.parentElement.children.indexOf(this) - 1];
        }
        get nextElementSibling() {
            return this.parentElement.children[this.parentElement.children.indexOf(this) + 1];
        }
        querySelectorAll(selector) {
            const all = this.children.flatMap(child => [child, ...child.querySelectorAll('*')]);
            if (selector === '*') return all;
            if (selector === 'button.theme') return all.filter(child => child.classList.contains('theme'));
            if (selector === '.theme-selected') return all.filter(child => child.classList.contains('theme-selected'));
            throw new Error(`Unhandled selector: ${selector}`);
        }
        querySelector(selector) {
            if (selector === 'button') return this.children[0] || null;
            if (selector === 'li:first-child button') return this.children[0]?.children[0] || null;
            if (selector === 'li:last-child button') return this.children.at(-1)?.children[0] || null;
            if (selector.startsWith('button#')) return this.querySelectorAll('*').find(child => child.id === selector.slice(7)) || null;
            throw new Error(`Unhandled selector: ${selector}`);
        }
    }
    const popup = new Element('mdbook-theme-list');
    popup.setAttribute('role', 'menu');
    const buttons = {};
    for (const theme of ['default_theme', 'light', 'rust', 'coal', 'navy', 'ayu']) {
        const li = new Element();
        li.setAttribute('role', 'none');
        const button = new Element('mdbook-theme-' + theme, 'theme');
        button.textContent = theme[0].toUpperCase() + theme.slice(1);
        button.setAttribute('role', 'menuitem');
        li.append(button);
        popup.append(li);
        buttons[theme] = button;
    }
    const toggle = new Element('mdbook-theme-toggle');
    const page = new Element('', 'light sidebar-visible js');
    const meta = new Element();
    const styles = Object.fromEntries(['ayu-highlight', 'tomorrow-night', 'highlight'].map(name => [name, new Element()]));
    const events = new Element();
    document.createElement = () => new Element();
    document.addEventListener = events.addEventListener.bind(events);
    document.documentElement = page;
    document.getElementById = id => [popup, toggle, ...popup.querySelectorAll('*')].find(node => node.id === id) || null;
    document.querySelector = selector => {
        if (selector === 'html') return page;
        if (selector === 'meta[name="theme-color"]') return meta;
        return styles[selector.replace('#mdbook-', '').replace('-css', '')];
    };
    const storage = new Map(saved === null ? [] : [['mdbook-theme', saved]]);
    const localStorage = {
        getItem(key) { if (blocked) throw new Error('Storage denied'); return storage.get(key) ?? null; },
        setItem(key, value) { if (blocked) throw new Error('Storage denied'); storage.set(key, value); },
        removeItem(key) { if (blocked) throw new Error('Storage denied'); storage.delete(key); },
    };
    const queries = [];
    function matchMedia() {
        const query = new Element();
        query.matches = dark;
        query.onchange = null;
        queries.push(query);
        return query;
    }
    const editor = { setTheme(theme) { this.theme = theme; } };
    const context = vm.createContext({
        document, localStorage, window: { matchMedia, ace: {}, editors: [editor] },
        default_theme: dark ? 'navy' : 'light', default_dark_theme: 'navy', default_light_theme: 'light',
        setTimeout: callback => callback(), getComputedStyle: () => ({ backgroundColor: '#fff' }),
    });
    if (native) {
        // Run the generated early startup script too, including its legacy class.
        const startup = html.match(/<script>\s*(const default_theme = window\.matchMedia[\s\S]*?)<\/script>/)?.[1];
        assert.ok(startup, 'mdBook early theme startup must still exist');
        vm.runInContext(`{ ${startup} }`, context);
        vm.runInContext(nativeSource, context);
    }
    const nativeHandlers = queries.map(query => query.onchange);
    vm.runInContext(source, context);
    nativeHandlers.forEach((handler, index) => assert.equal(queries[index].onchange, handler));
    buttons['warm-dark'] = document.getElementById('mdbook-theme-warm-dark');
    return {
        popup, toggle, buttons, page, styles, editor, storage, document,
        changeOS(value) {
            dark = value;
            queries.forEach(query => { query.matches = value; });
            for (const query of [...queries]) {
                query.onchange?.({ matches: value });
                query.dispatch('change', { matches: value });
            }
        },
        key(key) { events.dispatch('keydown', { key, target: document.activeElement }); },
    };
}

test('offers six schemes plus the native system reset, retaining menu roles', () => {
    const h = harness();
    assert.deepEqual(h.popup.children.map(li => li.children[0].textContent), [
        'System preference', 'Corrode Light', 'Rust', 'Coal', 'Corrode Dark', 'Ayu', 'Warm Dark',
    ]);
    assert.equal(h.popup.attributes.role, 'menu');
    for (const li of h.popup.children) {
        assert.equal(li.attributes.role, 'none');
        assert.equal(li.children[0].attributes.role, 'menuitem');
    }
    assert.equal(h.storage.size, 0);
});

test('missing theme menu is harmless', () => {
    vm.runInNewContext(source, { document: { getElementById: () => null } });
});

test('blocked storage is harmless, including the system reset', () => {
    const h = harness({ blocked: true });
    h.buttons.default_theme.click();
});

for (const dark of [false, true]) {
    test(`native startup follows ${dark ? 'dark' : 'light'} OS without saving a preference`, integration, () => {
        const h = harness({ native: true, dark });
        assert.ok(h.page.classList.contains(dark ? 'navy' : 'light'));
        assert.ok(h.buttons.default_theme.classList.contains('theme-selected'));
        h.changeOS(!dark);
        assert.ok(h.page.classList.contains(dark ? 'light' : 'navy'));
        assert.equal(h.storage.size, 0);
    });
}

for (const theme of ['light', 'navy', 'coal', 'rust', 'ayu', 'warm-dark']) {
    test(`${theme} manual choice survives OS changes and a new page`, integration, () => {
        const h = harness({ native: true });
        h.buttons[theme].click();
        assert.equal(h.storage.get('mdbook-theme'), theme);
        h.changeOS(true);
        h.changeOS(false);
        assert.ok(h.page.classList.contains(theme));
        assert.ok(h.buttons[theme].classList.contains('theme-selected'));
        const next = harness({ native: true, dark: true, saved: h.storage.get('mdbook-theme') });
        assert.ok(next.page.classList.contains(theme));
        assert.ok(next.buttons[theme].classList.contains('theme-selected'));
        next.toggle.click();
        assert.equal(next.document.activeElement, next.buttons[theme]);
        next.changeOS(false);
        assert.ok(next.page.classList.contains(theme));
        const light = theme === 'light' || theme === 'rust';
        assert.equal(next.editor.theme, light ? 'ace/theme/dawn' : 'ace/theme/tomorrow_night');
        assert.equal(next.styles.highlight.disabled, !light);
        assert.equal(next.styles['tomorrow-night'].disabled, light || theme === 'ayu');
        assert.equal(next.styles['ayu-highlight'].disabled, theme !== 'ayu');
        next.changeOS(true);
        next.buttons.default_theme.click();
        assert.equal(next.storage.size, 0);
        assert.ok(next.page.classList.contains('navy'));
        next.changeOS(false);
        assert.ok(next.page.classList.contains('light'));
    });
}

for (const saved of ['rust', 'ayu', 'coal']) {
    test(`preserves saved native ${saved} without migration`, integration, () => {
        const expected = saved;
        const h = harness({ native: true, saved });
        assert.equal(h.storage.get('mdbook-theme'), expected);
        assert.ok(h.page.classList.contains(expected));
        assert.equal(h.page.className.split(' ').filter(name => ['light', 'navy', 'rust', 'coal', 'ayu', 'warm-dark'].includes(name)).length, 1);
        assert.ok(h.buttons[expected].classList.contains('theme-selected'));
        h.toggle.click();
        assert.equal(h.document.activeElement, h.buttons[expected]);
        h.changeOS(true);
        assert.ok(h.page.classList.contains(expected));
    });
}

test('native keyboard navigation includes native and custom items and preserves expanded state', integration, () => {
    const h = harness({ native: true });
    h.toggle.click();
    assert.equal(h.toggle.attributes['aria-expanded'], 'true');
    assert.equal(h.document.activeElement, h.buttons.light);
    h.key('ArrowDown');
    assert.equal(h.document.activeElement, h.buttons.rust);
    h.key('ArrowDown');
    assert.equal(h.document.activeElement, h.buttons.coal);
    h.key('ArrowDown');
    assert.equal(h.document.activeElement, h.buttons.navy);
    h.key('ArrowUp');
    assert.equal(h.document.activeElement, h.buttons.coal);
    h.key('Home');
    assert.equal(h.document.activeElement, h.buttons.default_theme);
    h.key('End');
    assert.equal(h.document.activeElement, h.buttons['warm-dark']);
    // Native buttons generate clicks for Enter/Space activation.
    h.document.activeElement.click();
    assert.equal(h.storage.get('mdbook-theme'), 'warm-dark');
    h.key('Escape');
    assert.equal(h.toggle.attributes['aria-expanded'], 'false');
    assert.equal(h.document.activeElement, h.toggle);
});

test('Warm Dark can switch to every native theme without leaving a stale class', integration, () => {
    for (const theme of ['light', 'navy', 'rust', 'coal', 'ayu']) {
        const h = harness({ native: true, saved: 'warm-dark' });
        h.buttons[theme].click();
        h.changeOS(true);
        assert.ok(!h.page.classList.contains('warm-dark'));
        assert.ok(h.page.classList.contains(theme));
        assert.equal(h.storage.get('mdbook-theme'), theme);
    }
});

test('native palettes have their own readable surfaces without replacing native chrome', () => {
    const css = readFileSync(path.join(root, 'book/theme/css/corrode.css'), 'utf8');
    for (const theme of ['rust', 'coal', 'ayu', 'warm-dark']) {
        const palette = css.match(new RegExp(`html\\.${theme} \\{([^}]+)\\}`))?.[1];
        assert.ok(palette, `${theme} has an independent palette`);
        for (const token of ['fg', 'code-bg', 'surface', 'surface-2', 'syntax-comment', 'syntax-keyword', 'error', 'success']) {
            assert.ok(palette.includes(`--corrode-${token}:`), `${theme} defines ${token}`);
        }
    }
    assert.match(css, /html:is\(\.navy, \.light, \.warm-dark\) \{/);
    assert.match(css, /html:is\(\.navy, \.light, \.warm-dark, \.rust, \.coal, \.ayu\) \.content :is\(\.hljs, \.ace_editor, \.result\)/);
});

test('native picker remains usable when storage is denied', integration, () => {
    const h = harness({ native: true, blocked: true });
    h.buttons.coal.click();
    assert.ok(h.page.classList.contains('coal'));
    assert.doesNotThrow(() => h.buttons.default_theme.click());
});

test('configuration uses native OS defaults and loads customization before other extras', () => {
    const config = readFileSync(path.join(root, 'book/book.toml'), 'utf8');
    assert.match(config, /^default-theme = "light"$/m);
    assert.match(config, /^preferred-dark-theme = "navy"$/m);
    assert.match(config, /^additional-js = \["theme\/js\/themes.js",/m);
});

test('built page supplies native defaults and includes the custom script in order', integration, () => {
    assert.match(html, /const default_light_theme = "light";/);
    assert.match(html, /const default_dark_theme = "navy";/);
    const scripts = [...html.matchAll(/<script src="([^"]+)"><\/script>/g)].map(match => match[1]);
    const themes = scripts.findIndex(script => /^theme\/js\/themes-[^/]+\.js$/.test(script));
    assert.ok(themes > scripts.indexOf(bookScript));
    assert.match(scripts[themes + 1], /^theme\/js\/playground-tests-/);
    assert.match(scripts[themes + 2], /^theme\/js\/code-font-/);
});
