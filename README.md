# guiver

An experiment in GUI programming with Rust.

guiver can be used in immediate mode, which is inspired by [egui](https://github.com/emilk/egui). Events from
[druid-shell](https://github.com/linebender/druid/tree/master/druid-shell)
are handled in `Application::handle_user_event()`. Rendering via [Piet](https://github.com/linebender/piet)
can be done in `Application::paint()`.

There are widgets that can be used with an optional widget manager.
The widgets are decoupled from the application data via message passing, which is inspired by
[Tk commands](https://en.wikipedia.org/wiki/Tk_(software)).

The examples can be run with e.g. `cargo run --example 7guis_counter`

<img width="707" alt="Bildschirmfoto 2022-07-05 um 13 04 14" src="https://user-images.githubusercontent.com/391975/177331930-3eca983d-7f1e-47e9-be97-54a786a3911b.png">


## Backlog

* `WidgetManager`: use `Rc<RefCel<...>>`:
  * [ ] `send_commands()`: handle `WidgetManagerCommand::SetMainWidget(...)`
  * [ ] add `WidgetCommand::AppendChild(Rc<RefCel<...>>)`
  * [ ] add `WidgetManagerCommand::AppendChild(...)`
  * [ ] remove children from the Ctor of `Padding`, `Row`
  * [ ] add a function to remove widgets without parents
* [ ] `paint()`: use save, restore
* [ ] implement `Button`
* [ ] `Label`: clip painting, for the case when the rectangle is too small
* [ ] add `HorizontalAlignment`, `VerticalAlignment`
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
* [ ] `WidgetManager`: add tab order
* [ ] `WidgetManager`: use focused widget
* [ ] allow a "no loop" optimization for static applications that only render once
* [ ] add Python bindings
* [ ] provide native widgets? 
* [ ] provide a WebAssembly demo

See also ["So you want to write a GUI framework"](https://www.cmyr.net/blog/gui-framework-ingredients.html)
