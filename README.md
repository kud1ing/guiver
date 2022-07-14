# guiver

An experiment in GUI programming with Rust, where you pay only for what you use.

You can use immediate mode rendering where events are coming from
[druid-shell](https://github.com/linebender/druid/tree/master/druid-shell)l and rendering is handled by
[Piet](https://github.com/linebender/piet).

There is an optional widget manager, which handels retained mode widgets.
There are no event handlers, widgets are decoupled from the model data with message passing, inspired by
[Tcl/Tk](https://en.wikipedia.org/wiki/Tk_(software)).

Run the examples with e.g. `cargo run --example 7guis_counter`

<img width="707" alt="Bildschirmfoto 2022-07-05 um 13 04 14" src="https://user-images.githubusercontent.com/391975/177331930-3eca983d-7f1e-47e9-be97-54a786a3911b.png">

## Notes

* an application implements the `Application` trait
  * it handles widget events in `Application::handle_widget_event()`
* widgets implement the `Widget` trait
* child widgets are owned by the parent widgets (via `Box<dyn Widget>`). This allows:
  * efficient resize
  * automatic widget lifetime management


## Todo

* [ ] add `Error`: `NoSuchWidget(WidgetId)`, `CommandNotHandled`
* `WidgetManager`:
  * [ ] add `pub fn send_command_dictionary(&mut self, widget_command_dictionary: HashMap<WidgetId, WidgetCommand>)`
* [ ] `Widget`: `handle_widget_command()` => `handle_command_dictionary()`
* [ ] remove `WidgetId` from `WidgetCommand`
* `Command`:
  * [ ] add `Append(WidgetId, WidgetId)`
    * the widget manager must find the child widget
      * first among the `added_widgets`
    * the widget manager must find the parent widget
      * first among the `added_widgets`
    * the widget manager must then ask the parent widget to add the child widget
  * [ ] add `Clear(WidgetId)`
* [ ] remove children from the Ctor of `Padding`, `Row`
* [ ] `paint()`: use save, restore
* [ ] implement `Button`
* [ ] `Label`: clip painting, for the case when the rectangle is too small
* [ ] add `HorizontalAlignment`, `VerticalAlignment`
* [ ] `Widget Manager`: add tab order
* [ ] `WidgetManager`: hold (smart) pointers instead of widget IDs?
* [ ] implement [7GUIs](https://eugenkiss.github.io/7guis/tasks)
  * [ ] "Counter": use `Button`
  * [ ] "Temperature Converter"
    * [ ] implement `TextEdit`
  * [ ] "Flight Booker"
    * [ ] implement `DropdownBox`
  * [ ] "Timer"
  * [ ] "CRUD"
  * [ ] "Circle Drawer"
  * [ ] "Cells"
* [ ] allow a "no loop" optimization for static applications that only render once
* [ ] add Python bindings
* [ ] provide native widgets? 
* [ ] provide a WebAssembly demo

See also ["So you want to write a GUI framework"](https://www.cmyr.net/blog/gui-framework-ingredients.html)
