use crate::definitions::gltf::{GlTf, GlTfMeshPrimitive};
use crate::loader::glb::GlbBuffer;
use js_sys::Promise;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{
    HtmlImageElement, WebGlProgram, WebGlRenderingContext, WebGlTexture, WebGlUniformLocation,
};

#[wasm_bindgen(module = "/assets/ffi/utility.js")]
extern "C" {
    fn load_image(data: js_sys::Uint8Array) -> Promise;
}

#[derive(Debug)]
struct MeshShaderTexture {
    image: HtmlImageElement,
    buffer: WebGlTexture,
    location: Option<WebGlUniformLocation>,
}

#[derive(Debug)]
pub(super) struct MeshShaderTextures {
    base_color: Option<MeshShaderTexture>,
}

impl MeshShaderTextures {
    pub(super) async fn new(
        gl: &WebGlRenderingContext,
        program: &WebGlProgram,
        primitive: &GlTfMeshPrimitive,
        gltf: &GlTf,
        glb_buffer: &GlbBuffer,
    ) -> Result<MeshShaderTextures, JsValue> {
        let base_color = match primitive.material {
            Some(index) => {
                let material = gltf
                    .materials
                    .get(index)
                    .ok_or(JsValue::from_str("could not find texture"))?;

                match &material.pbr_metallic_roughness.base_color_texture {
                    Some(reference) => {
                        let texture = gltf
                            .textures
                            .get(reference.index)
                            .ok_or(JsValue::from_str("could not find texture"))?;
                        let image = gltf
                            .images
                            .get(texture.source)
                            .ok_or(JsValue::from_str("could not find texture image"))?;

                        match image.buffer_view {
                            Some(index) => {
                                let buffer_view = gltf
                                    .buffer_views
                                    .get(index)
                                    .ok_or("could not find texture image buffer view")?;

                                let byte_offset = glb_buffer.byte_offset + buffer_view.byte_offset;
                                let length = buffer_view.byte_length;

                                let data_array =
                                    js_sys::Uint8Array::new_with_byte_offset_and_length(
                                        &glb_buffer.bin,
                                        byte_offset,
                                        length,
                                    );
                                let mut options = web_sys::BlobPropertyBag::new();
                                options.type_("image/png");

                                web_sys::console::log_2(&byte_offset.into(), &length.into());

                                let texture_image = JsFuture::from(load_image(data_array))
                                    .await?
                                    .dyn_into::<HtmlImageElement>()?;

                                let texture_buffer = gl.create_texture().unwrap();
                                gl.bind_texture(
                                    WebGlRenderingContext::TEXTURE_2D,
                                    Some(&texture_buffer),
                                );
                                gl.tex_image_2d_with_u32_and_u32_and_image(
                                    WebGlRenderingContext::TEXTURE_2D,
                                    0,
                                    WebGlRenderingContext::RGBA as i32,
                                    WebGlRenderingContext::RGBA,
                                    WebGlRenderingContext::UNSIGNED_BYTE,
                                    &texture_image,
                                )?;
                                gl.generate_mipmap(WebGlRenderingContext::TEXTURE_2D);

                                Some(MeshShaderTexture {
                                    buffer: texture_buffer,
                                    image: texture_image,
                                    location: gl.get_uniform_location(&program, "uTexture"),
                                })
                            }
                            None => None,
                        }
                    }
                    None => None,
                }
            }
            None => None,
        };

        Ok(MeshShaderTextures { base_color })
    }
}
