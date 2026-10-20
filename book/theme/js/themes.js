'use strict';

(function corrodeThemes() {
    const popup = document.getElementById('mdbook-theme-list');
    if (!popup) return;

    const button = theme => document.getElementById('mdbook-theme-' + theme);
    const labels = { light: 'Corrode Light', navy: 'Corrode Dark' };
    for (const [theme, label] of Object.entries(labels)) {
        button(theme).textContent = label;
    }
    const auto = button('default_theme');
    auto.textContent = 'System preference';

    const item = document.createElement('li');
    item.setAttribute('role', 'none');
    const warm = document.createElement('button');
    warm.id = 'mdbook-theme-warm-dark';
    warm.className = 'theme';
    warm.textContent = 'Warm Dark';
    warm.setAttribute('role', 'menuitem');
    item.append(warm);
    popup.append(item);

    const isWarm = () => document.documentElement.classList.contains('warm-dark');
    function highlightWarm() {
        if (!isWarm()) return;
        document.querySelector('#mdbook-ayu-highlight-css').disabled = true;
        document.querySelector('#mdbook-highlight-css').disabled = true;
        document.querySelector('#mdbook-tomorrow-night-css').disabled = false;
        if (window.ace && window.editors) {
            window.editors.forEach(editor => editor.setTheme('ace/theme/tomorrow_night'));
        }
    }
    // Native delegated clicks own classes, persistence and selection. Its cached
    // theme ID list predates this button, so adapt only the custom-theme gaps.
    popup.addEventListener('click', highlightWarm);
    function savedTheme() {
        try { return localStorage.getItem('mdbook-theme'); } catch { return null; }
    }
    // matchMedia returns a separate MediaQueryList. Run after mdBook's existing
    // listener, which falls back to an OS theme for IDs outside its cached list.
    window.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', () => {
        if (savedTheme() === 'warm-dark') {
            // Native delegation requires className === 'theme', not a selected item.
            warm.classList.remove('theme-selected');
            warm.click();
        }
    });
    document.getElementById('mdbook-theme-toggle').addEventListener('click', () => {
        if (isWarm() && popup.style.display === 'block') warm.focus();
    });

    if (savedTheme() === 'warm-dark') warm.click();

    // Unlike native reads/writes, mdBook 0.5.4's Auto removal is not guarded.
    // If removal is denied, keep the existing choice rather than throw. On
    // success the native handler performs its normal reset (removal is idempotent).
    auto.addEventListener('click', event => {
        try {
            localStorage.removeItem('mdbook-theme');
        } catch {
            event.stopImmediatePropagation();
        }
    });
})();
