use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct TextureReference {
    pub(crate) index: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct GlTfScene {
    pub(crate) nodes: Vec<usize>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct GlTfNode {
    pub(crate) mesh: Vec<usize>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct GlTfPbrMetallicRoughness {
    #[serde(rename = "baseColorFactor")]
    pub(crate) base_color_factor: Option<[f32; 4]>,
    #[serde(rename = "metallicFactor")]
    pub(crate) metallic_factor: Option<f32>,
    #[serde(rename = "roughnessFactor")]
    pub(crate) roughness_factor: Option<f32>,
    #[serde(rename = "baseColorTexture")]
    pub(crate) base_color_texture: Option<TextureReference>,
    #[serde(rename = "metallicTexture")]
    pub(crate) metallic_texture: Option<TextureReference>,
    #[serde(rename = "roughnessTexture")]
    pub(crate) roughness_texture: Option<TextureReference>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct GlTfMaterial {
    #[serde(rename = "pbrMetallicRoughness")]
    pub(crate) pbr_metallic_roughness: GlTfPbrMetallicRoughness,
    pub(crate) name: Option<String>,
    #[serde(rename = "doubleSided")]
    pub(crate) double_sided: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct GlTfMeshPrimitiveAttributes {
    #[serde(rename = "POSITION")]
    pub(crate) position: usize,
    #[serde(rename = "NORMAL")]
    pub(crate) normal: Option<usize>,
    #[serde(rename = "TEXCOORD_0")]
    pub(crate) texture_coord_0: Option<usize>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct GlTfMeshPrimitive {
    pub(crate) attributes: GlTfMeshPrimitiveAttributes,
    pub(crate) indices: usize,
    pub(crate) material: Option<usize>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct GlTfMesh {
    pub(crate) primitives: Vec<GlTfMeshPrimitive>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct GlTfTexture {
    pub(crate) sampler: usize,
    pub(crate) source: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct GlTfImage {
    pub(crate) uri: Option<String>,
    #[serde(rename = "bufferView")]
    pub(crate) buffer_view: Option<usize>,
    #[serde(rename = "mimeType")]
    pub(crate) mime_type: Option<String>,
    pub(crate) name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct GlTfBuffer {
    #[serde(rename = "byteLength")]
    pub(crate) byte_length: u32,
    pub(crate) uri: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct GlTfBufferView {
    pub(crate) buffer: usize,
    #[serde(rename = "byteOffset")]
    pub(crate) byte_offset: u32,
    #[serde(rename = "byteLength")]
    pub(crate) byte_length: u32,
    pub(crate) target: Option<usize>,
    #[serde(rename = "byteStride")]
    pub(crate) byte_stride: Option<usize>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct GlTfSampler {
    #[serde(rename = "magFilter")]
    pub(crate) mag_filter: usize,
    #[serde(rename = "minFilter")]
    pub(crate) min_filter: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct GlTfAccessor {
    #[serde(rename = "bufferView")]
    pub(crate) buffer_view: usize,
    #[serde(rename = "componentType")]
    pub(crate) component_type: u32,
    pub(crate) count: usize,
    #[serde(rename = "type")]
    pub(crate) type_name: String,
    #[serde(rename = "byteOffset")]
    pub(crate) byte_offset: Option<u32>,
    pub(crate) max: Option<Vec<f32>>,
    pub(crate) min: Option<Vec<f32>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct GlTfAsset {
    pub(crate) version: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct GlTf {
    // scenes: Vec<GlTfScene>,
    // nodes: Vec<GlTfNode>,
    pub(crate) materials: Vec<GlTfMaterial>,
    pub(crate) meshes: Vec<GlTfMesh>,
    pub(crate) textures: Vec<GlTfTexture>,
    pub(crate) images: Vec<GlTfImage>,
    pub(crate) accessors: Vec<GlTfAccessor>,
    pub(crate) buffers: Vec<GlTfBuffer>,
    #[serde(rename = "bufferViews")]
    pub(crate) buffer_views: Vec<GlTfBufferView>,
    pub(crate) samplers: Vec<GlTfSampler>,
    pub(crate) asset: GlTfAsset,
}
