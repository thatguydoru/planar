"use strict";

import { DragDropInteraction, DropDetail } from 'lib/interactions.js';
import { topOrBottom } from 'lib/utils.js';

const dragdrop = new DragDropInteraction(false, 2);

dragdrop.addEventListener("drop", event => {
    /** @type DropDetail */
    const { draggable, dropzone, mouseCoords } = event.detail;
    const isCard = draggable.classList.contains("card");
    const isColumn = dropzone.classList.contains("column");

    if (isCard && isColumn) {
        const cards = dropzone.querySelectorAll(".card");
        if (cards.length === 0) {
            dropzone.append(draggable);
            return;
        }
        for (const card of cards) {
            switch (topOrBottom(mouseCoords.x, mouseCoords.y, card)) {
                case "top":
                    card.before(draggable);
                    return;
                case "bottom":
                    card.after(draggable);
                    return;
                case "none":
                    dropzone.append(draggable);
                    return;
            }
        }
    }
});
