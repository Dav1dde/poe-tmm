const root = document.getElementsByTagName('svg')[0];

const CONNECTIONS = [...root.querySelectorAll('.connections > *')]
    .map(element => element.id.substring(1).split('-'));

function load(data) {
    const css = new CSSStyleSheet();

    // Activate nodes.
    for (const node_id of data.nodes) {
        css.insertRule(`#n${node_id} { color: ${window._ACTIVE_COLOR} }`);
    }

    // Activate connections.
    const nodes_set = new Set(data.nodes);
    for (const [a, b] of CONNECTIONS) {
        if (nodes_set.has(a) && nodes_set.has(b)) {
            css.insertRule(`#c${a}-${b} { color: ${window._ACTIVE_COLOR} }`);
        }
    }

    // Activate ascendancy.
    css.insertRule(`
        .${window._ascendancy_name(data.classId, data.ascendancyId)}, 
        .${window._alternate_ascendancy_name(data.classId, data.alternateAscendancyId)}
    { 
        display: block !important;
    }`);

    document.adoptedStyleSheets = [css];
}

window.tree_load = load;
