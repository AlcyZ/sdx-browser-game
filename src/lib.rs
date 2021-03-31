use gl_matrix::common::Mat4;
use gl_matrix::{mat4, vec3};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, WebGlRenderingContext};

use renderer::mesh::shader::program;

use crate::renderer::mesh::shader::MeshRenderDescriptor;

mod definitions;
mod loader;
mod renderer;
mod util;

fn resize_canvas(canvas: &HtmlCanvasElement) {
    let display_width = canvas.client_width() as u32;
    let display_height = canvas.client_height() as u32;

    let canvas_width = canvas.width();
    let canvas_height = canvas.height();

    let need_resize = canvas_width != display_width || canvas_height != display_height;

    if need_resize {
        canvas.set_width(display_width);
        canvas.set_height(display_height);
    }
}
