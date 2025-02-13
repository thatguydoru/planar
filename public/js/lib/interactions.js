"use strict";

import { pointInElement } from 'lib/utils.js';

/**
    @typedef {{
        draggable: Element,
        dropzone: Element,
        mouseCoords: { x: number, y: number }
    }} DropDetail

    @typedef {{
        draggable: Element,
        ghost: Element,
        mouseCoords: { x: number, y: number },
    }} DragDetail
*/

export class DragDropInteraction extends EventTarget {
    #oldClientX = 0;
    #oldClientY = 0;
    #oldTop = 0;
    #oldLeft = 0;

    /**
        @param {boolean} [hideTarget=true]
        @param {number} [ghostZ=1]
    */
    constructor(hideTarget = true, ghostZ = 1) {
        super();

        for (const draggable of this.draggables) {
            draggable.addEventListener("mousedown", event => {
                event.stopPropagation();
                event.preventDefault();
                const rect = draggable.getBoundingClientRect();
                this.startDrag(
                    draggable,
                    event.clientX,
                    event.clientY,
                    rect.left,
                    rect.top,
                );
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
        return this.target && this.ghost;
    }

    get draggables() {
        return document.querySelectorAll(".draggable");
    }

    get dropzones() {
        return document.querySelectorAll(".dropzone");
    }

    /**
        @param {number} zIndex
    */
    summonGhost(zIndex) {
        this.ghost = this.target.cloneNode(true);
        this.ghost.style.position = "fixed";
        this.ghost.style.zIndex = zIndex;
        this.ghost.classList.add("drag-ghost");
        this.ghost.classList.remove("dropzone", "draggable");
        document.body.insertAdjacentElement("afterbegin", this.ghost);
    }

    /**
        @param {number} clientX
        @param {number} clientY
    */
    dragGhost(clientX, clientY) {
        const x = this.#oldLeft + (clientX - this.#oldClientX);
        const y = this.#oldTop + (clientY - this.#oldClientY);
        this.ghost.style.transform = `translate(${x}px, ${y}px)`;
    }

    /**
        @param {number} x
        @param {number} y
    */
    emitDragEvent(x, y) {
        /** @type DragDetail */
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
        @param {number} x
        @param {number} y
    */
    emitDropEvent(dropzone, x, y) {
        /** @type DropDetail */
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
        @param {number} left
        @param {number} top
    */
    startDrag(target, clientX, clientY, left, top) {
        this.target = target;
        this.#oldClientX = clientX;
        this.#oldClientY = clientY;
        this.#oldLeft = left;
        this.#oldTop = top;
    }

    endDrag() {
        this.ghost?.remove();
        this.target.hidden = false;
        this.target.classList.remove("dragged");
    }

    reset() {
        this.target = null;
        this.ghost = null;
    }
}
