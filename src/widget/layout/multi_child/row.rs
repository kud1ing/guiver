use crate::stroke::Stroke;
use crate::widget::{WidgetCommand, WidgetError, WidgetId};
use crate::widget_manager::WidgetBox;
use crate::{Event, SizeConstraints, VerticalAlignment, Widget, WidgetEvent};
use druid_shell::kurbo::{Point, Rect, Size};
use druid_shell::piet::{Piet, RenderContext};
use druid_shell::{piet, Region};
use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::cmp::max;

///
pub struct Row {
    child_widgets: Vec<WidgetBox>,
    debug_rendering: bool,
    debug_rendering_stroke: Stroke,
    is_hidden: bool,
    rectangle: Rect,
    size_constraints: SizeConstraints,
    spacing: f64,
    vertical_alignment: VerticalAlignment,
    widget_id: WidgetId,
}

impl Row {
    ///
    pub fn new(
        widget_id: WidgetId,
        debug_rendering_stroke: Stroke,
        vertical_alignment: VerticalAlignment,
        spacing: f64,
    ) -> Self {
        Row {
            child_widgets: vec![],
            debug_rendering: false,
            debug_rendering_stroke,
            is_hidden: false,
            rectangle: Rect::default(),
            size_constraints: SizeConstraints::unbounded(),
            spacing,
            vertical_alignment,
            widget_id,
        }
    }

    ///
    fn layout_children(&mut self) {
        let number_of_child_widgets = self.child_widgets.len();

        // There are no children.
        if number_of_child_widgets <= 0 {
            return;
        }

        let number_of_spacers = max(number_of_child_widgets - 1, 0);

        // Determine the child size constraints.
        let child_size_constraints = SizeConstraints::new(
            Size::new(0.0, self.size_constraints.minimum().height),
            Size::new(
                (self.size_constraints.maximum().width - number_of_spacers as f64 * self.spacing)
                    / (number_of_child_widgets as f64),
                self.size_constraints.maximum().height,
            ),
        );

        let mut parent_size = Size::ZERO;
        let mut child_x = self.rectangle.origin().x;

        for (i, child_widget) in &mut self.child_widgets.iter().enumerate() {
            let child_size = RefCell::borrow_mut(&child_widget)
                .borrow_mut()
                .apply_size_constraints(child_size_constraints);

            // Update the parent size.
            {
                parent_size.height = child_size.height.max(parent_size.height);
                parent_size.width += child_size.width;

                if i > 0 {
                    parent_size.width += self.spacing;
                }
            }

            // Determine the child's vertical position.
            let child_y = match self.vertical_alignment {
                VerticalAlignment::Bottom => {
                    self.rectangle.origin().y
                        + (self.rectangle.size().height - child_size.height).max(0.0)
                }
                VerticalAlignment::Middle => {
                    self.rectangle.origin().y
                        + 0.5 * (self.rectangle.size().height - child_size.height).max(0.0)
                }
                VerticalAlignment::Top => self.rectangle.origin().y,
            };

            // Set the children's origins.
            RefCell::borrow_mut(&child_widget)
                .borrow_mut()
                .set_origin((child_x, child_y).into());

            child_x += child_size.width + self.spacing;
        }

        // Set the parent size.
        self.rectangle = self.rectangle.with_size(parent_size);
    }
}

impl Widget for Row {
    ///
    fn apply_size_constraints(&mut self, size_constraints: SizeConstraints) -> Size {
        self.size_constraints = size_constraints;

        // Layout the children.
        self.layout_children();

        self.rectangle.size()
    }

    fn handle_command(&mut self, widget_command: WidgetCommand) -> Result<(), WidgetError> {
        match widget_command {
            WidgetCommand::AppendChild(child_widget) => {
                self.child_widgets.push(child_widget);
            }
            WidgetCommand::RemoveAllChildren => {
                self.child_widgets.clear();
            }
            WidgetCommand::RemoveChild(_) => {
                // TODO
                println!("`Row::handle_command(RemoveChild)`: TODO");
            }
            WidgetCommand::SetDebugRendering(debug_rendering) => {
                self.debug_rendering = debug_rendering;
            }
            WidgetCommand::SetFill(ref _value) => {
                return Err(WidgetError::CommandNotHandled(
                    self.widget_id,
                    widget_command,
                ));
            }
            WidgetCommand::SetFont(_) => {
                return Err(WidgetError::CommandNotHandled(
                    self.widget_id,
                    widget_command,
                ));
            }
            WidgetCommand::SetHasFocus(_) => {
                return Err(WidgetError::CommandNotHandled(
                    self.widget_id,
                    widget_command,
                ));
            }
            WidgetCommand::SetHorizontalAlignment(_) => {
                return Err(WidgetError::CommandNotHandled(
                    self.widget_id,
                    widget_command,
                ));
            }
            WidgetCommand::SetIsDisabled(_) => {
                // TODO
                println!("`Row::handle_command(SetIsDisabled)`: TODO");
            }
            WidgetCommand::SetIsHidden(is_hidden) => {
                // Hide/show this widget.
                self.is_hidden = is_hidden;
            }
            WidgetCommand::SetStroke(ref _value) => {
                return Err(WidgetError::CommandNotHandled(
                    self.widget_id,
                    widget_command,
                ));
            }
            WidgetCommand::SetValue(_) => {
                return Err(WidgetError::CommandNotHandled(
                    self.widget_id,
                    widget_command,
                ));
            }
            WidgetCommand::SetVerticalAlignment(vertical_alignment) => {
                self.vertical_alignment = vertical_alignment;
            }
        }

        Ok(())
    }

    fn handle_event(&mut self, event: &Event, widget_events: &mut Vec<WidgetEvent>) {
        // Iterate over the child widgets.
        for child_widget in &mut self.child_widgets {
            RefCell::borrow_mut(&child_widget).handle_event(event, widget_events);
        }
    }

    fn paint(&self, piet: &mut Piet, region: &Region) -> Result<(), piet::Error> {
        if self.is_hidden {
            return Ok(());
        }

        // Iterate over the child widgets.
        for child_widget in &self.child_widgets {
            RefCell::borrow(&child_widget).paint(piet, region)?;
        }

        // Render debug hints.
        if self.debug_rendering {
            piet.stroke(
                self.rectangle,
                &self.debug_rendering_stroke.brush,
                self.debug_rendering_stroke.width,
            );
        }

        Ok(())
    }

    fn set_origin(&mut self, origin: Point) {
        self.rectangle = self.rectangle.with_origin(origin);

        // Layout the children.
        self.layout_children();
    }

    fn widget_id(&self) -> &WidgetId {
        &self.widget_id
    }
}
