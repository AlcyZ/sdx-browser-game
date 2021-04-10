use wasm_bindgen::prelude::*;
use web_sys::{WebGlProgram, WebGlRenderingContext};

use crate::definitions::gltf::{GlTf, GlTfMeshPrimitive};
use crate::loader::glb::GlbBuffer;
use crate::renderer::mesh::shader::buffers::{
    Accessor, MeshShaderFrameBuffer, MeshShaderFrameBuffers,
};
use crate::renderer::mesh::shader::locations::MeshShaderLocations;
use crate::renderer::mesh::shader::textures::MeshShaderTextures;
use crate::renderer::mesh::MeshRenderDescriptor;

mod buffers;
mod locations;
pub mod program;
mod textures;

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

const F_SHADER: &str = "#version 100

precision mediump float;

varying vec2 fTextureCoords;

uniform sampler2D uTexture;

void main() {
    // gl_FragColor = vec4(0.8, 0.0, 0.0, 1.0);
    gl_FragColor = texture2D(uTexture, fTextureCoords);
}";

#[derive(Debug)]
pub struct MeshShader {
    locations: MeshShaderLocations,
    frame_buffers: MeshShaderFrameBuffers,
    textures: MeshShaderTextures,
    program: WebGlProgram,
}

impl MeshShader {
    pub(super) async fn from_gltf(
        gl: &WebGlRenderingContext,
        primitive: &GlTfMeshPrimitive,
        gltf: &GlTf,
        glb_buffer: &GlbBuffer,
    ) -> Result<MeshShader, JsValue> {
        let program = program::compile_to_program(&gl, V_SHADER, F_SHADER)?;
        let locations = MeshShaderLocations::new(&gl, &program);
        let frame_buffers = MeshShaderFrameBuffers::from_gltf(&gl, &primitive, &gltf, &glb_buffer)?;
        let textures =
            MeshShaderTextures::from_gltf(&gl, &program, &primitive, &gltf, &glb_buffer).await?;

        Ok(MeshShader {
            locations,
            frame_buffers,
            textures,
            program,
        })
    }

    pub(super) fn render(
        &self,
        gl: &WebGlRenderingContext,
        descriptor: &MeshRenderDescriptor,
    ) -> Result<(), JsValue> {
        gl.use_program(Some(&self.program));

        MeshShader::enable_attribute(
            &gl,
            &self.frame_buffers.position,
            self.locations.attribute.position as u32,
        )?;

        if let Some(buffer) = &self.frame_buffers.normal {
            MeshShader::enable_attribute(&gl, &buffer, self.locations.attribute.normal as u32)?;
        }

        if let Some(buffer) = &self.frame_buffers.texture_coord {
            MeshShader::enable_attribute(
                &gl,
                &buffer,
                self.locations.attribute.texture_coord as u32,
            )?;
        }

        gl.uniform_matrix4fv_with_f32_array(
            self.locations.uniform.model_matrix.as_ref(),
            false,
            &descriptor.model_matrix,
        );
        gl.uniform_matrix4fv_with_f32_array(
            self.locations.uniform.view_matrix.as_ref(),
            false,
            &descriptor.view_matrix,
        );
        gl.uniform_matrix4fv_with_f32_array(
            self.locations.uniform.projection_matrix.as_ref(),
            false,
            &descriptor.projection_matrix,
        );

        let byte_offset = match self.frame_buffers.index.accessor.byte_offset {
            Some(byte_offset) => byte_offset,
            None => 0,
        } as i32;

        gl.draw_elements_with_i32(
            WebGlRenderingContext::TRIANGLES,
            self.frame_buffers.index.accessor.count as i32,
            self.frame_buffers.index.accessor.component_type,
            byte_offset,
        );

        Ok(())
    }
}

impl MeshShader {
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

    fn accessor_type_2_size(accessor: &Accessor) -> Result<i32, JsValue> {
        match accessor.type_name.as_ref() {
            "SCALAR" => Ok(1),
            "VEC2" => Ok(2),
            "VEC3" => Ok(3),
            _ => Err(JsValue::from_str("accessor type not supported")),
        }
    }
}
