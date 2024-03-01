import {Size} from "./size.js";

export class SizeConstraint {
    #maximum;
    #minimum;

    constructor(minimum, maximum) {
        this.#maximum = maximum;
        this.#minimum = minimum;
    };

    get maximum() {
        return this.#maximum;
    }

    get minimum() {
        return this.#minimum;
    }

    static unbounded() {
        return new SizeConstraint(new Size(Number.MAX_VALUE, Number.MAX_VALUE), Size.zero());
    }
}