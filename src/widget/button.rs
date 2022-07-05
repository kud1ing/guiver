/*
use crate::Widget;
use druid_shell::kurbo::{Rect, RoundedRect};
use druid_shell::piet::{
    Color, FontFamily, PaintBrush, Piet, RenderContext, Text, TextLayout, TextLayoutBuilder,
};
use druid_shell::Region;

///
#[derive(Clone, Debug)]
pub struct Button {
    button_fill_brush: Option<PaintBrush>,
    button_stroke_brush: Option<PaintBrush>,
    button_stroke_width: f64,
    corner_radius: f64,
    font_size: f64,
    horizontal_padding: f64,
    minimum_width: f64,
    text: String,
    vertical_padding: f64,
    x: f64,
    y: f64,
}

impl Button {
    pub fn new(text: impl ToString) -> Self {
        Button {
            button_fill_brush: Some(PaintBrush::Linear(druid::LinearGradient::new(
                druid::UnitPoint::TOP,
                druid::UnitPoint::BOTTOM,
                (Color::rgb8(100, 100, 100), Color::rgb8(80, 80, 80)),
            ))),
            button_stroke_brush: Some(PaintBrush::Color(Color::rgb8(200, 200, 200))),
            button_stroke_width: 1.0,
            corner_radius: 4.0,
            font_size: 14.0,
            horizontal_padding: 4.0,
            minimum_width: 0.0,
            text: text.to_string(),
            vertical_padding: 4.0,
            x: 0.0,
            y: 0.0,
        }
    }

    ///
    pub fn button_fill_brush(mut self, button_fill_brush: Option<PaintBrush>) -> Self {
        self.button_fill_brush = button_fill_brush;
        self
    }

    ///
    pub fn font_size(mut self, font_size: f64) -> Self {
        self.font_size = font_size;
        self
    }

    ///
    pub fn minimum_width(mut self, minimum_width: f64) -> Self {
        self.minimum_width = minimum_width;
        self
    }

    ///
    pub fn x(mut self, x: f64) -> Self {
        self.x = x;
        self
    }

    ///
    pub fn y(mut self, y: f64) -> Self {
        self.y = y;
        self
    }
}

impl Widget for Button {
    fn box_constraints(&self) -> BoxConstraints {
        todo!()
    }

    fn draw(&mut self, piet: &mut Piet, _region: &Region) {
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
    }

    fn handle_event(&mut self, _event: &Event, _region_to_redraw: &mut Region) {}

    fn rectangle(&mut self) -> &Rect {
        todo!()
    }

    fn set_rectangle(&mut self, _rectangle: Rect) {
        todo!()
    }
}
*/
