use crate::widget::{Text, WidgetCommand, WidgetError};
use crate::{Event, Font, Piet, Size, SizeConstraints, Stroke, Widget, WidgetEvent, WidgetId};
use druid_shell::kurbo::Point;
use druid_shell::piet::Error;
use druid_shell::Region;

///
pub struct Hyperlink {
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
        adjust_font(&mut font_visited);

        Hyperlink {
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

    fn handle_command(&mut self, widget_command: WidgetCommand) -> Result<(), WidgetError> {
        todo!()
    }

    fn handle_event(&mut self, event: &Event, widget_events: &mut Vec<WidgetEvent>) {
        todo!()
    }

    fn paint(&self, piet: &mut Piet, region: &Region) -> Result<(), Error> {
        self.text_widget.paint(piet, region)
    }

    fn set_origin(&mut self, origin: Point) {
        self.text_widget.set_origin(origin)
    }

    fn size(&self) -> Size {
        self.text_widget.size()
    }

    fn widget_id(&self) -> &WidgetId {
        self.text_widget.widget_id()
    }
}
