import {Rectangle} from "../rectangle.js";
import {Widget} from "./mod.js";

const xmlns = "http://www.w3.org/2000/svg";

export class Placeholder extends Widget {
    #desired_size;
    #dom_element_div;

    constructor(desired_size) {
        super();
        this.#desired_size = desired_size;

        this.#dom_element_div = document.createElement("div");
    }

    apply_size_constraints(size_constraints) {
        // Calculate the new size.
        const new_size = this.#desired_size.clamp(
            size_constraints.minimum,
            size_constraints.maximum,
        );

        // Adjust the widget rectangle.
        this.rectangle.size = new_size;

        // Adjust the DIV size.
        this.#dom_element_div.style.width = new_size.width;
        this.#dom_element_div.style.height = new_size.height;

        // Update the SVG.
        this.#update_svg();
    }

    mount_to(parent_element) {
        parent_element.appendChild(this.#dom_element_div);
    }

    #update_svg() {
        const svg = document.createElementNS(xmlns,"svg");
        svg.setAttribute("width", this.#dom_element_div.offsetWidth);
        svg.setAttribute("height", this.#dom_element_div.offsetHeight);
        svg.setAttribute("preserveAspectRatio", "none");
 
        const style = "stroke:black;stroke-width:1;fill:none;stroke-dasharray:5;";
        const width_string = (this.#dom_element_div.offsetWidth - 1.0).toString();
        const height_string = (this.#dom_element_div.offsetHeight - 1.0).toString();

        const rect = document.createElementNS(xmlns,"rect");
        rect.setAttribute("style", style);
        rect.setAttribute("x", "0.5");
        rect.setAttribute("y", "0.5");
        rect.setAttribute("width", width_string);
        rect.setAttribute("height", height_string);
        svg.appendChild(rect);

        const line1 = document.createElementNS(xmlns,"line");
        line1.setAttribute("style", style);
        line1.setAttribute("x1", "0.5");
        line1.setAttribute("y1", "0.5");
        line1.setAttribute("x2", width_string);
        line1.setAttribute("y2", height_string);
        svg.appendChild(line1);

        const line2 = document.createElementNS(xmlns,"line");
        line2.setAttribute("style", style);
        line2.setAttribute("x1", "0.5");
        line2.setAttribute("y1", height_string);
        line2.setAttribute("x2", width_string);
        line2.setAttribute("y2", "0.5");
        svg.appendChild(line2);

        // TODO: use `replaceChild(newChild, oldChild)`?
        this.#dom_element_div.innerHTML = "";
        this.#dom_element_div.appendChild(svg);
    }

    unmount_from(parent_element) {
        parent_element.removeChild(this.#dom_element_div);
    }
}