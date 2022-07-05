# guiver

An experiment in simple GUI programming with Rust:
* [Piet](https://github.com/linebender/piet) is used for rendering
* (multipass) immediate mode rendering is used, inspired by [egui](https://github.com/emilk/egui)
* but also retained mode widgets with message passing are provided, inspired by [Tcl/Tk](https://en.wikipedia.org/wiki/Tk_(software))

<img width="716" alt="Bildschirmfoto 2022-07-05 um 12 55 06" src="https://user-images.githubusercontent.com/391975/177312581-99097776-67a1-433a-9b6e-05efd1ddf911.png">

## Notes

* an application implements the `Application` trait
* an application is in full control:
  * it handles widget events in `Application::handle_widget_event()`
* widgets implement the `Widget` trait
* the `WidgetManager` owns all widgets


## Todo

* [ ] publish a crate
* Layout:
  * [ ] multiple single child layout widgets or only one `Container`?
  * [ ] Layout Widgets need to:
    * [ ] get the `Rect`s of all its children
      * `WidgetRequest`?
    * [ ] set the `Rect`s of all its children
      * `WidgetCommand`
      * [ ] what about `Label`s? their minimum rectangle is driven by the text and the font 
        * `tight`?
    * [ ] `PaintBrush`
* [ ] implement [7GUIs](https://eugenkiss.github.io/7guis/tasks)
  * [ ] "Counter"
    * [ ] implement `Button`, `Row`, `Column`, `Spacer`, `Padding`
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
* [ ] provide a WebAssembly demo

See also ["So you want to write a GUI framework"](https://www.cmyr.net/blog/gui-framework-ingredients.html)
