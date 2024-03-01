import {Widget} from "../../mod.js";
import {SizeConstraint} from "../../../size_constraints.js";

export class Row extends Widget {
    #child_widgets;
    #dom_element_div;
    #size_constraints;
    #spacing;
    #vertical_alignment;

    constructor(vertical_alignment, spacing) {
        super();
        this.#child_widgets = [];
        this.#dom_element_div = document.createElement("div");
        this.#size_constraints = SizeConstraint.unbounded();
        this.#spacing = spacing;
        this.#vertical_alignment = vertical_alignment;
    }

    add_child(child_widget){
        // Add the child widget to the list of child widgets.
        this.#child_widgets.push(child_widget);

        // Add the child widget to the DIV.
        child_widget.mount_to(this.#dom_element_div)

        // Layout all child widgets.
        this.#layout_child_widgets();
    }

    apply_size_constraints(size_constraints) {
        this.#size_constraints = size_constraints;

        // Layout the child widgets.
        this.#layout_child_widgets();
    }

    #layout_child_widgets() {
        // TODO
        /*
        // Create the child size constraints.
        let child_size_constraints =
            SizeConstraints::new(Size::ZERO, *self.core.size_constraints.maximum());

        let mut child_and_spacing_size_sum = Size::ZERO;
        let mut flex_factor_sum: u16 = 0;

        // First pass over the child widgets.
        for (i, child_widget) in &mut self.child_widgets.iter().enumerate() {
            // Apply the size constraints to the current child widget.
            let child_size = RefCell::borrow_mut(child_widget)
                .borrow_mut()
                .apply_size_constraints(child_size_constraints);

            // Update the sum of child and spacing sizes.
            // Include the child widget's height.
            child_and_spacing_size_sum.height =
                child_and_spacing_size_sum.height.max(child_size.height);

            // Add the spacer to child and spacing sizes.
            if i > 0 {
                child_and_spacing_size_sum.width += self.spacing;
            }

            // Get the child widget's flex factor.
            let flex_factor = RefCell::borrow(child_widget).borrow().flex_factor();

            // The child widget does not have a flex factor.
            if flex_factor == 0 {
                // Add the child widget's width.
                child_and_spacing_size_sum.width += child_size.width;
            }
            // The child widget does have a flex factor.
            else {
                // Do not add the child widget's width. It will grab the remaining width together
                // with all other widgets having a flex factor.

                // Add the child widget's flex factor.
                flex_factor_sum += flex_factor;
            }
        }

        // The child widgets do not have a flex factor.
        if flex_factor_sum == 0 {
            // Set the parent size to the sum of the child and spacing sizes.
            this.rectangle = this.rectangle.with_size(child_and_spacing_size_sum);
        }
        // The child widgets do have a flex factor.
        else {
            // Set the parent size to the child widget's height and the maximum width.
            this.rectangle = this.rectangle.with_size(Size::new(
                self.core.size_constraints.maximum().width,
                child_and_spacing_size_sum.height,
            ));
        }

        // Calculate the remaining width.
        let remaining_width =
            (this.rectangle.width() - child_and_spacing_size_sum.width).max(0.0);

        let mut child_x = this.rectangle.origin().x;

        // Second pass over the child widgets.
        for child_widget in &mut self.child_widgets {
            // Get the child widget's flex factor.
            let flex_factor = RefCell::borrow(child_widget).borrow().flex_factor();

            // The child widget does not have a flex factor.
            let child_size = if flex_factor == 0 {
                let size = RefCell::borrow(child_widget).borrow().rectangle().size();

                Size::new(size.width, size.height)
            }
            // The child widget does have a flex factor.
            else {
                let child_size = RefCell::borrow(child_widget).borrow().rectangle().size();

                // Devide the remaining width among the child widgets with flex factor.
                let expanded_child_size = Size::new(
                    remaining_width * (flex_factor as f64 / flex_factor_sum as f64),
                    child_size.height,
                );

                // Apply the size constraints to the current child widget.
                RefCell::borrow_mut(child_widget)
                    .borrow_mut()
                    .apply_size_constraints(SizeConstraints::tight(expanded_child_size));

                expanded_child_size
            };

            // Determine the child widget's vertical position.
            let child_y = match self.vertical_alignment {
                VerticalAlignment::Bottom => {
                    this.rectangle.origin().y
                        + (this.rectangle.size().height - child_size.height).max(0.0)
                }
                VerticalAlignment::Middle => {
                    this.rectangle.origin().y
                        + 0.5 * (this.rectangle.size().height - child_size.height).max(0.0)
                }
                VerticalAlignment::Top => this.rectangle.origin().y,
            };

            // Set the child widget's origins.
            RefCell::borrow_mut(child_widget)
                .borrow_mut()
                .set_origin(Point::new(child_x, child_y));

            child_x += child_size.width + self.spacing;
        }
        */
    }

    mount_to(parent_element) {
        parent_element.appendChild(this.#dom_element_div);
    }

    remove_child(child_widget){
        // Try to find the child widget.
        const widget_index = this.#child_widgets.indexOf(child_widget);

        // The child widget was not found.
        if (widget_index <= -1) {
            return;
        }

        // Remove the child widget from the child widget list.
        this.#child_widgets.splice(widget_index, 1);

        // Remove the child widget from the DIV.
        child_widget.unmount_from(this.#dom_element_div)

        // Layout the remaining child widgets.
        this.#layout_child_widgets();
    }

    unmount_from(parent_element) {
        parent_element.removeChild(this.#dom_element_div);
    }
}