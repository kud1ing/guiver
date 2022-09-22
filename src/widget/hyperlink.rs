use crate::widget::core::WidgetCore;
use crate::widget::{Text, WidgetCommand, WidgetError};
use crate::{Event, Font, Piet, Size, SizeConstraints, Stroke, Widget, WidgetEvent, WidgetId};
use druid_shell::kurbo::{Point, Rect};
use druid_shell::piet::Error;
use druid_shell::Region;

///
pub struct Hyperlink {
    core: WidgetCore,
    is_being_clicked: bool,
    font_being_clicked: Font,
    font_unvisited: Font,
    font_visited: Font,
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
            core: WidgetCore::new(widget_id, debug_rendering_stroke.clone()),
            is_being_clicked: false,
            font_being_clicked,
            font_unvisited: font_unvisited.clone(),
            font_visited,
            text_widget: Text::new(widget_id, debug_rendering_stroke, font_unvisited, text),
            was_visited: false,
        }
    }
}

impl Widget for Hyperlink {
    fn apply_size_constraints(&mut self, size_constraints: SizeConstraints) -> Size {
        self.text_widget.apply_size_constraints(size_constraints)
    }

    fn handle_command(&mut self, widget_command: &WidgetCommand) -> Result<(), WidgetError> {
        match widget_command {
            WidgetCommand::SetDebugRendering(_) => {
                return self.text_widget.handle_command(widget_command)
            }
            WidgetCommand::SetFill(_) => return self.text_widget.handle_command(widget_command),
            WidgetCommand::SetFont(_) => {
                // TODO
                /*
                adjust_font(&mut font_visited);
                adjust_font(&mut font_being_clicked);
                adjust_font(&mut font_unvisited);
                */

                return self.text_widget.handle_command(widget_command);
            }
            WidgetCommand::SetHorizontalAlignment(_) => {
                return self.text_widget.handle_command(widget_command)
            }
            WidgetCommand::SetIsDisabled(_) => {
                return self.text_widget.handle_command(widget_command)
            }
            WidgetCommand::SetIsHidden(_) => {
                return self.text_widget.handle_command(widget_command)
            }
            WidgetCommand::SetStroke(_) => return self.text_widget.handle_command(widget_command),
            WidgetCommand::SetValue(_) => return self.text_widget.handle_command(widget_command),
            WidgetCommand::SetVerticalAlignment(_) => {
                return self.text_widget.handle_command(widget_command)
            }
            _ => {}
        };

        self.core.handle_command(widget_command)
    }

    fn handle_event(&mut self, event: &Event, widget_events: &mut Vec<WidgetEvent>) {
        match event {
            Event::ClipboardPaste(_) => {}
            Event::KeyDown(_) => {}
            Event::KeyUp(_) => {}
            Event::MouseDown(mouse_event) => {
                // The click is outside of the text.
                if !self.text_widget.rectangle().contains(mouse_event.pos) {
                    self.is_being_clicked = false;
                    return;
                }

                self.is_being_clicked = true;

                widget_events.push(WidgetEvent::Clicked(*self.text_widget.widget_id()));
            }
            Event::MouseMove(_) => {}
            Event::MouseUp(mouse_event) => {
                // The click is outside of the text.
                if !self.text_widget.rectangle().contains(mouse_event.pos) {
                    // TODO:
                    return;
                }

                if self.is_being_clicked {
                    self.is_being_clicked = false;
                    self.was_visited = true;
                }
            }
        }
    }

    fn paint(&self, piet: &mut Piet, region: &Region) -> Result<(), Error> {
        self.text_widget.paint(piet, region)
    }

    fn rectangle(&self) -> &Rect {
        self.text_widget.rectangle()
    }

    fn set_origin(&mut self, origin: Point) {
        self.text_widget.set_origin(origin)
    }

    fn widget_id(&self) -> &WidgetId {
        self.text_widget.widget_id()
    }
}
