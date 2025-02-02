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
                this.startDrag(draggable, event.offsetX, event.offsetY, ghostZ);
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
                        this.emitDropEvent(dropzone);
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
        this.ghost.style.left = clientX - this.#offsetX + "px";
        this.ghost.style.top = clientY - this.#offsetY + "px";
    }

    /**
        @param {{ x: number, y: number }} mouseCoords
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
    emitDropEvent(dropzone) {
        const detail = { draggable: this.target, dropzone };
        const event = new CustomEvent("drop", { detail });
        this.dispatchEvent(event);
    }

    /**
        @param {Element} target
        @param {number} offsetX
        @param {number} offsetY
        @param {number} ghostZ
    */
    startDrag(target, offsetX, offsetY, ghostZ) {
        this.target = target;
        this.target.classList.add("dragging");

        const offsetParentRect = this.target.offsetParent.getBoundingClientRect();
        this.#offsetX = offsetX + offsetParentRect.left;
        this.#offsetY = offsetY + offsetParentRect.top;

        this.summonGhost(ghostZ);
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

// -------- MAIN --------

const dragdrop = new DragDropInteraction(document.body, false, 2);

dragdrop.addEventListener("drag", event => {
    // TODO
});

dragdrop.addEventListener("drop", event => {
    // TODO
});
