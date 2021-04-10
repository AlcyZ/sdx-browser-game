use crate::definitions::gltf::{GlTf, GlTfAccessor, GlTfBufferView, GlTfMeshPrimitive};
use crate::loader::glb::GlbBuffer;
use wasm_bindgen::prelude::*;
use web_sys::{WebGlBuffer, WebGlRenderingContext};

#[derive(Debug)]
pub(super) struct BufferView {
    pub(super) buffer: usize,
    pub(super) byte_offset: u32,
    pub(super) byte_length: u32,
    pub(super) target: Option<usize>,
    pub(super) byte_stride: Option<usize>,
}

impl From<&GlTfBufferView> for BufferView {
    fn from(buffer_view: &GlTfBufferView) -> Self {
        BufferView {
            buffer: buffer_view.buffer,
            byte_offset: buffer_view.byte_offset,
            byte_length: buffer_view.byte_length,
            target: buffer_view.target,
            byte_stride: buffer_view.byte_stride,
        }
    }
}

#[derive(Debug)]
pub(super) struct Accessor {
    pub(super) buffer_view: usize,
    pub(super) component_type: u32,
    pub(super) count: usize,
    pub(super) type_name: String,
    pub(super) byte_offset: Option<u32>,
    pub(super) max: Option<Vec<f32>>,
    pub(super) min: Option<Vec<f32>>,
}

impl From<&GlTfAccessor> for Accessor {
    fn from(accessor: &GlTfAccessor) -> Self {
        Accessor {
            buffer_view: accessor.buffer_view,
            component_type: accessor.component_type,
            count: accessor.count,
            type_name: String::from(&accessor.type_name),
            byte_offset: accessor.byte_offset,
            max: accessor.max.clone(),
            min: accessor.min.clone(),
        }
    }
}

#[derive(Debug)]
pub(super) struct MeshShaderFrameBuffer {
    pub(super) buffer: WebGlBuffer,
    pub(super) buffer_view: BufferView,
    pub(super) accessor: Accessor,
}

impl MeshShaderFrameBuffer {
    fn new_from_accessor(
        gl: &WebGlRenderingContext,
        target: u32,
        accessor: &GlTfAccessor,
        gltf: &GlTf,
        glb_buffer: &GlbBuffer,
    ) -> Result<MeshShaderFrameBuffer, JsValue> {
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
        accessor: &GlTfAccessor,
        buffer_view: &GlTfBufferView,
    ) -> Result<MeshShaderFrameBuffer, JsValue> {
        let byte_offset = glb_buffer.byte_offset + buffer_view.byte_offset;
        let length = buffer_view.byte_length;

        let buffer_data =
            js_sys::DataView::new(&glb_buffer.bin, byte_offset as usize, length as usize);

        let buffer = MeshShaderFrameBuffer::new_buffer(&gl)?;
        gl.bind_buffer(target, Some(&buffer));
        gl.buffer_data_with_array_buffer_view(
            target,
            &buffer_data,
            WebGlRenderingContext::STATIC_DRAW,
        );

        Ok(MeshShaderFrameBuffer {
            accessor: Accessor::from(accessor),
            buffer_view: BufferView::from(buffer_view),
            buffer,
        })
    }

    fn new_uint16_buffer(
        gl: &WebGlRenderingContext,
        target: u32,
        glb_buffer: &GlbBuffer,
        accessor: &GlTfAccessor,
        buffer_view: &GlTfBufferView,
    ) -> Result<MeshShaderFrameBuffer, JsValue> {
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
            accessor: Accessor::from(accessor),
            buffer_view: BufferView::from(buffer_view),
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
pub(super) struct MeshShaderFrameBuffers {
    pub(super) index: MeshShaderFrameBuffer,
    pub(super) position: MeshShaderFrameBuffer,
    pub(super) normal: Option<MeshShaderFrameBuffer>,
    pub(super) texture_coord: Option<MeshShaderFrameBuffer>,
}

impl MeshShaderFrameBuffers {
    pub(super) fn from_gltf(
        gl: &WebGlRenderingContext,
        primitive: &GlTfMeshPrimitive,
        gltf: &GlTf,
        glb_buffer: &GlbBuffer,
    ) -> Result<MeshShaderFrameBuffers, JsValue> {
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
        gltf: &GlTf,
        glb_buffer: &GlbBuffer,
    ) -> Result<Option<MeshShaderFrameBuffer>, JsValue> {
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
