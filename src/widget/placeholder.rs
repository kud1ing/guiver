use crate::stroke::Stroke;
use crate::widget::{WidgetCommand, WidgetError, WidgetId};
use crate::{Event, SizeConstraints, Widget, WidgetEvent};
use druid_shell::kurbo::{Line, Point, Rect, Size};
use druid_shell::piet::{Color, Piet, RenderContext};
use druid_shell::{piet, Region};
use piet::{PaintBrush, StrokeDash, StrokeStyle};

/// A placeholder widget.
pub struct Placeholder {
    fill: Option<PaintBrush>,
    is_hidden: bool,
    maximum_size: Size,
    rectangle: Rect,
    stroke: Option<Stroke>,
    widget_id: WidgetId,
}

impl Placeholder {
    pub fn new(widget_id: WidgetId, maximum_size: Size) -> Self {
        let style = StrokeStyle {
            line_join: Default::default(),
            line_cap: Default::default(),
            dash_pattern: StrokeDash::default(),
            dash_offset: 0.0,
        };

        Placeholder {
            fill: None,
            is_hidden: false,
            maximum_size,
            rectangle: Rect::default(),
            stroke: Some(Stroke {
                brush: PaintBrush::Color(Color::rgb8(255, 255, 255)),
                style: style.dash_pattern(&[4.0, 2.0]),
                width: 1.0,
            }),
            widget_id,
        }
    }
}

impl Widget for Placeholder {
    ///
    fn apply_size_constraints(&mut self, size_constraints: SizeConstraints) -> Size {
        self.rectangle = self.rectangle.with_size(
            self.maximum_size
                .clamp(*size_constraints.minimum(), *size_constraints.maximum()),
        );
        self.rectangle.size()
    }

    fn handle_command(&mut self, widget_command: WidgetCommand) -> Result<(), WidgetError> {
        match widget_command {
            WidgetCommand::AppendChild(_) => {
                return Err(WidgetError::CommandNotHandled(
                    self.widget_id,
                    widget_command,
                ));
            }
            WidgetCommand::RemoveAllChildren => {
                return Err(WidgetError::CommandNotHandled(
                    self.widget_id,
                    widget_command,
                ));
            }
            WidgetCommand::RemoveChild(_) => {
                return Err(WidgetError::CommandNotHandled(
                    self.widget_id,
                    widget_command,
                ));
            }
            WidgetCommand::SetDebugRendering(_debug_rendering) => {
                // Debug rendering is unnecessary for placeholder widgets.
            }
            WidgetCommand::SetFill(fill) => {
                self.fill = fill;
            }
            WidgetCommand::SetFont(_) => {
                return Err(WidgetError::CommandNotHandled(
                    self.widget_id,
                    widget_command,
                ));
            }
            WidgetCommand::SetHasFocus(_has_focus) => {
                return Err(WidgetError::CommandNotHandled(
                    self.widget_id,
                    widget_command,
                ));
            }
            WidgetCommand::SetHorizontalAlignment(_) => {}
            WidgetCommand::SetIsDisabled(_) => {}
            WidgetCommand::SetIsHidden(is_hidden) => {
                self.is_hidden = is_hidden;
            }
            WidgetCommand::SetStroke(_) => {
                // TODO
                println!("`Placeholder::handle_command(SetStroke)`: TODO");
            }
            WidgetCommand::SetValue(ref _value) => {
                return Err(WidgetError::CommandNotHandled(
                    self.widget_id,
                    widget_command,
                ));
            }
            WidgetCommand::SetVerticalAlignment(_) => {}
        }

        Ok(())
    }

    fn handle_event(&mut self, _event: &Event, _widget_events: &mut Vec<WidgetEvent>) {
        // Nothing to do.
    }

    fn paint(&self, piet: &mut Piet, _region: &Region) -> Result<(), piet::Error> {
        // The placeholder widget is hidden.
        if self.is_hidden {
            return Ok(());
        }

        // TODO: check the region

        // Fill.
        if let Some(fill) = &self.fill {
            piet.fill(&self.rectangle, fill);
        }

        // Stroke.
        if let Some(stroke) = &self.stroke {
            // Draw a cross.
            piet.stroke_styled(
                Line::new(
                    (self.rectangle.x0, self.rectangle.y0),
                    (self.rectangle.x1, self.rectangle.y1),
                ),
                &stroke.brush,
                stroke.width,
                &stroke.style,
            );
            piet.stroke_styled(
                Line::new(
                    (self.rectangle.x0, self.rectangle.y1),
                    (self.rectangle.x1, self.rectangle.y0),
                ),
                &stroke.brush,
                stroke.width,
                &stroke.style,
            );

            // Draw the rectangle.
            piet.stroke_styled(&self.rectangle, &stroke.brush, stroke.width, &stroke.style);
        }

        Ok(())
    }

    fn set_origin(&mut self, origin: Point) {
        self.rectangle = self.rectangle.with_origin(origin)
    }

    fn size(&self) -> Size {
        self.rectangle.size()
    }

    fn widget_id(&self) -> &WidgetId {
        &self.widget_id
    }
}

// =================================================================================================

#[cfg(test)]
mod tests {
    use crate::widget::Placeholder;
    use crate::{Size, SizeConstraints, Widget};

    #[test]
    fn test_apply_size_constraints() {
        // Create the placeholder widget.
        let placeholder_maximum_size = Size::new(200.0, 100.0);
        let mut placeholder_widget = Placeholder::new(0, placeholder_maximum_size);

        // Apply an unbounded `SizeConstraints`.
        {
            placeholder_widget.apply_size_constraints(SizeConstraints::unbounded());

            assert_eq!(
                placeholder_widget.size(),
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
