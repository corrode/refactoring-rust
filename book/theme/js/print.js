'use strict';

(() => {
    let closedDetails = [];
    let snapshots = [];
    let printing = false;

    window.addEventListener('beforeprint', () => {
        if (printing) return;
        printing = true;
        closedDetails = Array.from(document.querySelectorAll(
            'main details.discussion-question:not([open])'
        ));
        closedDetails.forEach(details => { details.open = true; });

        // Ace uses positioned, clipped layers. Print the current source instead,
        // without destroying the editor or losing the reader's changes.
        snapshots = (window.editors || []).map(editor => {
            const code = document.createElement('code');
            code.className = 'print-code';
            code.textContent = editor.getValue();
            editor.container.after(code);
            return code;
        });
    });

    window.addEventListener('afterprint', () => {
        snapshots.forEach(code => code.remove());
        closedDetails.forEach(details => { details.open = false; });
        snapshots = [];
        closedDetails = [];
        printing = false;
    });
})();
