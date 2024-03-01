import {Rectangle} from "../rectangle.js";

export class Widget {
    #rectangle;

    constructor() {
        this.#rectangle = Rectangle.zero();
    }

    apply_size_constraints(size_constraints) {
    }

    mount_to(parent_element) {
    }

    get rectangle() {
        return this.#rectangle;
    }

    set rectangle(rectangle) {
        this.#rectangle = rectangle;
    }

    unmount_from(parent_element) {
    }
}