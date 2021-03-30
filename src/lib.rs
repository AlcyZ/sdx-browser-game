use crate::renderer::mesh::shader::MeshRenderDescriptor;
use gl_matrix::common::{Mat4};
use gl_matrix::{mat4, vec3};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, WebGlRenderingContext};

mod definitions;
mod loader;
mod program;
mod renderer;
mod util;

const F_SHADER: &str = "#version 100

precision mediump float;

varying vec2 fTextureCoords;

uniform sampler2D uTexture;

void main() {
    // gl_FragColor = vec4(0.8, 0.0, 0.0, 1.0);
    gl_FragColor = texture2D(uTexture, fTextureCoords);
}";

const V_SHADER: &str = "#version 100

attribute vec3 position;
attribute vec3 normal;
attribute vec2 textureCoords;

varying vec3 fNormal;
varying vec2 fTextureCoords;

uniform mat4 modelMatrix;
uniform mat4 viewMatrix;
uniform mat4 projectionMatrix;

void main() {
    vec4 worldPosition = modelMatrix * vec4(position, 1.0);
    gl_Position = projectionMatrix * viewMatrix * worldPosition;

    fNormal = normal;
    fTextureCoords = textureCoords;
}";

#[wasm_bindgen]
pub async fn example() {
    if let Err(e) = run().await {
        web_sys::console::error_2(&"Loading glb failed:".into(), &e);
    }
}

async fn run() -> Result<(), JsValue> {
    let url = "/models/test/suzanne1.glb";
    let glb = loader::glb::Glb::from_url(&url).await?;

    let document = util::document();

    let canvas = document
        .get_element_by_id("surface")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()?;
    let gl = canvas
        .get_context("webgl")?
        .unwrap()
        .dyn_into::<WebGlRenderingContext>()?;

    let vertex_shader = program::compile_vertex_shader(&gl, V_SHADER)?;
    let fragment_shader = program::compile_fragment_shader(&gl, F_SHADER)?;
    let program = program::link_program(&gl, &vertex_shader, &fragment_shader)?;

    let primitive = glb
        .json
        .gltf
        .meshes
        .first()
        .unwrap()
        .primitives
        .first()
        .unwrap();

    let mesh_shader = renderer::mesh::shader::MeshShader::new(
        &gl,
        &program,
        &primitive,
        &glb.json.gltf,
        &glb.buffer,
    ).await?;

    // let mut model_matrix: Mat4 = [0.; 16];
    // let mut view_matrix: Mat4 = [0.; 16];
    let mut projection_matrix: Mat4 = [0.; 16];

    let mut model_matrix: Mat4 = [
        0.9995500445365906,
        0.0,
        -0.029995501041412354,
        0.0,
        0.0,
        1.0,
        0.0,
        0.0,
        0.029995501041412354,
        0.0,
        0.9995500445365906,
        0.0,
        0.0,
        0.0,
        0.0,
        1.0,
    ];

    let view_matrix = [
        1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, -7.0, 1.0,
    ];

    let eye = vec3::from_values(0., 0., -15.);
    let center = vec3::from_values(0., 0., 0.);
    let up = vec3::from_values(0., 1., 0.);

    let width = canvas.width();
    let height = canvas.height();

    let aspect = width / height;

    // mat4::translate(&mut view_matrix, &[0.; 16], &[0.0, 0.0, -7.0]);
    // mat4::look_at(&mut view_matrix, &eye, &center, &up);
    mat4::perspective(
        &mut projection_matrix,
        1.04,
        aspect as f32,
        0.1,
        Some(100.0),
    );

    resize_canvas(&canvas);
    gl.viewport(0, 0, canvas.width() as i32, canvas.height() as i32);

    gl.clear_color(0.2, 0.2, 0.2, 1.0);
    gl.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);

    gl.enable(WebGlRenderingContext::CULL_FACE);
    gl.enable(WebGlRenderingContext::DEPTH_TEST);

    let descriptor = MeshRenderDescriptor {
        gl: &gl,
        model_matrix,
        view_matrix,
        projection_matrix,
    };
    mesh_shader.render(descriptor)?;

    // web_sys::console::log_1(&format!("rs-model: {:#?}", model_matrix).into());
    // web_sys::console::log_1(&format!("rs-view: {:#?}", view_matrix).into());
    // web_sys::console::log_1(&format!("rs-projection: {:#?}", projection_matrix).into());
    // web_sys::console::log_1(&format!("{:#?}", glb).into());

    Ok(())
}

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
