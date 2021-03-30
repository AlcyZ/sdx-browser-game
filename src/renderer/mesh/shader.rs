use crate::definitions::gltf::{GlTf, GlTfAccessor, GlTfBufferView, GlTfMeshPrimitive};
use crate::loader::glb::GlbBuffer;
use gl_matrix::common::Mat4;
use js_sys::Promise;
use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{
    HtmlImageElement, WebGlBuffer, WebGlProgram, WebGlRenderingContext, WebGlTexture,
    WebGlUniformLocation,
};

#[derive(Debug)]
struct MeshUniformLocations {
    model_matrix: Option<WebGlUniformLocation>,
    view_matrix: Option<WebGlUniformLocation>,
    projection_matrix: Option<WebGlUniformLocation>,
}

#[derive(Debug)]
struct MeshAttributeLocations {
    position: i32,
    normal: i32,
    texture_coord: i32,
}

#[derive(Debug)]
struct MeshShaderLocations {
    uniform: MeshUniformLocations,
    attribute: MeshAttributeLocations,
}

impl MeshShaderLocations {
    fn new(gl: &WebGlRenderingContext, program: &WebGlProgram) -> MeshShaderLocations {
        let uniform = MeshUniformLocations {
            model_matrix: gl.get_uniform_location(&program, "modelMatrix"),
            view_matrix: gl.get_uniform_location(&program, "viewMatrix"),
            projection_matrix: gl.get_uniform_location(&program, "projectionMatrix"),
        };

        let attribute = MeshAttributeLocations {
            position: gl.get_attrib_location(&program, "position"),
            normal: gl.get_attrib_location(&program, "normal"),
            texture_coord: gl.get_attrib_location(&program, "textureCoords"),
        };

        MeshShaderLocations { uniform, attribute }
    }
}

#[derive(Debug)]
struct MeshShaderFrameBuffer<'a> {
    buffer: WebGlBuffer,
    buffer_view: &'a GlTfBufferView,
    accessor: &'a GlTfAccessor,
}

impl<'a> MeshShaderFrameBuffer<'a> {
    fn new_from_accessor(
        gl: &WebGlRenderingContext,
        target: u32,
        accessor: &'a GlTfAccessor,
        gltf: &'a GlTf,
        glb_buffer: &GlbBuffer,
    ) -> Result<MeshShaderFrameBuffer<'a>, JsValue> {
        let buffer_view = gltf
            .buffer_views
            .get(accessor.buffer_view)
            .ok_or(JsValue::from_str("could not find buffer view"))?;

        let index = match accessor.component_type {
            WebGlRenderingContext::FLOAT => MeshShaderFrameBuffer::new_float32_buffer(
                &gl,
                target,
                &glb_buffer,
                &accessor,
                &buffer_view,
            ),
            WebGlRenderingContext::UNSIGNED_SHORT => MeshShaderFrameBuffer::new_uint16_buffer(
                &gl,
                target,
                &glb_buffer,
                &accessor,
                &buffer_view,
            ),
            _ => Err(JsValue::from_str("foo")),
        }?;

        Ok(index)
    }

    fn new_float32_buffer(
        gl: &WebGlRenderingContext,
        target: u32,
        glb_buffer: &GlbBuffer,
        accessor: &'a GlTfAccessor,
        buffer_view: &'a GlTfBufferView,
    ) -> Result<MeshShaderFrameBuffer<'a>, JsValue> {
        let byte_offset = glb_buffer.byte_offset + buffer_view.byte_offset;
        let length = buffer_view.byte_length;

        let buffer_data = js_sys::Float32Array::new_with_byte_offset_and_length(
            &glb_buffer.bin,
            byte_offset,
            length,
        );

        let buffer = MeshShaderFrameBuffer::new_buffer(&gl)?;
        gl.bind_buffer(target, Some(&buffer));
        gl.buffer_data_with_array_buffer_view(
            target,
            &buffer_data,
            WebGlRenderingContext::STATIC_DRAW,
        );

        Ok(MeshShaderFrameBuffer {
            accessor,
            buffer_view,
            buffer,
        })
    }

    fn new_uint16_buffer(
        gl: &WebGlRenderingContext,
        target: u32,
        glb_buffer: &GlbBuffer,
        accessor: &'a GlTfAccessor,
        buffer_view: &'a GlTfBufferView,
    ) -> Result<MeshShaderFrameBuffer<'a>, JsValue> {
        let byte_offset = glb_buffer.byte_offset + buffer_view.byte_offset;
        let length = buffer_view.byte_length;

        let buffer_data = js_sys::Uint16Array::new_with_byte_offset_and_length(
            &glb_buffer.bin,
            byte_offset,
            length,
        );

        let buffer = MeshShaderFrameBuffer::new_buffer(&gl)?;
        gl.bind_buffer(target, Some(&buffer));
        gl.buffer_data_with_array_buffer_view(
            target,
            &buffer_data,
            WebGlRenderingContext::STATIC_DRAW,
        );

        Ok(MeshShaderFrameBuffer {
            accessor,
            buffer_view,
            buffer,
        })
    }

    fn new_buffer(gl: &WebGlRenderingContext) -> Result<WebGlBuffer, JsValue> {
        match gl.create_buffer() {
            Some(buffer) => Ok(buffer),
            None => Err(JsValue::from_str("Could not create webgl buffer")),
        }
    }
}

#[derive(Debug)]
struct MeshShaderFrameBuffers<'a> {
    index: MeshShaderFrameBuffer<'a>,
    position: MeshShaderFrameBuffer<'a>,
    normal: Option<MeshShaderFrameBuffer<'a>>,
    texture_coord: Option<MeshShaderFrameBuffer<'a>>,
}

impl<'a> MeshShaderFrameBuffers<'a> {
    fn new(
        gl: &WebGlRenderingContext,
        primitive: &GlTfMeshPrimitive,
        gltf: &'a GlTf,
        glb_buffer: &GlbBuffer,
    ) -> Result<MeshShaderFrameBuffers<'a>, JsValue> {
        let indices_accessor = gltf
            .accessors
            .get(primitive.indices)
            .ok_or(JsValue::from_str("could not find indices accessor"))?;
        let position_accessor = gltf
            .accessors
            .get(primitive.attributes.position)
            .ok_or(JsValue::from_str("could not find position accessor"))?;

        let index = MeshShaderFrameBuffer::new_from_accessor(
            &gl,
            WebGlRenderingContext::ELEMENT_ARRAY_BUFFER,
            &indices_accessor,
            &gltf,
            &glb_buffer,
        )?;
        let position = MeshShaderFrameBuffer::new_from_accessor(
            &gl,
            WebGlRenderingContext::ARRAY_BUFFER,
            &position_accessor,
            &gltf,
            &glb_buffer,
        )?;
        let normal =
            MeshShaderFrameBuffers::try_new(&gl, primitive.attributes.normal, &gltf, &glb_buffer)?;
        let texture_coord = MeshShaderFrameBuffers::try_new(
            &gl,
            primitive.attributes.texture_coord_0,
            &gltf,
            &glb_buffer,
        )?;

        Ok(MeshShaderFrameBuffers {
            index,
            position,
            normal,
            texture_coord,
        })
    }

    fn try_new(
        gl: &WebGlRenderingContext,
        value: Option<usize>,
        gltf: &'a GlTf,
        glb_buffer: &GlbBuffer,
    ) -> Result<Option<MeshShaderFrameBuffer<'a>>, JsValue> {
        let result = match value {
            Some(index) => {
                let accessor = gltf
                    .accessors
                    .get(index)
                    .ok_or(JsValue::from_str("could not find accessor"))?;
                Some(MeshShaderFrameBuffer::new_from_accessor(
                    &gl,
                    WebGlRenderingContext::ARRAY_BUFFER,
                    &accessor,
                    &gltf,
                    &glb_buffer,
                )?)
            }
            None => None,
        };

        Ok(result)
    }
}

#[derive(Debug)]
struct MeshShaderTexture {
    image: HtmlImageElement,
    buffer: WebGlTexture,
    location: Option<WebGlUniformLocation>,
}

#[derive(Debug)]
struct MeshShaderTextures {
    base_color: Option<MeshShaderTexture>,
}

#[wasm_bindgen(module = "/assets/ffi/utility.js")]
extern "C" {
    fn load_image(data: js_sys::Uint8Array) -> Promise;
}

impl MeshShaderTextures {
    async fn new(
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

                                web_sys::console::log_2(
                                    &byte_offset.into(),
                                    &length.into(),
                                );

                                let texture_image =
                                    JsFuture::from(load_image(data_array)).await?
                                        .dyn_into::<HtmlImageElement>()?;

                                let texture_buffer = gl.create_texture().unwrap();
                                gl.bind_texture(WebGlRenderingContext::TEXTURE_2D, Some(&texture_buffer));
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

#[derive(Debug)]
pub(crate) struct MeshRenderDescriptor<'a> {
    pub(crate) gl: &'a WebGlRenderingContext,
    pub(crate) model_matrix: Mat4,
    pub(crate) view_matrix: Mat4,
    pub(crate) projection_matrix: Mat4,
}

#[derive(Debug)]
pub(crate) struct MeshShader<'a> {
    locations: MeshShaderLocations,
    frame_buffers: MeshShaderFrameBuffers<'a>,
    textures: MeshShaderTextures,
    program: &'a WebGlProgram,
}

impl<'a> MeshShader<'a> {
    pub(crate) async fn new(
        gl: &WebGlRenderingContext,
        program: &'a WebGlProgram,
        primitive: &GlTfMeshPrimitive,
        gltf: &'a GlTf,
        glb_buffer: &GlbBuffer,
    ) -> Result<MeshShader<'a>, JsValue> {
        let locations = MeshShaderLocations::new(&gl, &program);
        let frame_buffers = MeshShaderFrameBuffers::new(&gl, &primitive, &gltf, &glb_buffer)?;
        let textures = MeshShaderTextures::new(&gl, &program, &primitive, &gltf, &glb_buffer).await?;

        Ok(MeshShader {
            locations,
            frame_buffers,
            textures,
            program,
        })
    }

    pub(crate) fn render(&self, descriptor: MeshRenderDescriptor) -> Result<(), JsValue> {
        descriptor.gl.use_program(Some(&self.program));

        MeshShader::enable_attribute(
            &descriptor.gl,
            &self.frame_buffers.position,
            self.locations.attribute.position as u32,
        )?;

        if let Some(buffer) = &self.frame_buffers.normal {
            MeshShader::enable_attribute(
                &descriptor.gl,
                &buffer,
                self.locations.attribute.normal as u32,
            )?;
        }

        if let Some(buffer) = &self.frame_buffers.texture_coord {
            MeshShader::enable_attribute(
                &descriptor.gl,
                &buffer,
                self.locations.attribute.texture_coord as u32,
            )?;
        }

        descriptor.gl.uniform_matrix4fv_with_f32_array(
            self.locations.uniform.model_matrix.as_ref(),
            false,
            &descriptor.model_matrix,
        );
        descriptor.gl.uniform_matrix4fv_with_f32_array(
            self.locations.uniform.view_matrix.as_ref(),
            false,
            &descriptor.view_matrix,
        );
        descriptor.gl.uniform_matrix4fv_with_f32_array(
            self.locations.uniform.projection_matrix.as_ref(),
            false,
            &descriptor.projection_matrix,
        );

        let byte_offset = match self.frame_buffers.index.accessor.byte_offset {
            Some(byte_offset) => byte_offset,
            None => 0,
        } as i32;

        descriptor.gl.draw_elements_with_i32(
            WebGlRenderingContext::TRIANGLES,
            self.frame_buffers.index.accessor.count as i32,
            self.frame_buffers.index.accessor.component_type,
            byte_offset,
        );

        Ok(())
    }

    fn enable_attribute(
        gl: &WebGlRenderingContext,
        frame_buffer: &MeshShaderFrameBuffer,
        location: u32,
    ) -> Result<(), JsValue> {
        let byte_stride = match frame_buffer.buffer_view.byte_stride {
            Some(byte_stride) => byte_stride,
            None => 0,
        } as i32;
        let byte_offset = match frame_buffer.accessor.byte_offset {
            Some(byte_stride) => byte_stride,
            None => 0,
        } as i32;
        let size = MeshShader::accessor_type_2_size(&frame_buffer.accessor)?;

        gl.bind_buffer(
            WebGlRenderingContext::ARRAY_BUFFER,
            Some(&frame_buffer.buffer),
        );
        gl.enable_vertex_attrib_array(location);
        gl.vertex_attrib_pointer_with_i32(
            location,
            size,
            frame_buffer.accessor.component_type,
            false,
            byte_stride,
            byte_offset,
        );

        Ok(())
    }

    fn accessor_type_2_size(accessor: &GlTfAccessor) -> Result<i32, JsValue> {
        match accessor.type_name.as_ref() {
            "SCALAR" => Ok(1),
            "VEC2" => Ok(2),
            "VEC3" => Ok(3),
            _ => Err(JsValue::from_str("accessor type not supported")),
        }
    }
}
