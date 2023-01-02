#[cfg(test)]
mod tests {
    use druid_shell::kurbo::Size;
    use guiver::stroke::Stroke;
    use guiver::{Color, Font, HorizontalAlignment, SizeConstraints, VerticalAlignment};
    use guiver_piet::piet_text;
    use guiver_piet::widget::layout::{Center, Column, Padding, Row, SizedBox};
    use guiver_piet::widget::{Button, Hyperlink, PietWidget, Placeholder, Text, TextInput};
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn test_apply_size_constraints() {
        // Iterate over the widgets.
        for mut widget in widgets::<()>() {
            // Apply `SizeConstraints` that have a large minimum and an unbounded maximum.
            {
                let large_minimum_size = Size::new(1000.0, 800.0);
                widget.apply_size_constraints(SizeConstraints::new(
                    large_minimum_size,
                    Size::new(f64::INFINITY, f64::INFINITY),
                ));

                assert_eq!(
                    widget.rectangle().size(),
                    large_minimum_size,
                    "The widget should be at least as large as the given minimum size"
                );
            }

            // Apply `SizeConstraints` that have a small maximum and an unbounded minimum.
            {
                let small_maximum_size = Size::new(10.0, 10.0);
                widget.apply_size_constraints(SizeConstraints::loose(small_maximum_size));

                assert_eq!(
                    widget.rectangle().size(),
                    small_maximum_size,
                    "The widget should not be larger than the given maximum size"
                );
            }
        }
    }

    /// Returns all widgets.
    fn widgets<T: Clone + 'static>() -> Vec<Box<dyn PietWidget<T>>> {
        // TODO: add child widgets to the layout widgets.
        let mut widgets = widgets_layout();
        widgets.append(&mut widgets_non_layout());
        widgets
    }

    /// Returns the layout widgets.
    fn widgets_layout<T: Clone + 'static>() -> Vec<Box<dyn PietWidget<T>>> {
        let mut center_widget: Center<T> = Center::new(0, Stroke::default());
        center_widget
            .add_child(
                None,
                Rc::new(RefCell::new(Box::new(Placeholder::new(
                    1,
                    Stroke::default(),
                    Size::new(100.0, 50.0),
                )))),
            )
            .unwrap();

        let mut column_widget: Column<T> =
            Column::new(2, Stroke::default(), HorizontalAlignment::Center, 10.0);
        column_widget
            .add_child(
                None,
                Rc::new(RefCell::new(Box::new(Placeholder::new(
                    3,
                    Stroke::default(),
                    Size::new(100.0, 50.0),
                )))),
            )
            .unwrap();

        let mut padding_widget: Padding<T> =
            Padding::new(4, Stroke::default(), 10.0, 10.0, 10.0, 10.0);
        padding_widget
            .add_child(
                None,
                Rc::new(RefCell::new(Box::new(Placeholder::new(
                    5,
                    Stroke::default(),
                    Size::new(100.0, 50.0),
                )))),
            )
            .unwrap();

        let mut row_widget: Row<T> =
            Row::new(6, Stroke::default(), VerticalAlignment::Middle, 10.0);
        row_widget
            .add_child(
                None,
                Rc::new(RefCell::new(Box::new(Placeholder::new(
                    7,
                    Stroke::default(),
                    Size::new(100.0, 50.0),
                )))),
            )
            .unwrap();

        vec![
            // Add a center widget.
            Box::new(center_widget),
            // TODO: Add a column widget.
            //Box::new(column_widget),
            // TODO: Add a expanded widget?
            // TODO: Add a grid widget.
            //Box::new(Grid::new(0, Stroke::default(), 10.0)),
            // TODO: Add a padding widget.
            //Box::new(padding_widget),
            // TODO: Add a row widget.
            //Box::new(row_widget),
            // Add a sized box widget.
            Box::new(SizedBox::new(8, Stroke::default(), Size::new(100.0, 50.0))),
        ]
    }

    /// Returns the non-layout widgets.
    fn widgets_non_layout<T: Clone + 'static>() -> Vec<Box<dyn PietWidget<T>>> {
        let mut piet_text = piet_text();

        vec![
            // Add a text button widget.
            Box::new(Button::new(
                100,
                Stroke::default(),
                Rc::new(RefCell::new(Box::new(Text::new(
                    101,
                    Stroke::default(),
                    &mut piet_text,
                    Font::default(),
                    "Button text".to_string(),
                )))),
                None,
                None,
                None,
            )),
            // Add a placeholder widget.
            Box::new(Placeholder::new(
                102,
                Stroke::default(),
                Size::new(100.0, 50.0),
            )),
            // Add a hyperlink widget.
            Box::new(Hyperlink::new(
                103,
                Stroke::default(),
                &mut piet_text,
                Font::default(),
                Font::default(),
                Font::default(),
                "Test hyperlink".to_string(),
            )),
            // Add a text widget.
            Box::new(Text::new(
                104,
                Stroke::default(),
                &mut piet_text,
                Font::default(),
                "Test text".to_string(),
            )),
            // Add a text input widget.
            Box::new(TextInput::new(
                105,
                Stroke::default(),
                &mut piet_text,
                Font::default(),
                "Text input".to_string(),
                100.0,
                Color::rgb8(0, 0, 0),
                Color::rgb8(255, 255, 255),
            )),
        ]
    }
}
