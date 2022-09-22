use crate::stroke::Stroke;
use crate::widget::core::WidgetCore;
use crate::widget::{WidgetCommand, WidgetError, WidgetId};
use crate::{Event, SizeConstraints, Widget, WidgetEvent};
use druid_shell::kurbo::{Line, Point, Rect, Size};
use druid_shell::piet::{Color, Piet, RenderContext};
use druid_shell::{piet, Region};
use piet::{PaintBrush, StrokeDash, StrokeStyle};

/// A placeholder widget.
pub struct Placeholder {
    core: WidgetCore,
    desired_size: Size,
    fill: Option<PaintBrush>,
    stroke: Option<Stroke>,
    widget_id: WidgetId,
}

impl Placeholder {
    pub fn new(widget_id: WidgetId, debug_rendering_stroke: Stroke, desired_size: Size) -> Self {
        let stroke_style = StrokeStyle {
            line_join: Default::default(),
            line_cap: Default::default(),
            dash_pattern: StrokeDash::default(),
            dash_offset: 0.0,
        };

        Placeholder {
            core: WidgetCore::new(widget_id, debug_rendering_stroke),
            desired_size,
            fill: None,
            stroke: Some(Stroke {
                stroke_brush: PaintBrush::Color(Color::rgb8(255, 255, 255)),
                stroke_style: stroke_style.dash_pattern(&[4.0, 2.0]),
                stroke_width: 1.0,
            }),
            widget_id,
        }
    }
}

impl Widget for Placeholder {
    ///
    fn apply_size_constraints(&mut self, size_constraints: SizeConstraints) -> Size {
        self.core.rectangle = self.core.rectangle.with_size(
            self.desired_size
                .clamp(*size_constraints.minimum(), *size_constraints.maximum()),
        );
        self.core.rectangle.size()
    }

    fn handle_command(&mut self, widget_command: &WidgetCommand) -> Result<(), WidgetError> {
        match widget_command {
            WidgetCommand::SetFill(fill) => {
                self.fill = fill.clone();
                Ok(())
            }
            WidgetCommand::SetStroke(_) => {
                // TODO
                println!("`Placeholder::handle_command(SetStroke)`: TODO");

                Ok(())
            }
            _ => self.core.handle_command(widget_command),
        }
    }

    fn handle_event(&mut self, _event: &Event, _widget_events: &mut Vec<WidgetEvent>) {
        // Nothing to do.
    }

    fn paint(&self, piet: &mut Piet, _region: &Region) -> Result<(), piet::Error> {
        // The placeholder widget is hidden.
        if self.core.is_hidden {
            return Ok(());
        }

        // TODO: check the region

        // Fill.
        if let Some(fill) = &self.fill {
            piet.fill(&self.core.rectangle, fill);
        }

        // Stroke.
        if let Some(stroke) = &self.stroke {
            // Draw a cross.
            piet.stroke_styled(
                Line::new(
                    (self.core.rectangle.x0, self.core.rectangle.y0),
                    (self.core.rectangle.x1, self.core.rectangle.y1),
                ),
                &stroke.stroke_brush,
                stroke.stroke_width,
                &stroke.stroke_style,
            );
            piet.stroke_styled(
                Line::new(
                    (self.core.rectangle.x0, self.core.rectangle.y1),
                    (self.core.rectangle.x1, self.core.rectangle.y0),
                ),
                &stroke.stroke_brush,
                stroke.stroke_width,
                &stroke.stroke_style,
            );

            // Draw the rectangle.
            piet.stroke_styled(
                &self.core.rectangle,
                &stroke.stroke_brush,
                stroke.stroke_width,
                &stroke.stroke_style,
            );
        }

        Ok(())
    }

    fn rectangle(&self) -> &Rect {
        &self.core.rectangle
    }

    fn set_origin(&mut self, origin: Point) {
        self.core.rectangle = self.core.rectangle.with_origin(origin)
    }

    fn widget_id(&self) -> &WidgetId {
        &self.core.widget_id
    }
}

// =================================================================================================

#[cfg(test)]
mod tests {
    use crate::widget::Placeholder;
    use crate::{Size, SizeConstraints, Stroke, Widget};

    #[test]
    fn test_apply_size_constraints() {
        // Create the placeholder widget.
        let placeholder_maximum_size = Size::new(200.0, 100.0);
        let mut placeholder_widget =
            Placeholder::new(0, Stroke::default(), placeholder_maximum_size);

        // Apply an unbounded `SizeConstraints`.
        {
            placeholder_widget.apply_size_constraints(SizeConstraints::unbounded());

            assert_eq!(
                placeholder_widget.rectangle().size(),
                placeholder_maximum_size,
                "The placeholder widget should not be larger than its maximum size"
            );
        }

        // Common tests are in the integration test directory.
    }

    #[test]
    fn test_handle_command() {
        // TODO
    }

    #[test]
    fn test_handle_event() {
        // TODO
    }
}
