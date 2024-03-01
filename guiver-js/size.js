export class Size {
    #height;
    #width;

    constructor(width, height) {
        this.#height = height;
        this.#width = width;
    }

    clamp(minimum, maximum) {
        const width = Math.max(Math.min(this.#width, minimum.width), maximum.width);
        const height = Math.max(Math.min(this.#height, minimum.height), maximum.height);
        return new Size(width, height);
    }

    get height() {
        return this.#height;
    }

    set height(height) {
        this.#height = height;
    }

    get width() {
        return this.#width;
    }

    set width(width) {
        this.#width = width;
    }

    static zero() {
        return new Size(0.0, 0.0);
    }
}