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

<img width="300" alt="Bildschirmfoto 2022-07-24 um 13 41 24" src="https://user-images.githubusercontent.com/391975/180645285-1a287970-d6f1-4b83-986f-c9188a06b9b6.png">

<img width="276" alt="Bildschirmfoto 2022-07-24 um 11 59 31" src="https://user-images.githubusercontent.com/391975/180641972-a69f2a21-681d-4bfd-b972-e26e8d1932e0.png">

<img width="314" alt="Bildschirmfoto 2022-07-24 um 11 58 56" src="https://user-images.githubusercontent.com/391975/180641976-111d6751-acc1-4910-9b01-2f421053a463.png">

<img width="400" alt="Bildschirmfoto 2022-07-24 um 12 59 33" src="https://user-images.githubusercontent.com/391975/180644027-351f4490-4038-4629-9392-2cde4fa91c9c.png">

<img width="315" alt="Bildschirmfoto 2022-07-25 um 21 57 15" src="https://user-images.githubusercontent.com/391975/180863911-98ca0572-d700-426a-be9b-3f96c708f478.png">


## Backlog

* `TextInput`:
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
* [ ] `WidgetManager`: add tab order
* [ ] add Redmond 31 widgets
* add to `Style`:
  * [ ] `font`
* [ ] `WidgetManager`: implement `collect_garbage()`
  * remove all widgets that do not have the main widget as ancestor
* [ ] add Python bindings
* [ ] provide native widgets? 
* [ ] provide a WebAssembly demo

See also ["So you want to write a GUI framework"](https://www.cmyr.net/blog/gui-framework-ingredients.html)
