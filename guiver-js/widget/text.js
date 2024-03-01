import {Widget} from "./mod.js";

export class Text extends Widget {
    #dom_element_span;

    constructor(text) {
        super();
        this.#dom_element_span= document.createElement("span");
        this.#dom_element_span.textContent = text;
    }
    apply_size_constraints(size_constraints) {
        // TODO
    }

    mount_to(parent_element) {
        parent_element.appendChild(this.#dom_element_span);
    }

    set_value(value) {
        this.#dom_element_span.textContent = value;
    }

    unmount_from(parent_element) {
        parent_element.removeChild(this.#dom_element_span);
    }
}