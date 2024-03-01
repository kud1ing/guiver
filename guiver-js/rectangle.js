import {Size} from "./size.js";

export class Rectangle {
    #x0;
    #y0;
    #x1;
    #y1;

    constructor(x0, y0, x1, y1) {
        this.#x0 = x0;
        this.#y0 = y0;
        this.#x1 = x1;
        this.#y1 = y1;
    }

    get height() {
        return this.#y1 - this.#y0;
    }

    get size() {
        return new Size(this.width, this.height);
    }

    set size(size) {
        this.#x1 = this.#x0 + size.width;
        this.#y1 = this.#y0 + size.height;
    }

    get width() {
        return this.#x1 - this.#x0;
    }

    get x0() {
        return this.#x0;
    }

    get y0() {
        return this.#y0;
    }

    get x1() {
        return this.#x1;
    }

    get y1() {
        return this.#y1;
    }

    static zero() {
        return new Rectangle(0.0, 0.0,0.0,0.0);
    }
}