import {
    MeshShaderFrameBuffer,
    MeshShaderFrameBuffers,
    MeshShaderLocations,
    MeshShaderTexture,
    MeshShaderTextures
} from './types/MeshShader';
import {GlbBuffer} from '../../loader/glb';
import GlTf, {GlTfAccessor, GlTfBufferView, GlTfMeshPrimitive} from '../../types/glTf';

const setupUniformLocations = (gl: WebGLRenderingContext, program: WebGLProgram): MeshShaderLocations => {
    return {
        uniform: {
            modelMatrix: gl.getUniformLocation(program, 'modelMatrix'),
            viewMatrix: gl.getUniformLocation(program, 'viewMatrix'),
            projectionMatrix: gl.getUniformLocation(program, 'projectionMatrix'),
        },
        attribute: {
            position: gl.getAttribLocation(program, 'position'),
            normal: gl.getAttribLocation(program, 'normal'),
            textureCoord: gl.getAttribLocation(program, 'textureCoords'),
        }
    };
}

const createBufferView = (gl: WebGLRenderingContext, glbBuffer: GlbBuffer, glTf: GlTf, accessor: GlTfAccessor): [ArrayBufferView, GlTfBufferView] => {
    const bufferView = glTf.bufferViews[accessor.bufferView];

    const byteOffset = glbBuffer.binStart + bufferView.byteOffset;
    const length = bufferView.byteLength;

    switch (accessor.componentType) {
        case gl.FLOAT:
            return [new Float32Array(glbBuffer.glb, byteOffset, length), bufferView];
        case gl.UNSIGNED_SHORT:
            return [new Uint16Array(glbBuffer.glb, byteOffset, length), bufferView];
        default:
            throw new Error(`Component type (${accessor.componentType}) currently not supported`);
    }
}

const createWebGLBuffer = (gl: WebGLRenderingContext): WebGLBuffer => {
    const buffer = gl.createBuffer();
    if (!buffer) {
        throw new Error('Can not create webgl buffer');
    }

    return buffer;
}

const setupAttributeBuffer = (gl: WebGLRenderingContext, glbBuffer: GlbBuffer, glTf: GlTf, accessor: GlTfAccessor): MeshShaderFrameBuffer => {
    const [bufferData, bufferView] = createBufferView(gl, glbBuffer, glTf, accessor);
    const buffer = createWebGLBuffer(gl);
    gl.bindBuffer(gl.ARRAY_BUFFER, buffer);
    gl.bufferData(gl.ARRAY_BUFFER, bufferData, gl.STATIC_DRAW);

    return {
        buffer,
        bufferView,
        accessor,
    };
}

const setupIndexBuffer = (gl: WebGLRenderingContext, glbBuffer: GlbBuffer, glTf: GlTf, accessor: GlTfAccessor): MeshShaderFrameBuffer => {
    const [bufferData, bufferView] = createBufferView(gl, glbBuffer, glTf, accessor);
    const buffer = createWebGLBuffer(gl);
    gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, buffer);
    gl.bufferData(gl.ELEMENT_ARRAY_BUFFER, bufferData, gl.STATIC_DRAW);

    return {
        buffer,
        bufferView,
        accessor,
    };
}

const setupFrameBuffers = (primitive: GlTfMeshPrimitive, glTf: GlTf, gl: WebGLRenderingContext, glbBuffer: GlbBuffer): MeshShaderFrameBuffers => {
    const index = setupIndexBuffer(gl, glbBuffer, glTf, glTf.accessors[primitive.indices]);
    const position = setupAttributeBuffer(gl, glbBuffer, glTf, glTf.accessors[primitive.attributes.POSITION]);
    const normal = primitive.attributes.NORMAL !== undefined
        ? setupAttributeBuffer(gl, glbBuffer, glTf, glTf.accessors[primitive.attributes.NORMAL])
        : undefined;
    const textureCoord = primitive.attributes.TEXCOORD_0 !== undefined
        ? setupAttributeBuffer(gl, glbBuffer, glTf, glTf.accessors[primitive.attributes.TEXCOORD_0])
        : undefined;

    return {
        index,
        position,
        normal,
        textureCoord,
    };
}

const loadHtmlImage = async (url: string): Promise<HTMLImageElement> => {
    return new Promise((resolve, reject) => {
        const image = new Image();
        image.addEventListener('load', () => resolve(image));
        image.addEventListener('error', reject);

        image.src = url;
    });
}

const setupTextures = async (
    primitive: GlTfMeshPrimitive,
    glTf: GlTf, gl: WebGLRenderingContext,
    glbBuffer: GlbBuffer,
    program: WebGLProgram
): Promise<MeshShaderTextures> => {
    let baseColor: MeshShaderTexture | undefined = undefined;

    if (primitive.material !== undefined) {
        const material = glTf.materials[primitive.material];

        if (material.pbrMetallicRoughness.baseColorTexture) {
            const texture = glTf.textures[material.pbrMetallicRoughness.baseColorTexture.index];
            const image = glTf.images[texture.source];

            if (image.bufferView) {
                const bufferView = glTf.bufferViews[image.bufferView];

                const byteOffset = glbBuffer.binStart + bufferView.byteOffset;
                const length = bufferView.byteLength;

                const array = new Uint8Array(glbBuffer.glb, byteOffset, length);

                const blob = new Blob([array], {"type": "image/png"});
                const objectURL = URL.createObjectURL(blob);
                const textureImage = await loadHtmlImage(objectURL).catch(() => {
                    throw new Error('Could not load image from buffer view');
                });

                const textureBuffer = gl.createTexture();
                gl.bindTexture(gl.TEXTURE_2D, textureBuffer);
                gl.texImage2D(gl.TEXTURE_2D, 0, gl.RGBA, gl.RGBA, gl.UNSIGNED_BYTE, textureImage);
                gl.generateMipmap(gl.TEXTURE_2D);

                baseColor = {
                    buffer: textureBuffer,
                    image: textureImage,
                    locations: gl.getUniformLocation(program, 'uTexture')
                }
            }
        }
    }

    return {
        baseColor
    }
}

export {
    setupUniformLocations,
    createBufferView,
    createWebGLBuffer,
    setupAttributeBuffer,
    setupIndexBuffer,
    setupFrameBuffers,
    loadHtmlImage,
    setupTextures,
}

