#![recursion_limit = "1024"]

use wasm_bindgen::JsCast;
use web_sys::window;

fn main() {
    console_error_panic_hook::set_once();

    let window = window().expect("Could not get the window");
    let document = window.document().expect("Could not get the document");

    // Create a canvas.
    let canvas = document
        .create_element("canvas")
        .expect("Could not create the canvas");

    // Add the canvas.
    let body = document.body().expect("Could not get body");
    body.append_child(canvas.as_ref())
        .expect("Could not append text");

    // Cast the canvas to an HTML canvas element.
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .expect("Could not get type cast the canvas");

    // Get the rendering context.
    let rendering_context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .expect("Could not get the context");

    // Avoid canvas blur.
    {
        let width: f64 = 640.0;
        let height: f64 = 480.0;

        let scaling: f64 = window.device_pixel_ratio();

        let css_style_declaration = canvas.style();
        css_style_declaration.set_property("width", &format!("{width}px"));
        css_style_declaration.set_property("height", &format!("{height}px"));

        canvas.set_width((width * scaling) as u32);
        canvas.set_height((height * scaling) as u32);

        rendering_context.scale(scaling, scaling);
        rendering_context.translate(-0.5, -0.5);
    }

    // Draw.
    {
        rendering_context.begin_path();

        rendering_context
            .arc(75.0, 75.0, 50.0, 0.0, 5.0)
            .expect("Could not draw");

        rendering_context.move_to(100.0, 50.0);
        rendering_context.line_to(100.0, 150.0);

        rendering_context.move_to(50.0, 100.0);
        rendering_context.line_to(150.0, 100.0);

        rendering_context.stroke();
    }
}
