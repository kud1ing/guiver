#[cfg(test)]
mod tests {

    use druid_shell::kurbo::Size;
    use guiver::widget::{Button, Placeholder, Text, TextInput, WidgetCommand};
    use guiver::{widget, Color, Font, SizeConstraints, Stroke, Widget, WidgetId};

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
        let mut button = Button::default();
        button
            .handle_command(WidgetCommand::SetValue(Box::new("Button".to_string())))
            .unwrap();

        vec![
            // TODO: add a button
            // Box::new(button),
            Box::new(Placeholder::new(0, Size::new(100.0, 50.0))),
            // TODO: add a center
            // TODO: add a column
            // TODO: add a expanded?
            // TODO: add a padding
            // TODO: add a row
            // TODO: add a text
            /*
            Box::new(Text::new(
                0,
                Stroke::default(),
                Font::default(),
                "Text".to_string(),
            )),
            */
            // TODO: add a text input
            /*
            Box::new(TextInput::new(
                0,
                Stroke::default(),
                Font::default(),
                "Text Input".to_string(),
                100.0,
                Color::rgb8(0, 0, 0),
                Color::rgb8(255, 255, 255),
            )),
            */
        ]
    }
}
