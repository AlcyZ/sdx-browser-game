use crate::definitions::gltf::{GlTf, GlTfScene};
use crate::loader::glb::GlbBuffer;
use crate::renderer::camera::simple::SimpleCamera;
use crate::renderer::mesh::{Mesh, MeshRenderDescriptor};
use gl_matrix::mat4;
use wasm_bindgen::prelude::*;
use web_sys::{HtmlCanvasElement, WebGlRenderingContext};

#[derive(Debug)]
enum NodeType {
    Mesh(Mesh),
}

#[derive(Debug)]
struct SceneNode {
    name: String,
    rotation: Option<[f32; 4]>,
    translation: Option<[f32; 3]>,
    node_type: NodeType,
}

#[wasm_bindgen]
#[derive(Debug)]
struct Scene {
    nodes: Vec<SceneNode>,
    camera: SimpleCamera,
}

#[wasm_bindgen]
impl Scene {
    pub async fn from_url(
        gl: WebGlRenderingContext,
        canvas: HtmlCanvasElement,
        url: String,
    ) -> Result<Scene, JsValue> {
        let glb = crate::loader::glb::Glb::from_url(&url).await?;
        let scene_id = glb.json.gltf.scene;
        let scene = glb
            .json
            .gltf
            .scenes
            .get(scene_id)
            .ok_or(JsValue::from_str(&format!(
                "Scene with id {} not found",
                scene_id
            )))?;
        let aspect = (canvas.width() / canvas.height()) as f32;

        // Todo: Fix hardcoded camera
        let camera = SimpleCamera::new([-2., 5., 10.], [0., 0., 0.], [0., 1., 0.], aspect);

        Ok(Scene::from_gltf(&glb.json.gltf, &scene, &gl, &glb.buffer, Some(camera)).await?)
    }

    pub fn render(&self, gl: &WebGlRenderingContext) -> Result<(), JsValue> {
        for node in &self.nodes {
            match &node.node_type {
                NodeType::Mesh(mesh) => {
                    let descriptor = MeshRenderDescriptor {
                        model_matrix: mat4::create(),
                        view_matrix: self.camera.view(),
                        projection_matrix: self.camera.projection(),
                    };
                    mesh.render(&gl, &descriptor)?;
                }
            }
        }

        Ok(())
    }
}

impl Scene {
    async fn from_gltf(
        gltf: &GlTf,
        scene: &GlTfScene,
        gl: &WebGlRenderingContext,
        glb_buffer: &GlbBuffer,
        camera: Option<SimpleCamera>,
    ) -> Result<Scene, JsValue> {
        let mut nodes = vec![];
        for node_id in &scene.nodes {
            let node = gltf.nodes.get(*node_id).ok_or(JsValue::from_str(&format!(
                "Node with id {} not found",
                node_id
            )))?;

            if let Some(mesh_id) = node.mesh {
                let mesh = gltf.meshes.get(mesh_id).ok_or(JsValue::from_str(&format!(
                    "could not find mesh with id {}",
                    mesh_id
                )))?;
                let node_type =
                    NodeType::Mesh(Mesh::from_gltf(&gl, &mesh, &gltf, &glb_buffer).await?);

                nodes.push(SceneNode {
                    name: node.name.clone(),
                    rotation: node.rotation,
                    translation: node.translation,
                    node_type,
                })
            }
        }
        let camera = camera.ok_or(JsValue::from_str(
            "camera from gltf is currently not supported",
        ))?;

        Ok(Scene { nodes, camera })
    }
}
