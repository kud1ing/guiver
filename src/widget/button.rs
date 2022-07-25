use crate::widget::{WidgetCommand, WidgetError};
use crate::widget_manager::WidgetBox;
use crate::{SizeConstraints, SystemEvent, Widget, WidgetEvent, WidgetId};
use druid_shell::kurbo::{Point, Rect, Size};
use druid_shell::piet::{Color, LinearGradient, PaintBrush, Piet, UnitPoint};
use druid_shell::Region;

///
pub struct Button {
    child_widget: WidgetBox,
    corner_radius: f64,
    fill_brush: Option<PaintBrush>,
    has_focus: bool,
    is_disabled: bool,
    is_hidden: bool,
    padding_horizontal: f64,
    padding_vertical: f64,
    rectangle: Rect,
    size_constraints: SizeConstraints,
    stroke_brush: Option<PaintBrush>,
    stroke_width: f64,
    widget_id: WidgetId,
}

impl Button {
    ///
    pub fn new(widget_id: WidgetId, child_widget: WidgetBox) -> Self {
        Button {
            child_widget,
            corner_radius: 4.0,
            fill_brush: Some(PaintBrush::Linear(LinearGradient::new(
                UnitPoint::TOP,
                UnitPoint::BOTTOM,
                (Color::rgb8(100, 100, 100), Color::rgb8(80, 80, 80)),
            ))),
            has_focus: false,
            is_disabled: false,
            is_hidden: false,
            padding_horizontal: 4.0,
            padding_vertical: 4.0,
            rectangle: Rect::default(),
            size_constraints: SizeConstraints::default(),
            stroke_brush: Some(PaintBrush::Color(Color::rgb8(200, 200, 200))),
            stroke_width: 1.0,
            widget_id,
        }
    }

    ///
    fn layout_child(&mut self) {
        // TODO
        println!("`Button::layout_child()`: TODO");
    }
}

impl Widget for Button {
    fn apply_size_constraints(&mut self, size_constraints: SizeConstraints) -> Size {
        self.size_constraints = size_constraints;
        self.rectangle = self.rectangle.with_size(*size_constraints.maximum());

        // Layout the child.
        self.layout_child();

        self.rectangle.size()
    }

    fn handle_command(&mut self, widget_command: WidgetCommand) -> Result<(), WidgetError> {
        match widget_command {
            WidgetCommand::AppendChild(_) => {
                // TODO
                println!("`Button::handle_command()`: TODO");
            }
            WidgetCommand::Clear => {
                // TODO
                println!("`Button::handle_command()`: TODO");
            }
            WidgetCommand::RemoveChild(_) => {
                // TODO
                println!("`Button::handle_command()`: TODO");
            }
            WidgetCommand::SetHasFocus(has_focus) => {
                self.has_focus = has_focus;
            }
            WidgetCommand::SetIsDisabled(is_disabled) => {
                self.is_disabled = is_disabled;
            }
            WidgetCommand::SetIsHidden(is_hidden) => {
                self.is_hidden = is_hidden;
            }
            WidgetCommand::SetValue(_) => {
                // TODO
                println!("`Button::handle_command()`: TODO");
            }
        }

        Ok(())
    }

    fn handle_event(&mut self, _system_event: &SystemEvent, _widget_events: &mut Vec<WidgetEvent>) {
        // TODO
        println!("`Button::handle_event()`: TODO");
        /*
        self.child_widget
            .borrow_mut()
            .handle_event(system_event, widget_events);
         */
    }

    fn paint(&self, piet: &mut Piet, region: &Region) {
        // TODO
        println!("`Button::paint()`: TODO");
        self.child_widget.borrow().paint(piet, region);

        /*

        let piet_text = piet.text();
        let layout = piet_text
            .new_text_layout(self.text.to_string())
            .font(FontFamily::SANS_SERIF, self.font_size)
            .text_color(Color::rgb8(255, 255, 255))
            .build()
            .unwrap();

        let text_size = layout.size();

        let button_height = self.font_size + 2.0 * self.vertical_padding;
        let button_width =
            (text_size.width + 2.0 * self.horizontal_padding).max(self.minimum_width);

        let button_rectangle = Rect::new(
            self.x,
            self.y,
            self.x + button_width,
            self.y + button_height,
        );

        // TODO
        let _mouse_is_in_widget = false; //button_rectangle.contains(context.input.mouse_position);
        let is_hovered = false; //mouse_is_in_widget;
        let is_clicked = false; //mouse_is_in_widget && context.input.mouse_down;

        // Draw the button.
        // =========================================================================================
        let button_shape = RoundedRect::from_rect(button_rectangle, self.corner_radius);

        // The button is clicked.
        if is_clicked {
            piet.fill(button_shape, &PaintBrush::Color(Color::rgb8(200, 200, 200)));
        }
        // The button is not clicked.
        else {
            if let Some(button_fill_brush) = &self.button_fill_brush {
                piet.fill(button_shape, button_fill_brush);
            }
        }

        if is_hovered {
            if let Some(button_stroke_brush) = &self.button_stroke_brush {
                piet.stroke(button_shape, button_stroke_brush, self.button_stroke_width);
            }
        }

        // Draw the text.
        // =========================================================================================
        piet.draw_text(
            &layout,
            button_rectangle.center() - (text_size.width * 0.5, text_size.height * 0.5),
        );
         */
    }

    fn set_origin(&mut self, origin: Point) {
        self.rectangle = self.rectangle.with_origin(origin);

        // Layout the child.
        self.layout_child();
    }

    fn widget_id(&self) -> &WidgetId {
        &self.widget_id
    }
}
