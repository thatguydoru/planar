import { DragDropInteraction } from 'lib/interactions.js';
import { topOrBottom } from 'lib/utils.js';

const dragdrop = new DragDropInteraction(false, 2);

dragdrop.addEventListener("drop", event => {
    const { draggable, dropzone, mouseCoords } = event.detail;
    const isCard = draggable.classList.contains("card");
    const isColumn = dropzone.classList.contains("column");

    if (isCard && isColumn) {
        const cards = dropzone.querySelectorAll(".card");
        if (cards.length === 0) {
            dropzone.append(draggable);
            return;
        }
        for (const columnCard of cards) {
            switch (topOrBottom(mouseCoords.x, mouseCoords.y, columnCard)) {
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
