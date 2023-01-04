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

The [example code](guiver-examples/examples/) may appear a bit verbose.
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
<img width="140" alt="Bildschirm­foto 2022-12-20 um 08 08 19" src="https://user-images.githubusercontent.com/391975/208604813-3b5c2809-0622-436f-b742-39c8db102b9e.png">

<!-- grid -->
<img width="272" alt="Bildschirm­foto 2022-12-20 um 08 13 10" src="https://user-images.githubusercontent.com/391975/208605573-1a5d9171-51c9-419e-816e-53f9f6fc7a7b.png">

<!-- freundchen -->
<img width="912" alt="Bildschirm­foto 2022-12-20 um 08 16 51" src="https://user-images.githubusercontent.com/391975/208606239-c548f2da-d4a7-4ca5-a49f-c7e81ea740bf.png">

<!-- makamau 1 -->
<img width="677" alt="Bildschirm­foto 2022-12-20 um 08 21 59" src="https://user-images.githubusercontent.com/391975/208607090-5bd4d792-f75d-47bb-a0ac-ea5444d1da27.png">

<!-- makamau 2 -->
<img width="571" alt="Bildschirm­foto 2022-12-20 um 08 19 20" src="https://user-images.githubusercontent.com/391975/208606700-6091eb6b-18c3-4655-83c9-93e292dda556.png">

<!-- abermals -->
<img width="888" alt="Bildschirmfoto 2022-07-05 um 19 43 55" src="https://user-images.githubusercontent.com/391975/177385769-598d0fd0-c15b-4d7e-bb98-5fd46bd9d415.png">


## Status

* Operating systems:
  * macOS: tested intensively
  * Windows: works in general, probably has rough edges
  * other: testers are welcome
* Widget manager:
  * handles widget lifetimes
  * handles a main widget
  * handles widget focussing – including tab order
  * handles copy/paste
* Widgets:
  * Button
  * Hyperlink
  * Placeholder
  * Text
  * TextInput
* Layout widgets:
  * Center
  * Column
  * Expanded
  * Grid
  * Padding
  * Row
  * SizedBox


## Developer guide

Widgets are decoupled from the developers code via the `WidgetManager`. It owns the widgets and manages their lifetimes.
Widgets are created via `new_*()` methods and are modified and composed via `Command`s.
Commands allow transactional modification, where a re-layout happens once at the end.

Widgets implement the `Widget` trait. The methods can be used by the `WidgetManager` and by other widgets (e.g. the
`TextInput` widget contains a `Text` widget).
A widget reacts to user `Event`s and possibly creates `WidgetEvent`s accordingly.
The developer code can handle those widget events.



## Backlog

* [ ] `guiver-piet`: bundle the `Command`s per `WidgetId`: add `Command::ToWidget<Vec<WidgetCommand>>`
  * [ ] add `WidgetCommand`

* `guiver-piet`: unify widgets:
  * [ ] add `fn core(&self) -> &WidgetCore` to `PietWidget` or `Widget`?
  * [ ] add a method to return an iterator over th child widgets
  * [ ] default implement `PietWidget::paint()` using `core()` and the child widgets iterator yielding method

* [ ] `guiver-piet`: add support to `PietWidget` for the concept of layers/z-order for dropdown boxes, tooltips etc.
  * [ ] how?
    * Druid: [`PaintCtx::paint_with_z_index()`](https://docs.rs/druid/latest/src/druid/contexts.rs.html#735-746)

* add selectors:
  * [ ] add `Command::SetClass(Option<C>)`
  * [ ] add `WidgetSelector`:
    * `All`
    * `WithId(WidgetId)`
    * `ChildrenOf(WidgetId)`
    * `ParentOf(WidgetId)`
    * [ ] use it in
      * [ ] `Command::Destroy`
      * [ ] `Command::RemoveChild`
        * [ ] remove `Command::RemoveChildren`
      * [ ] `Command::RemoveEventObservation`
      * [ ] `Command::AddEventObservation`
      * [ ] `Command::SetDebugRendering`
      * [ ] `Command::SetFill`
      * [ ] `Command::SetFont`
      * [ ] `Command::SetHorizontalAlignment`
      * [ ] `Command::SetIsDisabled`
      * [ ] `Command::SetIsHidden`
      * [ ] `Command::SetStroke`
      * [ ] `Command::SetValue`
      * [ ] `Command::SetVerticalAlignment`
* [ ] add `Widget::class() -> Option<C>`
  * [ ] implement in `Core`
  * [ ] add `WidgetSelector::WithClass(C)`

* [ ] sketch a WASM backend?
* [ ] sketch a `cacao` backend?
* [ ] sketch a `egui` backend?

* `guiver-piet`: text:
  * `test_selected_text_replaced()`:
    * [ ] fix
    * [ ] add a test case using umlauts
  * [ ] `test_selected_text()`: add a test case using umlauts
  * [ ] `test_text_inserted()`: implement
    *  add a test case using umlauts
  * [ ] `TextInput::set_selected_value()`: implement
  * determine the graphical positions:
    * [TextLayout::hit_test_text_position(())](https://docs.rs/druid/latest/druid/piet/trait.TextLayout.html#tymethod.hit_test_text_position)
      * [HitTestPosition](https://docs.rs/druid/latest/druid/piet/struct.HitTestPosition.html)
    * [ ] text cursor
    * [ ] text selection
  * [ ] `paint()`: paint the cursor
  * [ ] `paint()`: paint the text selection
  * [ ] `TextInput::handle_event()`: adjust the text cursor on arrow left/right
  * [ ] `TextInput::handle_event()`: adjust the text selection on Shift + arrow left/right
  * [ ] `TextInput::update_caret_character_index()`: implement
  * [ ] `TextInput`: Meta+X should cut the selected text
  * [ ] `TextInput`: if a text is too large to fit in, the size of the text input should not increase but truncate
  * [ ] `TextInput::handle_event()`: select the whole text on double click

* [ ] `guiver-piet`: add support for scrolling
  * [ ] how?

* [ ] `test::widgets_layout()`: add remaining layout widgets
* [ ] `test::widgets()`: add child widgets to the layout widgets

* [ ] move `WidgetManager::focused_widget` to `WidgetFocusOrder`?
* [ ] publish 0.1.1, once the updated `druid-shell` >0.7.0 is [released](https://github.com/linebender)
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
* Apache License, Version 2.0 ([LICENSE-APACHE](guiver-piet/LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](guiver-piet/LICENSE-MIT) or http://opensource.org/licenses/MIT)
  at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you shall be dual licensed as above, without any
additional terms or conditions.
