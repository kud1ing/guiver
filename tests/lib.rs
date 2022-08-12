#[cfg(test)]
mod tests {
    use druid_shell::kurbo::Size;
    use guiver::widget::layout::{Center, SizedBox};
    use guiver::widget::{Button, Placeholder, Text, TextInput, WidgetCommand};
    use guiver::{Color, Font, SizeConstraints, Stroke, Widget};
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn test_apply_size_constraints() {
        // Iterate over the widgets.
        for mut widget in widgets() {
            // Apply a `SizeConstraints` that has a large minimum and the maximum is unbounded.
            {
                let large_minimum_size = Size::new(1000.0, 800.0);
                widget.apply_size_constraints(SizeConstraints::new(
                    large_minimum_size,
                    Size::new(f64::INFINITY, f64::INFINITY),
                ));

                assert_eq!(
                    widget.size(),
                    large_minimum_size,
                    "The widget should be at least as large as the given minimum size"
                );
            }

            // Apply a `SizeConstraints` that has a small maximum and the minimum is unbounded.
            {
                let small_maximum_size = Size::new(10.0, 10.0);
                widget.apply_size_constraints(SizeConstraints::loose(small_maximum_size));

                assert_eq!(
                    widget.size(),
                    small_maximum_size,
                    "The widget should not be larger than the given maximum size"
                );
            }
        }
    }

    ///
    fn widgets() -> Vec<Box<dyn Widget>> {
        let button_with_text = Button::new(
            0,
            Stroke::default(),
            Rc::new(RefCell::new(Box::new(Text::new(
                1,
                Stroke::default(),
                Font::default(),
                "Button text".to_string(),
            )))),
            None,
            None,
            None,
        );

        let mut center_widget = Center::new(0, Stroke::default());
        center_widget
            .handle_command(WidgetCommand::AppendChild(Rc::new(RefCell::new(Box::new(
                Placeholder::new(0, Size::new(100.0, 50.0)),
            )))))
            .unwrap();

        let mut sized_box_widget_without_a_child =
            SizedBox::new(0, Stroke::default(), Size::new(100.0, 50.0));

        vec![
            // Add a text button widget.
            Box::new(button_with_text),
            // Add a placeholder widget.
            Box::new(Placeholder::new(0, Size::new(100.0, 50.0))),
            // Add a center widget.
            Box::new(center_widget),
            // TODO: Add a column widget.
            // TODO: Add a expanded widget?
            // TODO: Add a padding widget.
            // TODO: Add a row widget.
            // Add a sized box widget without a child widget.
            Box::new(sized_box_widget_without_a_child),
            // TODO: Add a sized box widget with a child widget.
            // Add a text widget.
            Box::new(Text::new(
                0,
                Stroke::default(),
                Font::default(),
                "Test text".to_string(),
            )),
            // Add a text input widget.
            Box::new(TextInput::new(
                0,
                Stroke::default(),
                Font::default(),
                "Text Input".to_string(),
                100.0,
                Color::rgb8(0, 0, 0),
                Color::rgb8(255, 255, 255),
            )),
        ]
    }
}
