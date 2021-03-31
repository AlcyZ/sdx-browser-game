use web_sys::{WebGlProgram, WebGlRenderingContext, WebGlUniformLocation};

#[derive(Debug)]
pub(super) struct MeshUniformLocations {
    pub(super) model_matrix: Option<WebGlUniformLocation>,
    pub(super) view_matrix: Option<WebGlUniformLocation>,
    pub(super) projection_matrix: Option<WebGlUniformLocation>,
}

#[derive(Debug)]
pub(super) struct MeshAttributeLocations {
    pub(super) position: i32,
    pub(super) normal: i32,
    pub(super) texture_coord: i32,
}

#[derive(Debug)]
pub(super) struct MeshShaderLocations {
    pub(super) uniform: MeshUniformLocations,
    pub(super) attribute: MeshAttributeLocations,
}

impl MeshShaderLocations {
    pub(super) fn new(gl: &WebGlRenderingContext, program: &WebGlProgram) -> MeshShaderLocations {
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
