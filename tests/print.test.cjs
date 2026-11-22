const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const test = require('node:test');
const vm = require('node:vm');

const source = fs.readFileSync(path.join(__dirname, '../book/theme/js/print.js'), 'utf8');

function setup(withEditor = true) {
    const listeners = {};
    const closed = { open: false };
    const open = { open: true };
    const snapshots = [];
    const editor = {
        getValue: () => 'fn main() { println!("<edited>"); }',
        container: { after: code => snapshots.push(code) },
    };
    vm.runInNewContext(source, {
        window: {
            addEventListener: (name, callback) => { listeners[name] = callback; },
            ...(withEditor ? { editors: [editor] } : {}),
        },
        document: {
            querySelectorAll: selector => {
                assert.equal(selector,
                    'main details.solution:not([open]), main details.discussion-question:not([open])');
                return [closed, open].filter(details => !details.open);
            },
            createElement: tag => {
                assert.equal(tag, 'code');
                return { remove() { snapshots.splice(snapshots.indexOf(this), 1); } };
            },
        },
    });
    return { listeners, closed, open, snapshots };
}

test('print expands answers, snapshots edited source, and restores reading state', () => {
    const { listeners, closed, open, snapshots } = setup();
    listeners.beforeprint();
    assert.equal(closed.open, true);
    assert.equal(snapshots.length, 1);
    assert.equal(snapshots[0].className, 'print-code');
    assert.equal(snapshots[0].textContent, 'fn main() { println!("<edited>"); }');
    listeners.beforeprint();
    assert.equal(snapshots.length, 1);
    listeners.afterprint();
    assert.equal(closed.open, false);
    assert.equal(open.open, true);
    assert.equal(snapshots.length, 0);
    // Printing again must take a fresh snapshot and restore the same state.
    listeners.beforeprint();
    assert.equal(snapshots.length, 1);
    listeners.afterprint();
    assert.equal(closed.open, false);
});

test('printing works without Ace and tolerates an unmatched afterprint event', () => {
    const { listeners, closed } = setup(false);
    listeners.afterprint();
    listeners.beforeprint();
    assert.equal(closed.open, true);
    listeners.afterprint();
    assert.equal(closed.open, false);
});
