import {mat4} from 'gl-matrix';
import {Glb} from '../../loader/glb';
import {GlTfAccessor, GlTfBufferView, GlTfMesh} from '../../types/glTf';

interface BufferDescriptor {
    buffer: WebGLBuffer;
    bufferView: GlTfBufferView;
    accessor: GlTfAccessor;
}

interface AttributeBufferDescriptor extends BufferDescriptor {
    location: number;
}

interface ProgramDescriptor {
    indexDescriptor: BufferDescriptor;
    positionsDescriptor: AttributeBufferDescriptor;
    normalsDescriptor?: AttributeBufferDescriptor;
    textureCoordsDescriptor?: AttributeBufferDescriptor;

    textures: {
        textureImage: TexImageSource;
        textureBuffer: WebGLTexture | null;
        textureLocation: WebGLUniformLocation | null;
    }[];
}

const createBufferViews = (accessor: GlTfAccessor, glb: Glb, gl: WebGLRenderingContext): [ArrayBufferView, GlTfBufferView] => {
    const bufferView = glb.json.glTf.bufferViews[accessor.bufferView];

    // Todo: double-check of accessor offset is needed here
    // const accessorOffset = accessor.byteOffset || 0;

    const byteOffset = glb.buffer.binStart + bufferView.byteOffset/* + accessorOffset*/;
    const length = bufferView.byteLength;

    switch (accessor.componentType) {
        case gl.FLOAT:
            return [new Float32Array(glb.buffer.glb, byteOffset, length), bufferView];
        case gl.UNSIGNED_SHORT:
            return [new Uint16Array(glb.buffer.glb, byteOffset, length), bufferView];
        default:
            throw new Error(`Component type (${accessor.componentType}) currently not supported`);
    }
}

const createBufferDescriptor = (bufferAccessor: GlTfAccessor, glb: Glb, gl: WebGLRenderingContext, target?: GLenum): BufferDescriptor => {
    const [bufferData, bufferView] = createBufferViews(bufferAccessor, glb, gl);
    const bufferTarget = target || gl.ELEMENT_ARRAY_BUFFER;

    const buffer = gl.createBuffer();
    if (!buffer) {
        throw new Error('Can not create webgl buffer');
    }
    gl.bindBuffer(bufferTarget, buffer);
    gl.bufferData(bufferTarget, bufferData, gl.STATIC_DRAW);

    return {
        buffer: buffer,
        bufferView: bufferView,
        accessor: bufferAccessor,
    };
}

const createAttributeBufferDescriptor = (bufferAccessor: GlTfAccessor, glb: Glb, gl: WebGLRenderingContext, program: WebGLProgram, locationName: string): AttributeBufferDescriptor => {
    let bufferDescriptor = createBufferDescriptor(bufferAccessor, glb, gl, gl.ARRAY_BUFFER);

    return {
        ...bufferDescriptor,
        location: gl.getAttribLocation(program, locationName)
    }
}

const loadHtmlImage = async (url: string): Promise<HTMLImageElement> => {
    return new Promise((resolve, reject) => {
        const image = new Image();
        image.addEventListener('load', () => resolve(image));
        image.addEventListener('error', reject);

        image.src = url;
    });
}

const sampleProgramDescriptor = async (mesh: GlTfMesh, glb: Glb, gl: WebGLRenderingContext, program: WebGLProgram): Promise<ProgramDescriptor[]> => {
    const descriptors: ProgramDescriptor[] = [];

    for (const primitive of mesh.primitives) {
        const indexDescriptor = createBufferDescriptor(glb.json.glTf.accessors[primitive.indices], glb, gl);
        const positionsDescriptor = createAttributeBufferDescriptor(glb.json.glTf.accessors[primitive.attributes.POSITION], glb, gl, program, 'position');

        const normalsDescriptor = primitive.attributes.NORMAL
            ? createAttributeBufferDescriptor(glb.json.glTf.accessors[primitive.attributes.NORMAL], glb, gl, program, 'normal')
            : undefined;

        const textureCoordsDescriptor = primitive.attributes.TEXCOORD_0
            ? createAttributeBufferDescriptor(glb.json.glTf.accessors[primitive.attributes.TEXCOORD_0], glb, gl, program, 'textureCoords')
            : undefined;

        const textures = [];

        if (primitive.material !== undefined) {
            const material = glb.json.glTf.materials[primitive.material];
            if (material.pbrMetallicRoughness.baseColorTexture) {
                const texture = glb.json.glTf.textures[material.pbrMetallicRoughness.baseColorTexture.index];
                const image = glb.json.glTf.images[texture.source];

                if (image.bufferView) {
                    const bufferView = glb.json.glTf.bufferViews[image.bufferView];

                    const byteOffset = glb.buffer.binStart + bufferView.byteOffset;
                    const length = bufferView.byteLength;

                    const array = new Uint8Array(glb.buffer.glb, byteOffset, length);

                    const blob = new Blob([array], {"type": "image/png"});
                    const objectURL = URL.createObjectURL(blob);
                    const textureImage = await loadHtmlImage(objectURL).catch(() => {
                        throw new Error('Could not load image from buffer view');
                    });

                    const textureBuffer = gl.createTexture();
                    gl.bindTexture(gl.TEXTURE_2D, textureBuffer);
                    gl.texImage2D(gl.TEXTURE_2D, 0, gl.RGBA, gl.RGBA, gl.UNSIGNED_BYTE, textureImage);
                    gl.generateMipmap(gl.TEXTURE_2D);

                    textures.push({
                        textureImage,
                        textureBuffer,
                        textureLocation: gl.getUniformLocation(program, "uTexture"),
                    });
                }
            }
        }

        descriptors.push({
            indexDescriptor,
            positionsDescriptor,
            normalsDescriptor,
            textureCoordsDescriptor,
            textures,
        });
    }

    return descriptors;
}

const accessorType2Size = (accessor: GlTfAccessor) => {
    switch (accessor.type) {
        case 'SCALAR':
            return 1;
        case 'VEC2':
            return 2;
        case 'VEC3':
            return 3;
        default:
            throw new Error(`Accessor type (${accessor.type}) not supported yet`);
    }
}

const enableAttribute = (gl: WebGLRenderingContext, descriptor: AttributeBufferDescriptor) => {
    gl.bindBuffer(gl.ARRAY_BUFFER, descriptor.buffer);
    gl.enableVertexAttribArray(descriptor.location);
    gl.vertexAttribPointer(
        descriptor.location,
        accessorType2Size(descriptor.accessor),
        descriptor.accessor.componentType,
        false,
        descriptor.bufferView.byteStride || 0,
        descriptor.accessor.byteOffset || 0,
    );
}

export interface SetupRenderDescriptor {
    mesh: GlTfMesh;
    glb: Glb;
    gl: WebGLRenderingContext;
    program: WebGLProgram;
}

const setupProgramDescriptor = async (descriptor: SetupRenderDescriptor): Promise<ProgramDescriptor[]> => {
    return await sampleProgramDescriptor(descriptor.mesh, descriptor.glb, descriptor.gl, descriptor.program);
}

export interface RenderDescriptor {
    gl: WebGLRenderingContext;
    program: WebGLProgram,
    programDescriptors: ProgramDescriptor[];

    modelMatrixLocation: WebGLUniformLocation | null;
    viewMatrixLocation: WebGLUniformLocation | null;
    projectionMatrixLocation: WebGLUniformLocation | null;


    modelMatrix: mat4;
    viewMatrix: mat4;
    projectionMatrix: mat4;
}

const render = (descriptor: RenderDescriptor) => {
    const gl = descriptor.gl;

    descriptor.programDescriptors.forEach((programDescriptor: ProgramDescriptor): void => {
        gl.useProgram(descriptor.program);

        enableAttribute(gl, programDescriptor.positionsDescriptor);
        if (programDescriptor.normalsDescriptor) {
            enableAttribute(gl, programDescriptor.normalsDescriptor);
        }
        if (programDescriptor.textureCoordsDescriptor) {
            enableAttribute(gl, programDescriptor.textureCoordsDescriptor);
        }

        gl.uniformMatrix4fv(descriptor.modelMatrixLocation, false, descriptor.modelMatrix);
        gl.uniformMatrix4fv(descriptor.viewMatrixLocation, false, descriptor.viewMatrix);
        gl.uniformMatrix4fv(descriptor.projectionMatrixLocation, false, descriptor.projectionMatrix);

        gl.clearColor(0.1, 0.1, 0.1, 1.0);
        gl.clear(gl.COLOR_BUFFER_BIT);

        gl.drawElements(
            gl.TRIANGLES,
            programDescriptor.indexDescriptor.accessor.count,
            programDescriptor.indexDescriptor.accessor.componentType,
            programDescriptor.indexDescriptor.accessor.byteOffset || 0
        );
    });
}

export {setupProgramDescriptor, render};