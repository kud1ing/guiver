use crate::shared_state::PietSharedState;
use crate::widget::{Text, WidgetError};
use crate::{Event, Piet, PietWidget};
use druid_shell::kurbo::{Point, Rect};
use druid_shell::piet::{Error, PietText};
use druid_shell::Region;
use guiver::stroke::Stroke;
use guiver::{
    Font, Size, SizeConstraints, Widget, WidgetEvent, WidgetEventType, WidgetId, WidgetIdProvider,
};
use std::borrow::BorrowMut;

///
pub struct Hyperlink<T: Clone> {
    is_being_clicked: bool,
    font_is_being_clicked: Font,
    font_normal: Font,
    font_was_visited: Font,
    text_widget: Text<T>,
    was_visited: bool,
}

///
fn adjust_font(font: &mut Font) {
    font.has_underline = true;
}

impl<T: Clone> Hyperlink<T> {
    ///
    pub fn new(
        widget_id: WidgetId,
        debug_rendering_stroke: Stroke,
        piet_text: &mut PietText,
        mut font_unvisited: Font,
        mut font_being_clicked: Font,
        mut font_visited: Font,
        text: String,
    ) -> Self {
        adjust_font(&mut font_visited);
        adjust_font(&mut font_being_clicked);
        adjust_font(&mut font_unvisited);

        Hyperlink {
            is_being_clicked: false,
            font_is_being_clicked: font_being_clicked,
            font_normal: font_unvisited.clone(),
            font_was_visited: font_visited,
            text_widget: Text::new(
                widget_id,
                debug_rendering_stroke,
                piet_text,
                font_unvisited,
                text,
            ),
            was_visited: false,
        }
    }

    ///
    fn set_is_being_clicked(&mut self, shared_state: &mut PietSharedState, is_being_clicked: bool) {
        self.is_being_clicked = is_being_clicked;

        // The hyperlink is being clicked.
        if is_being_clicked {
            // Set the "is being clicked" font.
            self.text_widget
                .borrow_mut()
                .set_font(shared_state, self.font_is_being_clicked.clone())
                .unwrap();
        }
        // The hyperlink is not being clicked anymore.
        else {
            // The hyperlink was visited.
            if self.was_visited {
                // Set the "was visited" font.
                self.text_widget
                    .borrow_mut()
                    .set_font(shared_state, self.font_was_visited.clone())
                    .unwrap();
            } else {
                // Set the "normal" font.
                self.text_widget
                    .borrow_mut()
                    .set_font(shared_state, self.font_normal.clone())
                    .unwrap();
            }
        }
    }

    ///
    fn set_was_visited(&mut self, shared_state: &mut PietSharedState) {
        self.is_being_clicked = false;
        self.was_visited = true;

        // Set the "was visited" font.
        self.text_widget
            .borrow_mut()
            .set_font(shared_state, self.font_was_visited.clone());
    }
}

impl<T: Clone> Widget<T> for Hyperlink<T> {
    fn add_event_observation(
        &mut self,
        widget_event_type: WidgetEventType,
        widget_event: WidgetEvent<T>,
    ) {
        self.text_widget
            .add_event_observation(widget_event_type, widget_event);
    }

    fn apply_size_constraints(&mut self, size_constraints: SizeConstraints) -> Size {
        self.text_widget.apply_size_constraints(size_constraints)
    }

    fn event_observation(
        &mut self,
        widget_event_type: &WidgetEventType,
    ) -> Option<&WidgetEvent<T>> {
        self.text_widget.event_observation(widget_event_type)
    }

    fn rectangle(&self) -> &Rect {
        self.text_widget.rectangle()
    }

    fn remove_event_observation(&mut self, widget_event_type: &WidgetEventType) {
        self.text_widget.remove_event_observation(widget_event_type);
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

impl<T: Clone> PietWidget<T> for Hyperlink<T> {
    fn handle_event(
        &mut self,
        _widget_id_provider: &mut WidgetIdProvider,
        shared_state: &mut PietSharedState,
        event: &Event,
        widget_events: &mut Vec<WidgetEvent<T>>,
    ) {
        match event {
            Event::MouseDown(mouse_event) => {
                // The click is outside of the text.
                if !self.text_widget.rectangle().contains(mouse_event.pos) {
                    self.set_is_being_clicked(shared_state, false);
                    return;
                }

                // The hyperlink is being clicked.
                self.set_is_being_clicked(shared_state, true);

                // There is a widget event observation.
                if let Some(widget_event) = self
                    .text_widget
                    .event_observation(&WidgetEventType::Clicked)
                {
                    widget_events.push(widget_event.clone());
                }
            }
            Event::MouseUp(mouse_event) => {
                // The click is outside of the text.
                if !self.text_widget.rectangle().contains(mouse_event.pos) {
                    // The hyperlink is not being clicked.
                    self.set_is_being_clicked(shared_state, false);
                    return;
                }

                if self.is_being_clicked {
                    self.set_was_visited(shared_state);

                    // There is a widget event observation.
                    if let Some(widget_event) = self
                        .text_widget
                        .event_observation(&WidgetEventType::Submitted)
                    {
                        widget_events.push(widget_event.clone());
                    }
                } else {
                    // The hyperlink is not being clicked.
                    self.set_is_being_clicked(shared_state, false);
                }
            }
            _ => {}
        }
    }

    fn paint(&self, piet: &mut Piet, region: &Region) -> Result<(), Error> {
        self.text_widget.paint(piet, region)
    }
}
