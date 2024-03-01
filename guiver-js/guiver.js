import {Size} from "./size.js";
import {SizeConstraint} from "./size_constraints.js";

export function set_main_widget(widget) {
    function resize_main_widget() {
        const window_size = new Size(window.innerWidth, window.innerHeight);
        const size_constraint = new SizeConstraint(window_size, window_size);
        widget.apply_size_constraints(size_constraint);
    }

    // Make sure the body has no margin.
    document.body.setAttribute("style","margin:0;");

    // Add the main widget to the body.
    widget.mount_to(document.body);

    // Resize the main widget,
    resize_main_widget();

    // Make sure that window resizes are applied to the main widget.
    window.onresize = resize_main_widget;
}