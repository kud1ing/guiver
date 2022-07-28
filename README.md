# guiver 0.1.0

guiver is an approach to making GUI programming with Rust simple, with the downside of not perfect efficiency.

guiver can be used in immediate mode, which is inspired by [egui](https://github.com/emilk/egui). In this case it is
just a thin wrapper around [druid-shell](https://github.com/linebender/druid/tree/master/druid-shell) for event
handling/windowing and [Piet](https://github.com/linebender/piet) for rendering.

There are retained mode widgets that can be used with the help of an optional widget manager.
The widgets are decoupled from the application data via message passing, which is inspired by
[Tk commands](https://en.wikipedia.org/wiki/Tk_(software)) and egui.
In a way the widget manager acts like a garbage collecting subsystem.

If you look at the example code, it appears a bit verbose.
On the upside you get simple setup and simple control flow.

<img width="179" alt="Bildschirmfoto 2022-07-25 um 19 35 35" src="https://user-images.githubusercontent.com/391975/180839538-64f2a0a7-6dd8-4e1f-bdd7-ddeac2e98ed7.png">

<img width="325" alt="Bildschirmfoto 2022-07-25 um 19 59 34" src="https://user-images.githubusercontent.com/391975/180843587-fbaa38fb-92dc-4201-98cd-db387aa122b7.png">

<img width="300" alt="Bildschirmfoto 2022-07-25 um 22 22 21" src="https://user-images.githubusercontent.com/391975/180867499-e1ea127a-acbd-4bcc-b614-7949c6bf6ad1.png">

<img width="348" alt="Bildschirmfoto 2022-07-25 um 22 23 16" src="https://user-images.githubusercontent.com/391975/180867615-52a6b8fa-1066-462d-af5b-5ab630bfe345.png">

<img width="347" alt="Bildschirmfoto 2022-07-25 um 22 23 55" src="https://user-images.githubusercontent.com/391975/180867695-c977d027-5eb4-45a3-9d21-279ac4d3a3f6.png">

<img width="400" alt="Bildschirmfoto 2022-07-25 um 22 24 23" src="https://user-images.githubusercontent.com/391975/180867766-5d564ba6-1507-49e2-907c-1ec0f43dbcc5.png">

<img width="315" alt="Bildschirmfoto 2022-07-25 um 21 57 15" src="https://user-images.githubusercontent.com/391975/180863911-98ca0572-d700-426a-be9b-3f96c708f478.png">

## Backlog

* [ ] create `Stroke` with `PaintBrush` and width
* use `Stroke` for debug rendering:
  * [ ] `Button`
  * [ ] `Text`
  * [ ] `TextInput`
  * [ ] `Center`
  * [ ] `Column`
  * [ ] `Padding`
  * [ ] `Row`
* `TextInput`:
  * [ ] uses `HorizontalAlignment`
  * [ ] handle a command to set the `HorizontalAlignment`
  * [ ] accept paste
  * [ ] display a caret
  * [ ] arrow keys should move the caret
  * support text selection:
    * [ ] Shift + cursor movement
    * [ ] Ctrl + A
  * [ ] Ctrl+X should cut the text
  * [ ] Ctrl-C should copy the text
* [ ] support a concept of layers/z-order for dropdown boxes, tooltips etc.
* [ ] implement [7GUIs](https://eugenkiss.github.io/7guis/tasks)
  * [ ] 7GUIs "Flight Booker"
    * [ ] implement `DropdownBox`
  * [ ] 7GUIs "Timer"
  * [ ] 7GUIs "CRUD"
  * [ ] 7GUIs "Circle Drawer"
  * [ ] 7GUIs "Cells"
* `WidgetManager`: tab order
  * [ ] define tab order (explicitly or implicitly?)
  * [ ] use tab order when tab is pressed
* [ ] add Redmond 31 widgets
* add to `Style`:
  * [ ] `font`
* [ ] `WidgetManager`: implement `collect_garbage()`
  * remove all widgets that do not have the main widget as ancestor
* [ ] add Python bindings
* [ ] provide native widgets? 
* [ ] provide a WebAssembly demo

See also ["So you want to write a GUI framework"](https://www.cmyr.net/blog/gui-framework-ingredients.html)
