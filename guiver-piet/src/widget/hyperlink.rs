use crate::font::Font;
use crate::shared_state::PietSharedState;
use crate::stroke::Stroke;
use crate::widget::{Text, WidgetError};
use crate::{Event, Piet, PietWidget};
use druid_shell::piet::{Error, PietText};
use druid_shell::Region;
use guiver::{
    Point, Rectangle, Size, SizeConstraints, Widget, WidgetEvent, WidgetEventType, WidgetId,
    WidgetIdProvider,
};
use std::borrow::BorrowMut;

///
pub struct Hyperlink<APP_EVENT: Clone> {
    is_being_clicked: bool,
    font_is_being_clicked: Font,
    font_normal: Font,
    font_was_visited: Font,
    text_widget: Text<APP_EVENT>,
    was_visited: bool,
}

///
fn adjust_font(font: &mut Font) {
    font.has_underline = true;
}

impl<APP_EVENT: Clone> Hyperlink<APP_EVENT> {
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
                .set_font(self.font_is_being_clicked.clone(), shared_state)
                .unwrap();
        }
        // The hyperlink is not being clicked anymore.
        else {
            // The hyperlink was visited.
            if self.was_visited {
                // Set the "was visited" font.
                self.text_widget
                    .borrow_mut()
                    .set_font(self.font_was_visited.clone(), shared_state)
                    .unwrap();
            } else {
                // Set the "normal" font.
                self.text_widget
                    .borrow_mut()
                    .set_font(self.font_normal.clone(), shared_state)
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
            .set_font(self.font_was_visited.clone(), shared_state);
    }
}

impl<APP_EVENT: Clone> Widget<APP_EVENT> for Hyperlink<APP_EVENT> {
    fn add_event_observation(
        &mut self,
        widget_event_type: WidgetEventType,
        widget_event: WidgetEvent<APP_EVENT>,
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
    ) -> Option<&WidgetEvent<APP_EVENT>> {
        self.text_widget.event_observation(widget_event_type)
    }

    fn rectangle(&self) -> &Rectangle {
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

    fn widget_id(&self) -> &WidgetId {
        self.text_widget.widget_id()
    }
}

impl<APP_EVENT: Clone> PietWidget<APP_EVENT> for Hyperlink<APP_EVENT> {
    fn handle_event(
        &mut self,
        event: &Event,
        shared_state: &mut PietSharedState,
        _widget_id_provider: &mut WidgetIdProvider,
        widget_events: &mut Vec<WidgetEvent<APP_EVENT>>,
    ) {
        match event {
            Event::MouseDown(mouse_event) => {
                // The click is outside of the text.
                if !self
                    .text_widget
                    .rectangle()
                    .contains(mouse_event.pos.x, mouse_event.pos.y)
                {
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
                if !self
                    .text_widget
                    .rectangle()
                    .contains(mouse_event.pos.x, mouse_event.pos.y)
                {
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

    fn set_stroke(&mut self, stroke: Option<Stroke>) -> Result<(), WidgetError> {
        self.text_widget.set_stroke(stroke)
    }
}
