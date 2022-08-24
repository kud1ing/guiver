# guiver

guiver is an attempt to make GUI programming with Rust simple, but perhaps without perfect efficiency.

guiver can be used in [immediate mode](https://en.wikipedia.org/wiki/Immediate_mode_(computer_graphics)), which is
inspired by [egui](https://github.com/emilk/egui). In this case it is
just a thin wrapper around [druid-shell](https://github.com/linebender/druid/tree/master/druid-shell) for event
handling/windowing and [Piet](https://github.com/linebender/piet) for rendering.

There are [retained mode](https://en.wikipedia.org/wiki/Retained_mode) widgets that can be used with the help of an
optional widget manager.
The widgets are decoupled from the application data via message passing, that is inspired by
[Tk commands](https://www.tcl.tk/man/tcl/TkCmd/contents.html) and egui.
Some layout widgets are inspired by [Flutter](https://flutter.dev).

If you look at the [example code](examples/), it may appear a bit verbose.
On the upside you get simple setup and simple control flow.

<img width="912" alt="Bildschirmfoto 2022-08-14 um 21 09 06" src="https://user-images.githubusercontent.com/391975/184551353-b7327855-c88f-4041-8868-4b21c3494a12.png">

<img width="888" alt="Bildschirmfoto 2022-07-05 um 19 43 55" src="https://user-images.githubusercontent.com/391975/177385769-598d0fd0-c15b-4d7e-bb98-5fd46bd9d415.png">

<img width="232" alt="Bildschirmfoto 2022-07-29 um 21 09 16" src="https://user-images.githubusercontent.com/391975/181828207-8c268397-f815-484a-a28f-22501fc04ec4.png">

<img width="334" alt="Bildschirmfoto 2022-07-29 um 21 08 38" src="https://user-images.githubusercontent.com/391975/181828121-526bddbf-8f9a-4e5a-8db8-2ce9ce4ec8c0.png">

<img width="293" alt="Bildschirmfoto 2022-07-29 um 21 07 20" src="https://user-images.githubusercontent.com/391975/181827939-ba986efe-b0b7-4a2a-ba09-9eedff8c7846.png">

<img width="357" alt="Bildschirmfoto 2022-07-29 um 21 06 35" src="https://user-images.githubusercontent.com/391975/181827857-42b6ffb0-9837-4bfa-9552-e4e68f68d639.png">

<img width="399" alt="Bildschirmfoto 2022-07-29 um 21 05 50" src="https://user-images.githubusercontent.com/391975/181827755-3e8e9e82-155c-4c0e-aa37-4963f1db49b0.png">

<img width="631" alt="Bildschirmfoto 2022-07-29 um 21 04 20" src="https://user-images.githubusercontent.com/391975/181827655-c8945f84-be44-47ab-836c-64a1c41bc945.png">

<img width="315" alt="Bildschirmfoto 2022-07-25 um 21 57 15" src="https://user-images.githubusercontent.com/391975/180863911-98ca0572-d700-426a-be9b-3f96c708f478.png">

## Backlog

* [ ] `TextInput`: display a caret
  * [ ] `paint()`: draw the caret when the widget has focus
  * update `caret_character_index` = child postion.x + child_size.width + offset:
    * [ ] when the text changes
      * [ ] update `caret_x`
    * [ ] when the caret is moved:
      * [ ] update `caret_x`
      * `caret_height` in `layout()`: `child_size.height`
* [ ] `TextInput`: Meta+C should copy the (selected) text
  * [ ] add `Widget::get_selected_value() -> Option<Box<dyn Any>>`
  * [ ] implement `TextInput::get_selected_value()`
  * [ ] `WidgetManager`: call `Widget::get_selected_value()` on the focused widget on a Meta+C
  * [ ] `WidgetManager`: put the value in the clipboard
* [ ] make `Text` selectable
  * [ ] double click
  * [ ] click + drag
* `Hyperlink`:
  * adjust `handle_event()`:
    * [ ] set `is_being_clicked`
    * [ ] set `was_visited`
    * set the fonts to the underlying `Text`
      * [ ] if `is_being_clicked`
      * [ ] if `was_visited`
      * [ ] if !`was_visited`
* [ ] add a `Grid` layout widget
  * [ ] add itegration test
  * supply functionality:
    * clear: `WidgetCommand::RemoveAllChildren`
      * Nur alle Kinder entfernen?
      * Auch Tabelle implodieren?
    * remove widget:
      * RemoveWidget(ID)?
        * braucht man sowieso
      * RemoveWidgetAt(…)
        * Any? Nachteil: Box
        * x, y?
    * remove column:
      * RemoveWidget(ID)?
      * ???
    * remove row:
      * RemoveWidget(ID)?
      * ???
    * set widget to a grid cell:
      * SetWidgetAt(…)?
        * Any? Nachteil: Box
        * X, y?
* [ ] publish 0.1.1, once the updated `druid-shell` and `piet` are released
  * https://github.com/linebender/druid/issues/2236
* add integration tests:
  * `widgets()`:
    * [ ] add `Column`
    * [ ] add `Padding`
    * [ ] add `Row`
  * add for `Column`:
    * [ ] `test_apply_size_constraints()`
    * [ ] `test_handle_command()`
    * [ ] `test_handle_event()`
  * add for `Row`:
    * [ ] `test_apply_size_constraints()`
    * [ ] `test_handle_command()`
    * [ ] `test_handle_event()`
  * add for `Center`:
    * [ ] `test_apply_size_constraints()`
    * [ ] `test_handle_command()`
    * [ ] `test_handle_event()`
  * add for `Padding`:
    * [ ] `test_apply_size_constraints()`
    * [ ] `test_handle_command()`
    * [ ] `test_handle_event()`
* add unit tests:
  * add for `Button`:
    * [ ] `test_apply_size_constraints()`
    * [ ] `test_handle_command()`
    * [ ] `test_handle_event()`
  * add for `Placeholder`:
    * [ ] `test_handle_command()`
    * [ ] `test_handle_event()`
  * add for `Text`:
    * [ ] `test_handle_command()`
    * [ ] `test_handle_event()`
  * add for `TextInput`:
    * [ ] `test_apply_size_constraints()`
    * [ ] `test_handle_command()`
    * [ ] `test_handle_event()`
* [ ] add a `Table` widget
* [ ] example `layout_expanded_row_column.rs`: make the row not grab all height
* `WidgetManager`: tab order:
  * [ ] explicitly define a tab order
    * [ ] how?
  * [ ] use tab order when tab key is pressed
* [ ] all layout widgets need to clip too big child widgets
* [ ] add layout widget `Stacked` + `Positioned`
* [ ] `TextInput`: if a text is too large to fit in, the size of the text input should not increase but truncate
* [ ] `TextInput`: arrow keys should move the caret
* [ ] `TextInput`: Shift + arrow keys should de/select text
* [ ] `TextInput`: double click should select the text
* support text selection:
  * [ ] Shift + cursor movement
  * [ ] Meta+A
* [ ] `TextInput`: Meta+X should cut the text
* [ ] support a concept of layers/z-order for dropdown boxes, tooltips etc.
* [ ] support Drag and drop
  * `druid-shell` has [no support](https://github.com/linebender/druid/issues/1742)
* [ ] implement [7GUIs](https://eugenkiss.github.io/7guis/tasks)
  * [ ] 7GUIs "Flight Booker"
    * [ ] implement `DropdownBox`
  * [ ] 7GUIs "Timer"
  * [ ] 7GUIs "CRUD"
  * [ ] 7GUIs "Circle Drawer"
  * [ ] 7GUIs "Cells"
* [ ] optimize: do not paint on every event. Make the widgets request the repaint in a region/Rect
* [ ] add Redmond 31 widgets?
* [ ] add widget garbage collection:
  * add a `Command::Destroy(WidgetId)`? 
  * implement `WidgetManager::collect_garbage()`?
    * remove all widgets that do not have the main widget as ancestor
* [ ] add Python bindings
* [ ] provide native widgets?
* [ ] provide a WebAssembly demo
* [ ] consider decoupling from druid-shell/piet 

See also ["So you want to write a GUI framework"](https://www.cmyr.net/blog/gui-framework-ingredients.html)
