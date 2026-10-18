'use strict';

window.addEventListener('load', async () => {
    const fontFamily = getComputedStyle(document.documentElement)
        .getPropertyValue('--corrode-font-mono').trim();
    const editors = window.editors || [];
    // Override mdBook's inline size so editors share the pre's size with output.
    editors.forEach(editor => editor.setOptions({ fontFamily, fontSize: 'inherit' }));

    try {
        await document.fonts.load('16px "Zed Mono"');
    } catch {
        // Keep the monospace fallback if the bundled font cannot load.
    }
    // Ace measures characters before web fonts finish loading.
    editors.forEach(editor => {
        editor.renderer.updateFontSize();
        editor.resize(true);
    });
});
