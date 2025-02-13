"use strict";

/**
    @param {number} x
    @param {number} y
    @param {Element} element
*/
export function pointInElement(x, y, element) {
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
export function topOrBottom(x, y, element) {
    const rect = element.getBoundingClientRect();

    if (pointInElement(x, y, element)) {
        const mid = rect.top + rect.height / 2;
        return y >= rect.top && y < mid ? "top" : "bottom";
    }

    return "none";
}
