/**
    @typedef {{
        draggable: Element,
        dropzone: Element,
        mouseCoords: { x: number, y: number }
    }} DropEventDetail

    @typedef {{
        draggable: Element,
        ghost: Element,
        mouseCoords: { x: number, y: number }
    }} DragEventDetail
*/

class DragDropInteraction extends EventTarget {
    #oldClientX = 0;
    #oldClientY = 0;

    /**
        @param {boolean} [hideTarget=true]
        @param {number} [ghostZ=1]
    */
    constructor(hideTarget = true, ghostZ = 1) {
        super();

        for (const draggable of this.draggables) {
            draggable.addEventListener("mousedown", event => {
                event.preventDefault();
                this.startDrag(draggable, event.clientX, event.clientY);
            });
        }

        document.addEventListener("mousemove", event => {
            if (this.target) {
                const { clientX, clientY } = event;

                if (!this.ghost) {
                    this.summonGhost(ghostZ);
                    this.target.classList.add("dragged");
                    this.target.hidden = hideTarget;
                }

                this.dragGhost(clientX, clientY);
                this.emitDragEvent(clientX, clientY);
            }
        });

        document.addEventListener("mouseup", event => {
            if (this.active) {
                const { clientX, clientY } = event;
                for (const dropzone of this.dropzones) {
                    const dropIt = dropzone !== this.target
                        && pointInElement(clientX, clientY, dropzone);
                    if (dropIt) {
                        this.emitDropEvent(dropzone, clientX, clientY);
                        break;
                    }
                }
                this.endDrag();
            }
            this.reset();
        });
    }

    get active() {
        return !!this.target && this.ghost;
    }

    get draggables() {
        return document.querySelectorAll(".draggable");
    }

    get dropzones() {
        return document.querySelectorAll(".dropzone");
    }

    /**
        @param {number} ghostZ;
    */
    summonGhost(ghostZ) {
        this.ghost = this.target.cloneNode(true);
        this.ghost.style.position = "fixed";
        this.ghost.style.zIndex = ghostZ;
        this.ghost.classList.add("drag-ghost");
        this.ghost.classList.remove("dropzone", "draggable");
        document.body.firstChild.before(this.ghost);
    }

    /**
        @param {number} clientX
        @param {number} clientY
    */
    dragGhost(clientX, clientY) {
        const rect = this.target.getBoundingClientRect();
        this.ghost.style.left = rect.left + (clientX - this.#oldClientX) + "px";
        this.ghost.style.top = rect.top + (clientY - this.#oldClientY) + "px";
    }

    /**
        @param {number} x
        @param {number} y
    */
    emitDragEvent(x, y) {
        const detail = {
            draggable: this.target,
            ghost: this.ghost,
            mouseCoords: { x, y },
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
        @param {number} clientX
        @param {number} clientY
    */
    startDrag(target, clientX, clientY) {
        this.target = target;
        this.#oldClientX = clientX;
        this.#oldClientY = clientY;
    }

    endDrag() {
        this.ghost?.remove();
        this.target.hidden = false;
        this.target.classList.remove("dragged");
    }

    reset() {
        this.#oldClientX = 0;
        this.#oldClientY = 0;
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
    @param {number} x
    @param {number} y
    @param {Element} element
*/
function topOrBottom(x, y, element) {
    const rect = element.getBoundingClientRect();

    if (pointInElement(x, y, element)) {
        const mid = rect.top + rect.height / 2;
        return y >= rect.top && y <= mid ? "top" : "bottom";
    }

    return "none";
}

// -------- MAIN --------

const board = document.querySelector(".board");
const cards = document.querySelectorAll(".cards");
const columns = document.querySelectorAll(".column");
const dragdrop = new DragDropInteraction(false, 2);

dragdrop.addEventListener("drop", event => {
    /** @type DropEventDetail */
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
            switch (topOrBottom(mouseCoords,x, mouseCoords.y, columnCard)) {
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
