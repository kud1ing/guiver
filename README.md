# guiver

An experiment in GUI programming with Rust.

guiver can be used in immediate mode, which is inspired by [egui](https://github.com/emilk/egui). In this case it is
just a thin wrapper around [druid-shell](https://github.com/linebender/druid/tree/master/druid-shell) for event
handling/windowing and [Piet](https://github.com/linebender/piet) for rendering.

There are retained mode widgets that can be used with an optional widget manager.
The widgets are decoupled from the application data via message passing, which is inspired by
[Tk commands](https://en.wikipedia.org/wiki/Tk_(software)).

The examples can be run with `cargo run --example EXAMPLE_NAME`

<img width="222" alt="Bildschirmfoto 2022-07-24 um 11 59 16" src="https://user-images.githubusercontent.com/391975/180641967-f69c5227-bba7-453d-92ef-cd94552d3529.png">

<img width="300" alt="Bildschirmfoto 2022-07-24 um 13 41 24" src="https://user-images.githubusercontent.com/391975/180645285-1a287970-d6f1-4b83-986f-c9188a06b9b6.png">

<img width="276" alt="Bildschirmfoto 2022-07-24 um 11 59 31" src="https://user-images.githubusercontent.com/391975/180641972-a69f2a21-681d-4bfd-b972-e26e8d1932e0.png">

<img width="314" alt="Bildschirmfoto 2022-07-24 um 11 58 56" src="https://user-images.githubusercontent.com/391975/180641976-111d6751-acc1-4910-9b01-2f421053a463.png">

<img width="400" alt="Bildschirmfoto 2022-07-24 um 12 59 33" src="https://user-images.githubusercontent.com/391975/180644027-351f4490-4038-4629-9392-2cde4fa91c9c.png">

## Backlog

* [ ] implement `Button` widget
  * [ ] `paint()`: use save, restore, clip painting
* [ ] 7GUIs "Counter": use `Button`
* [ ] add `Style:
  * `accent_color`
  * `font`
  * `spacing`
  * [ ] implement Default()
  * [ ] add it to `WidgetManager`
* [ ] implement a text input widget
  * [ ] paste
  * [ ] enter text
  * [ ] cursor movement with arrow keys
  * [ ] backspace deletes text
  * [ ] select text
  * [ ] Ctrl+X deletes text
  * [ ] Ctrl-C
* [ ] `WidgetManager`: track the focused widget
  * [ ] unfocus any previously focused widget
  * [ ] use `Style::accent_color`
* [ ] `Label::paint()`: use save, restore, clip painting
* [ ] implement [7GUIs](https://eugenkiss.github.io/7guis/tasks)
  * [ ] 7GUIs "Temperature Converter"
    * [ ] implement `TextEdit`
  * [ ] 7GUIs "Flight Booker"
    * [ ] implement `DropdownBox`
  * [ ] 7GUIs "Timer"
  * [ ] 7GUIs "CRUD"
  * [ ] 7GUIs "Circle Drawer"
  * [ ] 7GUIs "Cells"
* [ ] add Redmond 31 widgets
* [ ] `WidgetManager`: implement `collect_garbage()`
  * remove all widgets that do not have the main widget as ancestor
* [ ] `WidgetManager`: add tab order
* [ ] add `HorizontalAlignment`, `VerticalAlignment`
* [ ] allow a "no loop" optimization for static applications that only render once
* [ ] add Python bindings
* [ ] provide native widgets? 
* [ ] provide a WebAssembly demo

See also ["So you want to write a GUI framework"](https://www.cmyr.net/blog/gui-framework-ingredients.html)
