use crate::widget::{Text, WidgetError, WidgetEventType};
use crate::{Event, Font, Piet, Size, SizeConstraints, Stroke, Widget, WidgetEvent, WidgetId};
use druid_shell::kurbo::{Point, Rect};
use druid_shell::piet::Error;
use druid_shell::Region;
use std::borrow::BorrowMut;

///
pub struct Hyperlink {
    is_being_clicked: bool,
    font_is_being_clicked: Font,
    font_normal: Font,
    font_was_visited: Font,
    text_widget: Text,
    was_visited: bool,
}

///
fn adjust_font(font: &mut Font) {
    font.has_underline = true;
}

impl Hyperlink {
    ///
    pub fn new(
        widget_id: WidgetId,
        debug_rendering_stroke: Stroke,
        mut font_unvisited: Font,
        mut font_being_clicked: Font,
        mut font_visited: Font,
        text: impl Into<String>,
    ) -> Self {
        adjust_font(&mut font_visited);
        adjust_font(&mut font_being_clicked);
        adjust_font(&mut font_unvisited);

        Hyperlink {
            is_being_clicked: false,
            font_is_being_clicked: font_being_clicked,
            font_normal: font_unvisited.clone(),
            font_was_visited: font_visited,
            text_widget: Text::new(widget_id, debug_rendering_stroke, font_unvisited, text),
            was_visited: false,
        }
    }

    ///
    fn set_is_being_clicked(&mut self, is_being_clicked: bool) {
        self.is_being_clicked = is_being_clicked;

        // The hyperlink is being clicked.
        if is_being_clicked {
            // Set the "is being clicked" font.
            self.text_widget
                .borrow_mut()
                .set_font(self.font_is_being_clicked.clone())
                .unwrap();
        }
        // The hyperlink is not being clicked anymore.
        else {
            // The hyperlink was visited.
            if self.was_visited {
                // Set the "was visited" font.
                self.text_widget
                    .borrow_mut()
                    .set_font(self.font_was_visited.clone())
                    .unwrap();
            } else {
                // Set the "normal" font.
                self.text_widget
                    .borrow_mut()
                    .set_font(self.font_normal.clone())
                    .unwrap();
            }
        }
    }

    ///
    fn set_was_visited(&mut self) {
        self.is_being_clicked = false;
        self.was_visited = true;

        // Set the "was visited" font.
        self.text_widget
            .borrow_mut()
            .set_font(self.font_was_visited.clone());
    }
}

impl Widget for Hyperlink {
    fn apply_size_constraints(&mut self, size_constraints: SizeConstraints) -> Size {
        self.text_widget.apply_size_constraints(size_constraints)
    }

    fn handle_event(&mut self, event: &Event, widget_events: &mut Vec<WidgetEvent>) {
        match event {
            Event::MouseDown(mouse_event) => {
                // The click is outside of the text.
                if !self.text_widget.rectangle().contains(mouse_event.pos) {
                    self.set_is_being_clicked(false);
                    return;
                }

                // The hyperlink is being clicked.
                self.set_is_being_clicked(true);

                widget_events.push((
                    self.text_widget.widget_id().clone(),
                    WidgetEventType::Clicked,
                ));
            }
            Event::MouseUp(mouse_event) => {
                // The click is outside of the text.
                if !self.text_widget.rectangle().contains(mouse_event.pos) {
                    // The hyperlink is not being clicked.
                    self.set_is_being_clicked(false);

                    return;
                }

                if self.is_being_clicked {
                    self.set_was_visited();
                } else {
                    // The hyperlink is not being clicked.
                    self.set_is_being_clicked(false);
                }
            }
            _ => {}
        }
    }

    fn paint(&self, piet: &mut Piet, region: &Region) -> Result<(), Error> {
        self.text_widget.paint(piet, region)
    }

    fn rectangle(&self) -> &Rect {
        self.text_widget.rectangle()
    }

    fn set_debug_rendering(&mut self, debug_rendering: bool) {
        self.text_widget.set_debug_rendering(debug_rendering);
    }

    fn set_is_disabled(&mut self, _is_disabled: bool) {
        // TODO
        println!("`Hyperlink::set_is_disabled()`: TODO");
    }

    fn set_is_hidden(&mut self, is_hidden: bool) {
        self.text_widget.set_is_hidden(is_hidden);
    }

    fn set_origin(&mut self, origin: Point) {
        self.text_widget.set_origin(origin)
    }

    fn set_stroke(&mut self, stroke: Option<Stroke>) -> Result<(), WidgetError> {
        self.text_widget.set_stroke(stroke)
    }

    fn widget_id(&self) -> &WidgetId {
        self.text_widget.widget_id()
    }
}
