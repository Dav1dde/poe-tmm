const root = document.getElementsByTagName('svg')[0];

const CONNECTIONS = [...root.querySelectorAll('.connections > *')]
    .map(element => element.id.substring(1).split('-').map(n => parseInt(n)));


document.adoptedStyleSheets = [new CSSStyleSheet(), new CSSStyleSheet()];

window.tree_load = function(data) {
    const css = new CSSStyleSheet();

    // Activate nodes.
    for (const node_id of data.nodes) {
        css.insertRule(`#n${node_id} { color: var(--active-color) }`);
    }

    // Activate connections.
    const nodes_set = new Set(data.nodes);
    for (const [a, b] of CONNECTIONS) {
        if (nodes_set.has(a) && nodes_set.has(b)) {
            css.insertRule(`#c${a}-${b} { color: var(--active-color) }`);
        }
    }

    // Activate ascendancy.
    css.insertRule(`
        .${window._ascendancy_name(data.classId, data.ascendancyId)}, 
        .${window._alternate_ascendancy_name(data.classId, data.alternateAscendancyId)}
    { 
        display: block !important;
    }`);

    document.adoptedStyleSheets[0] = css;
}

window.tree_highlight = function(nodes) {
    const css = new CSSStyleSheet();

    for (node of nodes) {
        css.insertRule(`#n${node} { color: var(--highlight-color) !important; stroke-opacity: 1; }`);
    }

    document.adoptedStyleSheets[1] = css;
}
