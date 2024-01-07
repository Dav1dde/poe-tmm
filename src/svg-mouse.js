const root = document.getElementsByTagName('svg')[0];
root.style.touchAction = 'none';
const WIDTH = root.viewBox.baseVal.width;
const HEIGHT = root.viewBox.baseVal.height;

const MIN_ZOOM = 0.7;
const MAX_ZOOM = 3.0;
const ZOOM_FACTOR = 1 / 500;

let zoomLevel = 1;

let center_x = 0;
let center_y = 0;

let pointers = {};
let last_dist = null;

function update() {
    const dx = WIDTH / zoomLevel;
    const dy = HEIGHT / zoomLevel;
    const x = center_x / zoomLevel - dx/2;
    const y = center_y / zoomLevel - dy/2;

    root.setAttribute('viewBox', `${x} ${y} ${dx} ${dy}`);
}

function zoom(pixels) {
    if (pixels === 0) {
        return;
    }

    const newZoomLevel = Math.min(MAX_ZOOM, Math.max(MIN_ZOOM, zoomLevel - pixels * ZOOM_FACTOR));
    if (Math.abs(zoomLevel - newZoomLevel) < 0.001) {
        return false;
    }

    center_x -= center_x * (1 - newZoomLevel / zoomLevel);
    center_y -= center_y * (1 - newZoomLevel / zoomLevel);
    zoomLevel = newZoomLevel;

    update();
    return true;
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
        const scale_x = WIDTH / window.innerWidth;
        const scale_y = HEIGHT / window.innerHeight;
        center_x -= (event.clientX - last_position.x) * scale_x;
        center_y -= (event.clientY - last_position.y) * scale_y;
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
            zoom(pinch);
        }
        last_dist = dist;
    }

    pointers[event.pointerId] = {x: event.clientX, y: event.clientY};
}
function onPointerLeave(event) {
    delete pointers[event.pointerId];
    last_dist = null;
}

function onWheel(event) {
    if (event.ctrlKey || event.altKey) {
        return;
    }

    event.preventDefault();
    zoom(event.deltaY)
}

root.addEventListener('pointerdown', onPointerDown, {passive: true});
root.addEventListener('pointermove', onPointerMove, {passive: true});
root.addEventListener('pointerup', onPointerLeave, {passive: true});
root.addEventListener('pointercancel', onPointerLeave, {passive: true});
root.addEventListener('wheel', onWheel, {passive: false});
