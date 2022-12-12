# guiver

guiver tries to make GUI programming with Rust simple, but perhaps without perfect efficiency.

guiver can be used in [immediate mode](https://en.wikipedia.org/wiki/Immediate_mode_(computer_graphics)), which is
inspired by [egui](https://github.com/emilk/egui). In this case it is
just a thin wrapper around [druid-shell](https://github.com/linebender/druid/tree/master/druid-shell) for event
handling/windowing and [Piet](https://github.com/linebender/piet) for rendering.

There are [retained mode](https://en.wikipedia.org/wiki/Retained_mode) widgets that can be used with the help of an
optional widget manager.
The widgets are decoupled from the application data via message passing. This is inspired by
[Tk commands](https://www.tcl.tk/man/tcl/TkCmd/contents.html) and egui.
The widget size constraint system and some layout widgets are influenced by [Flutter](https://flutter.dev).

The [example code](examples/) may appear a bit verbose.
On the upside you get simple setup and simple control flow.

<img width="912" alt="Bildschirmfoto 2022-10-11 um 14 53 50" src="https://user-images.githubusercontent.com/391975/195096506-0ec82534-bd92-4879-8174-2e803776fa64.png">

<img width="788" alt="Bildschirmfoto 2022-10-11 um 20 51 41" src="https://user-images.githubusercontent.com/391975/195175173-8e016f19-7011-458a-bfc3-b2425ed93e22.png">

<img width="888" alt="Bildschirmfoto 2022-07-05 um 19 43 55" src="https://user-images.githubusercontent.com/391975/177385769-598d0fd0-c15b-4d7e-bb98-5fd46bd9d415.png">

<img width="232" alt="Bildschirmfoto 2022-07-29 um 21 09 16" src="https://user-images.githubusercontent.com/391975/181828207-8c268397-f815-484a-a28f-22501fc04ec4.png">

<img width="400" alt="Bildschirmfoto 2022-10-11 um 20 49 10" src="https://user-images.githubusercontent.com/391975/195174762-3daacb5a-5189-4961-8c45-b3746f04bb4d.png">

<img width="293" alt="Bildschirmfoto 2022-07-29 um 21 07 20" src="https://user-images.githubusercontent.com/391975/181827939-ba986efe-b0b7-4a2a-ba09-9eedff8c7846.png">

<img width="357" alt="Bildschirmfoto 2022-07-29 um 21 06 35" src="https://user-images.githubusercontent.com/391975/181827857-42b6ffb0-9837-4bfa-9552-e4e68f68d639.png">

<img width="399" alt="Bildschirmfoto 2022-07-29 um 21 05 50" src="https://user-images.githubusercontent.com/391975/181827755-3e8e9e82-155c-4c0e-aa37-4963f1db49b0.png">

<img width="631" alt="Bildschirmfoto 2022-07-29 um 21 04 20" src="https://user-images.githubusercontent.com/391975/181827655-c8945f84-be44-47ab-836c-64a1c41bc945.png">

<img width="315" alt="Bildschirmfoto 2022-07-25 um 21 57 15" src="https://user-images.githubusercontent.com/391975/180863911-98ca0572-d700-426a-be9b-3f96c708f478.png">


## Backlog

* Lifecycle management:
  * [ ] add to `Widget`:
    * `destroy_child(widget_id: WidgetId, parent_widget_ids: &mut HashSet<WidgetId>)`
    * `destroy_all_children(parent_widget_ids: &mut HashSet<WidgetId>)`
  * [ ] add to `Command`:
    * `DESTROY_CHILD` or `REMOVE_CHILD` with a parameter `destroy`?
    * `DESTROY_ALL_CHILDREN` or `REMOVE_ALL_CHILDREN` with a parameter `destroy`?
    * `DESTROY_WIDGET`
* `Grid`:
  * [ ] implement `Grid::layout_child_widgets()`
  * [ ] try `examples/layout_grid.rs` to see how `Grid` looks in action
* [ ] `Hyperlink`: cache the two main `TextLayout`s to speed up mouse down/up
* [ ] use `WidgetId`s that are independent of a `WidgetManager` for ease of use
  * UUID? Positional Memoization(Egui?)? https://docs.rs/egui/latest/egui/struct.Id.html
* [ ] publish 0.1.1, once the updated `druid-shell` >0.7.0 is [released](https://github.com/linebender/druid/issues/2236)
* [ ] `test::widgets_layout()`: add remaining layout widgets
* [ ] `test::widgets()`: add child widgets to the layout widgets
* `TextInput` caret:
  * [ ] try to understand how https://github.com/linebender/druid/blob/master/druid/src/widget/textbox.rs does it 
  * [ ] add a hash map from caret character indices to x positions
    * [ ] update it when the text is changed
    * [ ] use it in `paint()` to position the caret
  * [ ] `TextInput::handle_event()`: increase/decrease `self.caret_character_index` on arrow left/right
  * [ ] `TextInput::update_caret_character_index()`: implement
* `WidgetManager`: implement tab order `tab_order: Vec<WidgetId>`:
  * [ ] build tab order, equal to the order of creation
    * [ ] widgets need to tell the widget manager somehow, that they accept focus
      * `accepts_focus()` (`Button`, `TextInput`)
  * [ ] give the next widget in the tab order the focus when tab key is pressed
* [ ] make `Text` selectable:
  * [ ] via double click
    * [ ] how?
  * [ ] via click + drag
* [ ] add `WidgetCore::is_disabled`?
  * makes sense for non-layout widgets:
    * `Button`
    * `Hyperlink`
    * `Text`
    * `TextInput`
  * [ ] does it make sense for layout widgets?
    * [ ] should they pass the command down to its child widgets?
* [ ] `Widget`: remove `flex_factor()`?
  * Pro:
    * for `Grid` it needs to be held externally (`GridColumnProperties`, `GridRowProperties`)
  * Cons:
    * does it make usage of `Column`, `Row` less pleasant?
* [ ] should all container widgets clip the child widget's painting?
  * Pro:
    * restricts misbehaving widgets paint
  * Con:
    * performance impact if clipping is unnecessary?
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
* [ ] all layout widgets need to clip too big child widgets
* [ ] add layout widget `Stacked` + `Positioned`
* [ ] `TextInput`: if a text is too large to fit in, the size of the text input should not increase but truncate
* [ ] `TextInput`: arrow keys should move the caret
* [ ] `TextInput`: Shift + arrow keys should de/select text
* [ ] `TextInput`: double click should select the text
* [ ] `Hyperlink`: make it possible to remember/pass "was visited" status across widget lifetimes
* support text selection:
  * [ ] Shift + cursor movement
  * [ ] Meta+A
* [ ] `TextInput`: Meta+X should cut the text
* [ ] support a concept of layers/z-order for dropdown boxes, tooltips etc.
  * [ ] how?
* [ ] support Drag and drop
  * `druid-shell` has [no support](https://github.com/linebender/druid/issues/1742)
* [ ] implement [7GUIs](https://eugenkiss.github.io/7guis/tasks)
  * [ ] 7GUIs "Flight Booker"
    * [ ] implement `DropdownBox` (needs 2 layers)
  * [ ] 7GUIs "Timer"
  * [ ] 7GUIs "CRUD"
  * [ ] 7GUIs "Circle Drawer"
  * [ ] 7GUIs "Cells"
* [ ] optimize: do not paint on every event. Make the widgets request the repaint in a region/Rect
* [ ] add Redmond 31 widgets?
* [ ] add Python bindings?
* [ ] provide wrappers to native widgets?
* [ ] provide a WebAssembly demo
* [ ] consider decoupling from druid-shell/piet 

See also ["So you want to write a GUI framework"](https://www.cmyr.net/blog/gui-framework-ingredients.html)


## License

Licensed under either of
* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
  at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you shall be dual licensed as above, without any
additional terms or conditions.
