'use strict';

window.addEventListener('load', async () => {
    const fontFamily = getComputedStyle(document.documentElement)
        .getPropertyValue('--corrode-font-mono').trim();
    const editors = window.editors || [];
    editors.forEach(editor => editor.setOption('fontFamily', fontFamily));

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
