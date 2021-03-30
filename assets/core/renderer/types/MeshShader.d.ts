import {GlTfAccessor, GlTfBufferView} from "../../../types/glTf";

export interface MeshUniformLocations {
    modelMatrix: WebGLUniformLocation | null;
    viewMatrix: WebGLUniformLocation | null;
    projectionMatrix: WebGLUniformLocation | null;
}

export interface MeshAttributeLocations {
    position: number;
    normal: number;
    textureCoord: number;
}

export interface MeshShaderLocations {
    uniform: MeshUniformLocations;
    attribute: MeshAttributeLocations;
}

export interface MeshShaderFrameBuffer {
    buffer: WebGLBuffer;
    bufferView: GlTfBufferView;
    accessor: GlTfAccessor;
}

export interface MeshShaderFrameBuffers {
    index: MeshShaderFrameBuffer;
    position: MeshShaderFrameBuffer;
    normal?: MeshShaderFrameBuffer;
    textureCoord?: MeshShaderFrameBuffer;
}

export interface MeshShaderTexture {
    image: TexImageSource;
    buffer: WebGLTexture | null;
    locations: WebGLUniformLocation | null;
}

export interface MeshShaderTextures {
    baseColor?: MeshShaderTexture
}
