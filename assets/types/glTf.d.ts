type GlTfVersion = '2.0';
type vec4 = [number, number, number, number];
export type TextureReference = {
    index: number;
}

export interface GlTfScene {
    nodes: number[];
}

export interface GlTfNode {
    mesh: number[];
}



export interface GlTfMaterials {
    pbrMetallicRoughness: {
        baseColorFactor?: vec4;
        metallicFactor?: number;
        roughnessFactor?: number;
        baseColorTexture?: TextureReference;
        metallicTexture?: TextureReference;
        roughnessTexture?: TextureReference;
    };
    name?: string;
    doubleSided?: boolean;
}

export interface GlTfMeshPrimitive {
    attributes: {
        POSITION: number;
        NORMAL?: number;
        TEXCOORD_0?: number;
    },
    indices: number;
    material?: number;
}

export interface GlTfMesh {
    primitives: GlTfMeshPrimitive[]
}

export interface GlTfTexture {
    sampler: number;
    source: number;
}

export interface GlTfImage {
    uri?: string;
    bufferView?: number;
    mimeType?: string;
    name?: string;
}

export interface GlTfBuffer {
    byteLength: number;
    uri?: string;
}

export interface GlTfBufferView {
    buffer: number;
    byteOffset: number;
    byteLength: number;
    target: number;
    byteStride?: number;
}

export interface GlTfSampler {
    magFilter: number;
    minFilter: number;
}

export interface GlTfAccessor {
    bufferView: number;
    componentType: number;
    count: number;
    type: string;
    byteOffset?: number;
    max?: number[];
    min?: number[];
}

export interface GlTfAsset {
    version: GlTfVersion;
}

export default interface GlTf {
    scenes: GlTfScene[];
    nodes: GlTfNode[];
    materials: GlTfMaterials[];
    meshes: GlTfMesh[];
    textures: GlTfTexture[];
    images: GlTfImage[];
    accessors: GlTfAccessor[];
    buffers: GlTfBuffer[];
    bufferViews: GlTfBufferView[];
    samplers: GlTfSampler[];
    asset: GlTfAsset;

    // any else arbitrary value
    [index: string]: any;
}
