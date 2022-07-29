# guiver 0.1

guiver is an attempt to make GUI programming with Rust simple, perhaps without perfect efficiency.

guiver can be used in immediate mode, which is inspired by [egui](https://github.com/emilk/egui). In this case it is
just a thin wrapper around [druid-shell](https://github.com/linebender/druid/tree/master/druid-shell) for event
handling/windowing and [Piet](https://github.com/linebender/piet) for rendering.

There are retained mode widgets that can be used with the help of an optional widget manager.
The widgets are decoupled from the application data via message passing, which is inspired by
[Tk commands](https://en.wikipedia.org/wiki/Tk_(software)) and egui.
Some layout widgets are inspired by [Flutter](https://flutter.dev).

If you look at the [example code](examples/), it may appear a bit verbose.
On the upside you get simple setup and simple control flow.

<img width="788" alt="Bildschirmfoto 2022-07-29 um 21 09 50" src="https://user-images.githubusercontent.com/391975/181828289-797d1d0c-449e-4e38-9d3b-af0a3d4ef335.png">

<img width="232" alt="Bildschirmfoto 2022-07-29 um 21 09 16" src="https://user-images.githubusercontent.com/391975/181828207-8c268397-f815-484a-a28f-22501fc04ec4.png">

<img width="334" alt="Bildschirmfoto 2022-07-29 um 21 08 38" src="https://user-images.githubusercontent.com/391975/181828121-526bddbf-8f9a-4e5a-8db8-2ce9ce4ec8c0.png">

<img width="293" alt="Bildschirmfoto 2022-07-29 um 21 07 20" src="https://user-images.githubusercontent.com/391975/181827939-ba986efe-b0b7-4a2a-ba09-9eedff8c7846.png">

<img width="357" alt="Bildschirmfoto 2022-07-29 um 21 06 35" src="https://user-images.githubusercontent.com/391975/181827857-42b6ffb0-9837-4bfa-9552-e4e68f68d639.png">

<img width="399" alt="Bildschirmfoto 2022-07-29 um 21 05 50" src="https://user-images.githubusercontent.com/391975/181827755-3e8e9e82-155c-4c0e-aa37-4963f1db49b0.png">

<img width="631" alt="Bildschirmfoto 2022-07-29 um 21 04 20" src="https://user-images.githubusercontent.com/391975/181827655-c8945f84-be44-47ab-836c-64a1c41bc945.png">

<img width="315" alt="Bildschirmfoto 2022-07-25 um 21 57 15" src="https://user-images.githubusercontent.com/391975/180863911-98ca0572-d700-426a-be9b-3f96c708f478.png">

## Backlog

* [ ] resize of `7guis_temperature_converter` is slow. correlated to `TextInput`?
* [ ] add layout widget `Stacked` + `Positioned`
* [ ] add layout widget `Expanded`
  * [ ] rethink layouting in `Row`, `Column`
* [ ] `TextInput`: if a text is too large to fit in, the size of the text input should not increase but truncate 
* `TextInput`:
  * [ ] accept paste
  * [ ] display a caret
  * [ ] arrow keys should move the caret
  * support text selection:
    * [ ] Shift + cursor movement
    * [ ] Ctrl + A
  * [ ] Ctrl+X should cut the text
  * [ ] Ctrl-C should copy the text
* `WidgetManager`: add tab order
  * [ ] define tab order
    * how? explicitly or implicitly?
  * [ ] use tab order when tab key is pressed
* [ ] support a concept of layers/z-order for dropdown boxes, tooltips etc.
* [ ] implement [7GUIs](https://eugenkiss.github.io/7guis/tasks)
  * [ ] 7GUIs "Flight Booker"
    * [ ] implement `DropdownBox`
  * [ ] 7GUIs "Timer"
  * [ ] 7GUIs "CRUD"
  * [ ] 7GUIs "Circle Drawer"
  * [ ] 7GUIs "Cells"
* [ ] add Redmond 31 widgets?
* [ ] `WidgetManager`: implement `collect_garbage()`
  * remove all widgets that do not have the main widget as ancestor
* [ ] add Python bindings
* [ ] provide native widgets? 
* [ ] provide a WebAssembly demo

See also ["So you want to write a GUI framework"](https://www.cmyr.net/blog/gui-framework-ingredients.html)
