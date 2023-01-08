"use strict";

class WidgetPadding {
    #dom_element;
    #widget_id;

    constructor(widget_id) {
        this.#widget_id = widget_id;

        this.#dom_element = document.createElement("div");
        this.#dom_element.setAttribute("id", widget_id);
    }

    add_event_observation(widget_event_type, event_handler, custom_event) {
        console.log("`WidgetPadding.add_widget()`: TODO");
    }

    add_widget(widget_placement, child_widget) {
        this.#dom_element.innerHTML = '';
        this.#dom_element.appendChild(child_widget.dom_element());
    }

    dom_element() {
        return this.#dom_element;
    }

    set_value(value) {
        console.error("`WidgetPadding.set_value()`: not implemented");
    }
}

class WidgetRow {
    #dom_element;
    #widget_id;

    constructor(widget_id) {
        this.#widget_id = widget_id;

        this.#dom_element= document.createElement("div");
        this.#dom_element.setAttribute("id", widget_id);
    }

    add_event_observation(widget_event_type, event_handler, custom_event) {
        console.log("`WidgetRow.add_widget()`: TODO");
    }

    add_widget(widget_placement, child_widget) {
        this.#dom_element.appendChild(child_widget.dom_element());
    }

    dom_element() {
        return this.#dom_element;
    }

    set_value(value) {
        console.error("`WidgetRow.set_value()`: not implemented");
    }
}

class WidgetText {
    #dom_element;
    #widget_id;

    constructor(widget_id, text) {
        this.#widget_id = widget_id;

        this.#dom_element= document.createElement("span");
        this.#dom_element.setAttribute("id", widget_id);
        this.#dom_element.textContent = text;
    }

    add_event_observation(widget_event_type, event_handler, custom_event) {
        console.log("`WidgetText.add_widget()`: TODO");
    }

    add_widget(widget_placement, child_widget) {
        console.log("`WidgetText.add_widget()`: TODO");
    }

    dom_element() {
        return this.#dom_element;
    }

    set_value(value) {
        this.#dom_element.textContent = value;
    }
}

class WidgetTextButton {
    #dom_element;
    #widget_id;

    constructor(widget_id, text) {
        this.#widget_id = widget_id;

        this.#dom_element= document.createElement("button");
        this.#dom_element.setAttribute("id", widget_id);
        this.#dom_element.textContent = text;
    }

    add_event_observation(widget_event_type, event_handler, custom_event) {
        switch (widget_event_type) {
            case "WidgetEventType::Clicked":
                this.#dom_element.addEventListener("click", function () { event_handler(custom_event) });
                break;
            default:
                console.error("`WidgetTextButton.add_event_observation()`: unhandled widget event type " + widget_event_type);
        }
    }

    add_widget(widget_placement, child_widget) {
        console.log("`WidgetTextButton.add_widget()`: TODO");
    }

    dom_element() {
        return this.#dom_element;
    }

    set_value(value) {
        this.#dom_element.textContent = value;
    }
}


class WidgetManager {
    #event_handler;
    #next_widget_id;
    #widgets;

    constructor() {
        this.#next_widget_id = 0;
        this.#widgets = {};
    }

    /**
     * Handles the given commands.
     * @param command_tuples
     */
    handle_commands(command_tuples) {
        for (let command_tuple of command_tuples) {
            const command = command_tuple[0];
            const widget_id = command_tuple[1];

            switch (command) {
                case "Command::SetValue":
                    if (!(widget_id in this.#widgets)) {
                        console.error("Command::AddEventObservation: no widget with ID " + widget_id);
                        return;
                    }

                    this.#widgets[widget_id].set_value(command_tuple[2]);
                    break;
                case "Command::CreateWidget":
                    const widget_type = command_tuple[2];
                    switch (widget_type) {
                        case "WidgetType::Padding":
                            this.#widgets[widget_id] = new WidgetPadding(widget_id);
                            break;
                        case "WidgetType::Row":
                            this.#widgets[widget_id] = new WidgetRow(widget_id);
                            break;
                        case "WidgetType::Text":
                            this.#widgets[widget_id] = new WidgetText(widget_id, command_tuple[3]);
                            break;
                        case "WidgetType::TextButton":
                            this.#widgets[widget_id] = new WidgetTextButton(widget_id, command_tuple[3]);
                            break;
                        default:
                            console.log("TODO: `handle_commands(CreateWidget)`" + widget_type);
                    }
                    break;
                case "Command::SetMainWidget":
                    if (!(widget_id in this.#widgets)) {
                        console.error("Command::SetMainWidget: no widget with ID " + widget_id);
                        return;
                    }

                    document.body.innerHTML = '';
                    document.body.appendChild(this.#widgets[widget_id].dom_element());
                    break;
                case "Command::AddChild":
                    const add_child_parameters = command_tuple[1];
                    const parent_widget_id = add_child_parameters["parent_widget_id"];

                    if (!(parent_widget_id in this.#widgets)) {
                        console.error("Command::AddChild: no widget with ID " + parent_widget_id);
                        return;
                    }

                    const child_widget_id = add_child_parameters["child_widget_id"];

                    if (!(child_widget_id in this.#widgets)) {
                        console.error("Command::AddChild: no widget with ID " + child_widget_id);
                        return;
                    }

                    const parent_widget = this.#widgets[parent_widget_id];
                    const child_widget = this.#widgets[child_widget_id];
                    const widget_placement = add_child_parameters["widget_placement"] ?? null;

                    parent_widget.add_widget(widget_placement, child_widget);
                    break;
                case "Command::AddEventObservation":
                    if (!(widget_id in this.#widgets)) {
                        console.error("Command::AddEventObservation: no widget with ID " + widget_id);
                        return;
                    }

                    if (this.#event_handler === null) {
                        console.error("Command::AddEventObservation: no event handle is given");
                        return;
                    }

                    this.#widgets[widget_id].add_event_observation(command_tuple[2], this.#event_handler, command_tuple[3]);
                    break;
                default:
                    console.log("TODO: `handle_commands(" + command + ")`");
            }
        }
    }

    /**
     *
     * @param event_handler
     */
    handle_events(event_handler) {
        this.#event_handler = event_handler;
    }

    /**
     * Returns the next widget ID.
     * @returns {number}
     */
    next_widget_id() {
        return this.#next_widget_id++;
    }
}