use wasm_bindgen::prelude::*;

use crate::definitions::gltf::{GlTf, GlTfMesh};
use crate::loader::glb::GlbBuffer;
use crate::renderer::mesh::shader::MeshShader;
use web_sys::{ WebGlRenderingContext};

mod shader;

#[derive(Debug)]
struct MeshPrimitive {
    shader: MeshShader,
}

#[derive(Debug)]
pub(super) struct Mesh {
    name: String,
    primitives: Vec<MeshPrimitive>,
}

impl Mesh {
    pub(super) async fn from_gltf(
        gl: &WebGlRenderingContext,
        mesh: &GlTfMesh,
        gltf: &GlTf,
        glb_buffer: &GlbBuffer,
    ) -> Result<Mesh, JsValue> {
        let mut primitives = vec![];

        for primitive in &mesh.primitives {
            let shader = MeshShader::from_gltf(&gl, &primitive, &gltf, &glb_buffer).await?;

            primitives.push(MeshPrimitive { shader });
        }
        let name = mesh.name.clone();

        Ok(Mesh { name, primitives })
    }

    pub(super) fn render(
        &self,
        gl: &WebGlRenderingContext,
        descriptor: &MeshRenderDescriptor,
    ) -> Result<(), JsValue> {
        for primitive in &self.primitives {
            primitive.shader.render(&gl, &descriptor)?;
        }

        Ok(())
    }
}

#[derive(Debug)]
pub(super) struct MeshRenderDescriptor {
    pub(super) model_matrix: [f32; 16],
    pub(super) view_matrix: [f32; 16],
    pub(super) projection_matrix: [f32; 16],
}

// #[wasm_bindgen]
// impl MeshRenderDescriptor {
//     pub fn rotate_sample(&mut self) {
//         let a = self.model_matrix.clone();
//
//         mat4::rotate_y(&mut self.model_matrix, &a, to_radian(180.));
//     }
//
//     pub fn move_camera(&mut self, x: f32, y: f32, z: f32) {
//         self.camera.translate(x, y, z);
//     }
//
//     pub fn sample(canvas: HtmlCanvasElement) -> MeshRenderDescriptor {
//         let aspect = canvas.width() / canvas.height();
//
//         let mut model_matrix = mat4::create();
//
//         let eye = vec3::from_values(0., 3., 8.);
//         let center = vec3::from_values(0., 0., 0.);
//         let up = vec3::from_values(0., 1., 0.);
//
//         let camera = SimpleCamera::new(eye, center, up, aspect as f32);
//
//         MeshRenderDescriptor {
//             model_matrix,
//             camera,
//         }
//     }
// }
