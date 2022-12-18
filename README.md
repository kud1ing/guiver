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

<!-- center -->
<img width="313" alt="Bildschirm­foto 2022-12-16 um 09 19 34" src="https://user-images.githubusercontent.com/391975/208054566-4774326f-9953-4537-b08b-ad82f59c2ab8.png">

<!-- counter -->
<img width="193" alt="Bildschirm­foto 2022-12-16 um 09 21 12" src="https://user-images.githubusercontent.com/391975/208054852-b143b584-ec87-4bc7-a7bd-4c8fa38658ae.png">

<!-- temperature converter-->
<img width="303" alt="Bildschirm­foto 2022-12-16 um 09 26 50" src="https://user-images.githubusercontent.com/391975/208056001-2335f536-0a39-4a07-8f8e-436a59a81e26.png">

<!-- placeholder -->
<img width="183" alt="Bildschirm­foto 2022-12-16 um 09 22 23" src="https://user-images.githubusercontent.com/391975/208055118-18cc28fa-bf42-4980-acb9-713cc94c9697.png">

<!-- padding -->
<img width="195" alt="Bildschirm­foto 2022-12-16 um 09 25 50" src="https://user-images.githubusercontent.com/391975/208055749-705cc320-0520-47a8-b82c-1136d29bb8aa.png">

<!-- row -->
<img width="346" alt="Bildschirm­foto 2022-12-16 um 09 27 34" src="https://user-images.githubusercontent.com/391975/208056135-a3767c96-ec39-4309-8e6c-04ba81865b78.png">

<!-- column -->
<img width="457" alt="Bildschirm­foto 2022-12-16 um 09 28 14" src="https://user-images.githubusercontent.com/391975/208056264-7011bb0f-1b89-4eb0-873b-103ebfc16f55.png">

<!-- grid -->
<img width="278" alt="Bildschirm­foto 2022-12-16 um 09 18 37" src="https://user-images.githubusercontent.com/391975/208054426-cf546c7b-1f20-4ff3-a8d6-e1aff292a8d9.png">

<!-- freundchen -->
<img width="912" alt="Bildschirmfoto 2022-10-11 um 14 53 50" src="https://user-images.githubusercontent.com/391975/195096506-0ec82534-bd92-4879-8174-2e803776fa64.png">

<!-- makamau -->
<img width="788" alt="Bildschirmfoto 2022-10-11 um 20 51 41" src="https://user-images.githubusercontent.com/391975/195175173-8e016f19-7011-458a-bfc3-b2425ed93e22.png">

<!-- abermals -->
<img width="888" alt="Bildschirmfoto 2022-07-05 um 19 43 55" src="https://user-images.githubusercontent.com/391975/177385769-598d0fd0-c15b-4d7e-bb98-5fd46bd9d415.png">

## Status

* Operating systems:
  * macOS: tested intensively
  * Windows: works in general, probably has rough edges
  * other: testers are welcome
* Widget manager:
  * handles widget focussing, including tab/focus order
  * handle widget lifetimes
  * handles copy/paste
* Widgets:
  * Button
  * Hyperlink
  * Placeholder
  * Text
  * Text input
* Layout widgets:
  * Center
  * Column
  * Expanded
  * Grid
  * Padding
  * Row
  * SizedBox


## Backlog

* `TextInput` caret:
  * [ ] try to understand how https://github.com/linebender/druid/blob/master/druid/src/widget/textbox.rs does it
  * [ ] add a hash map from caret character indices to x positions
    * [ ] update it when the text is changed
    * [ ] use it in `paint()` to position the caret
  * [ ] `TextInput::handle_event()`: increase/decrease `self.caret_character_index` on arrow left/right
  * [ ] `TextInput::update_caret_character_index()`: implement
* [ ] `TextInput`: if a text is too large to fit in, the size of the text input should not increase but truncate
* [ ] `TextInput`: arrow keys should move the caret
* [ ] `TextInput`: Shift + arrow keys should de/select text
* [ ] `TextInput`: double click should select the text
* [ ] `TextInput`: Meta+X should cut the text
* [ ] `test::widgets_layout()`: add remaining layout widgets
* [ ] `test::widgets()`: add child widgets to the layout widgets
* [ ] use `WidgetId`s that are independent of a `WidgetManager`
  * Discussion:
    * Chances:
      * widget creation and composition can happen at the same time with the same means (`Command`)
        * GUI construction can deserialized from a serialization format 
    * Risks:
  * Options:
    * Positional ID (egui: https://docs.rs/egui/latest/egui/struct.Id.html) (8 bytes)
    * `usize`? (8 bytes)
    * UUID? (16 bytes)
    * String? (>= 24 bytes)
* [ ] `Hyperlink`: cache the two main `TextLayout`s to speed up mouse down/up
* [ ] move `WidgetManager::focused_widget` to `WidgetFocusOrder`?
* [ ] publish 0.1.1, once the updated `druid-shell` >0.7.0 is [released](https://github.com/linebender/druid/issues/2236)
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
* [ ] `Hyperlink`: make it possible to remember/pass "was visited" status across widget lifetimes
* support text selection:
  * [ ] Shift + cursor movement
  * [ ] Meta+A
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
