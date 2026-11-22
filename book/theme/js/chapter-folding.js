'use strict';

// Wait for mdBook to install its folding handlers and reveal the current chapter.
document.addEventListener('DOMContentLoaded', () => {
    const sidebar = document.getElementById('mdbook-sidebar');
    if (!sidebar) return;

    for (const toggle of sidebar.querySelectorAll('.chapter-fold-toggle')) {
        const chapter = toggle.closest('.chapter-item');
        const link = toggle.parentElement.querySelector('a[href]');
        if (!chapter || !link) continue;

        toggle.setAttribute('role', 'button');
        toggle.setAttribute('tabindex', '0');
        toggle.setAttribute('aria-label', `Toggle chapters under ${link.textContent.trim()}`);

        const updateExpanded = () => {
            toggle.setAttribute('aria-expanded', String(chapter.classList.contains('expanded')));
        };
        updateExpanded();
        // The native click handler runs first and owns the actual folding behavior.
        toggle.addEventListener('click', updateExpanded);
        toggle.addEventListener('keydown', event => {
            if (event.key === 'Enter' || event.key === ' ') {
                event.preventDefault();
                toggle.click();
            }
        });
    }
});
