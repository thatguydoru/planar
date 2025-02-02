/**
    @typedef {{
        draggable: Element,
        dropzone: Element,
        mouseCoords: { x: number, y: number }
    }} DropDetail

    @typedef { x: number, y: number } Point
*/

class DragDropInteraction extends EventTarget {
    #offsetX = 0;
    #offsetY = 0;

    /**
        @param {Element} root
        @param {boolean} [hideTarget=true]
        @param {number} [ghostZ=1]
    */
    constructor(root, hideTarget = true, ghostZ = 1) {
        super();
        this.root = root;

        for (const draggable of this.draggables) {
            draggable.addEventListener("mousedown", event => {
                event.preventDefault();
                event.stopPropagation();
                this.startDrag(draggable, event.offsetX, event.offsetY);
                this.summonGhost(ghostZ);
                this.dragGhost(event.clientX, event.clientY);
                draggable.hidden = hideTarget;
            });
        }

        this.root.addEventListener("mousemove", event => {
            if (this.active) {
                const { clientX, clientY } = event;
                this.dragGhost(clientX, clientY);
                this.emitDragEvent({ x: clientX, y: clientY });
            }
        });

        this.root.addEventListener("mouseup", event => {
            if (this.active) {
                for (const dropzone of this.dropzones) {
                    const dropIt = dropzone !== this.target
                        && pointInElement(event.clientX, event.clientY, dropzone);
                    if (dropIt) {
                        this.emitDropEvent(dropzone, event.clientX, event.clientY);
                        break;
                    }
                }
                this.endDrag();
                this.reset();
            }
        });
    }

    get active() {
        return !!this.target;
    }

    get draggables() {
        return this.root.querySelectorAll(".draggable");
    }

    get dropzones() {
        return this.root.querySelectorAll(".dropzone");
    }

    /**
        @param {number} ghostZ;
    */
    summonGhost(ghostZ) {
        this.ghost = this.target.cloneNode(true);
        this.ghost.style.position = "fixed";
        this.ghost.style.zIndex = ghostZ;
        this.ghost.classList.add("drag-ghost");
        this.ghost.classList.remove("dropzone", "dragging", "draggable");
        this.root.append(this.ghost);
    }

    /**
        @param {number} clientX
        @param {number} clientY
    */
    dragGhost(clientX, clientY) {
        const offsetParentRect = this.ghost.offsetParent.getBoundingClientRect();
        this.ghost.style.left =
            clientX - this.#offsetX - offsetParentRect.left + "px";
        this.ghost.style.top =
            clientY - this.#offsetY - offsetParentRect.top + "px";
    }

    /**
        @param {Point} mouseCoords
    */
    emitDragEvent(mouseCoords) {
        const detail = {
            draggable: this.target,
            ghost: this.ghost,
            mouseCoords,
        };
        const event = new CustomEvent("drag", { detail });
        this.dispatchEvent(event);
    }

    /**
        @param {Element} dropzone
    */
    emitDropEvent(dropzone, x, y) {
        const detail = {
            draggable: this.target,
            dropzone,
            mouseCoords: { x, y },
        };
        const event = new CustomEvent("drop", { detail });
        this.dispatchEvent(event);
    }

    /**
        @param {Element} target
        @param {number} offsetX
        @param {number} offsetY
    */
    startDrag(target, offsetX, offsetY) {
        this.target = target;
        this.#offsetX = offsetX;
        this.#offsetY = offsetY;
        this.target.classList.add("dragging");
    }

    endDrag() {
        this.ghost.remove();
        this.target.hidden = false;
        this.target.classList.remove("dragging");
    }

    reset() {
        this.#offsetX = 0;
        this.#offsetY = 0;
        this.target = null;
        this.ghost = null;
    }
}

/**
    @param {number} x
    @param {number} y
    @param {Element} element
*/
function pointInElement(x, y, element) {
    const rect = element.getBoundingClientRect();
    const withinX = x >= rect.left && x <= rect.right;
    const withinY = y >= rect.top && y <= rect.bottom;

    return withinX && withinY;
}

/**
    @param {Point} point
    @param {Element} element
*/
function topOrBottom(point, element) {
    const rect = element.getBoundingClientRect();
    const { x, y } = point;

    if (pointInElement(x, y, element)) {
        const mid = rect.top + rect.height / 2;
        return y >= rect.top && y <= mid ? "top" : "bottom";
    }

    return "none";
}

// -------- MAIN --------

const dragdrop = new DragDropInteraction(document.body, false, 2);

dragdrop.addEventListener("drop", event => {
    /** @type DropDetail */
    const { draggable, dropzone, mouseCoords } = event.detail;
    const isCard = draggable.classList.contains("card");
    const isColumn = dropzone.classList.contains("column");

    if (isCard && isColumn) {
        const columnCards = dropzone.querySelectorAll(".card");
        if (columnCards.length === 0) {
            dropzone.append(draggable);
            return;
        }
        for (const columnCard of columnCards) {
            switch (topOrBottom(mouseCoords, columnCard)) {
                case "top":
                    columnCard.before(draggable);
                    return;
                case "bottom":
                    columnCard.after(draggable);
                    return;
                case "none":
                    dropzone.append(draggable);
            }
        }
    }
});
