const root = document.getElementsByTagName('svg')[0];
root.style.touchAction = 'none';

const MIN_ZOOM = 0.025;
const MAX_ZOOM = 0.15;

let zoomLevel = 0.035;

let center_x = 0;
let center_y = 0;

let pointers = {};
let last_dist = null;

let initialized = false;
function init() {
    const width = window.innerWidth;
    if (width === 0) {
        requestAnimationFrame(() => init());
        return;
    }
    if (initialized) {
        return;
    }

    if (width > 1500) {
        zoomLevel = 0.07;
    } else if (width > 1000) {
        zoomLevel = 0.06;
    } else if (width > 700) {
        zoomLevel = 0.05;
    }

    initialized = true;
    update();
}
init();

function update() {
    const width = window.innerWidth;
    const height = window.innerHeight;

    const dx = width / zoomLevel;
    const dy = height / zoomLevel;
    const x = center_x / zoomLevel - dx/2;
    const y = center_y / zoomLevel - dy/2;

    root.setAttribute('viewBox', `${x} ${y} ${dx} ${dy}`);
}

function onPointerDown(event) {
    pointers[event.pointerId] = {x: event.clientX, y: event.clientY};
}
function onPointerMove(event) {
    if (!pointers[event.pointerId]) {
        return;
    }
    const numPointers = Object.keys(pointers).length;
    if (numPointers === 1) {
        const last_position = pointers[event.pointerId];
        center_x -= (event.clientX - last_position.x);
        center_y -= (event.clientY - last_position.y);
        update();
    } else if (numPointers === 2) {
        const other = Object.keys(pointers).find(x => x != event.pointerId);
        const last_position = pointers[other];

        const dist = Math.hypot(
            last_position.x - event.clientX,
            last_position.y - event.clientY,
        );
        if (last_dist !== null) {
            const pinch = last_dist - dist;
            const newZoomLevel = Math.min(MAX_ZOOM, Math.max(MIN_ZOOM, zoomLevel - pinch / 1000));

            center_x -= center_x * (1 - newZoomLevel / zoomLevel);
            center_y -= center_y * (1 - newZoomLevel / zoomLevel);
            zoomLevel = newZoomLevel;

            update();
        }
        last_dist = dist;
    }

    pointers[event.pointerId] = {x: event.clientX, y: event.clientY};
}
function onPointerLeave(event) {
    delete pointers[event.pointerId];
    last_dist = null;
}

window.addEventListener('resize', () => update(), {passive: true});
root.addEventListener('pointerdown', onPointerDown, {passive: true});
root.addEventListener('pointermove', onPointerMove, {passive: true});
root.addEventListener('pointerup', onPointerLeave, {passive: true});
root.addEventListener('pointercancel', onPointerLeave, {passive: true});
