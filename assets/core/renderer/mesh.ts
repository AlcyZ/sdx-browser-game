import {mat4} from 'gl-matrix';
import {Glb} from '../../loader/glb';
import {compileVertexShader, compileFragmentShader, createProgram} from '../../utility/gl';
import {GlTfAccessor, GlTfBufferView, GlTfMesh, GlTfMeshPrimitive} from '../../types/glTf';

const vShader = `#version 100

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
}`;

const fShader = `#version 100

precision mediump float;

void main() {

    gl_FragColor = vec4(0.8, 0.0, 0.0, 1.0);
}`;

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

const sliceArrayBuffer = (arrayBuffer: ArrayBufferLike, byteOffset: number, byteLength: number) => {
    const subArray = new Uint8Array(arrayBuffer).subarray(byteOffset, byteOffset + byteLength);
    const arrayCopy = new Uint8Array(subArray);
    return arrayCopy;
}

const sampleProgramDescriptor = (mesh: GlTfMesh, glb: Glb, gl: WebGLRenderingContext, program: WebGLProgram): ProgramDescriptor[] => {
    const descriptors: ProgramDescriptor[] = [];

    mesh.primitives.forEach((primitive: GlTfMeshPrimitive): void => {
        // INDEX BUFFER
        const indexDescriptor = createBufferDescriptor(glb.json.glTf.accessors[primitive.indices], glb, gl);
        const positionsDescriptor = createAttributeBufferDescriptor(glb.json.glTf.accessors[primitive.attributes.POSITION], glb, gl, program, 'position');

        const normalsDescriptor = primitive.attributes.NORMAL
            ? createAttributeBufferDescriptor(glb.json.glTf.accessors[primitive.attributes.NORMAL], glb, gl, program, 'normal')
            : undefined;

        const textureCoordsDescriptor = primitive.attributes.TEXCOORD_0
            ? createAttributeBufferDescriptor(glb.json.glTf.accessors[primitive.attributes.TEXCOORD_0], glb, gl, program, 'textureCoords')
            : undefined;

        if (primitive.material !== undefined) {
            const material = glb.json.glTf.materials[primitive.material];
            if (material.pbrMetallicRoughness.baseColorTexture) {
                const texture = glb.json.glTf.textures[material.pbrMetallicRoughness.baseColorTexture.index];
                const image = glb.json.glTf.images[texture.source];

                if (image.bufferView) {
                    const bufferView = glb.json.glTf.bufferViews[image.bufferView];

                    const byteOffset = glb.buffer.binStart + bufferView.byteOffset;
                    const length = bufferView.byteLength;

                    // const textureData = new Uint8ClampedArray(40000);
                    // // Iterate through every pixel
                    // for (let i = 0; i < textureData.length; i += 4) {
                    //     textureData[i + 0] = 0;    // R value
                    //     textureData[i + 1] = 190;  // G value
                    //     textureData[i + 2] = 0;    // B value
                    //     textureData[i + 3] = 255;  // A value
                    // }
                    // const imageData = new ImageData(textureData, 200); // Todo: Fix hardcoded res

                    // const textureData = new Uint8Array(glb.buffer.glb, byteOffset, length);
                    // const asd = sliceArrayBuffer(textureData.buffer, textureData.byteOffset, textureData.byteLength);
                    //
                    // console.log(asd, textureData.byteOffset, asd.byteOffset);
                    //
                    //
                    // const glTexture = gl.createTexture();
                    // gl.bindTexture(gl.TEXTURE_2D, glTexture);
                    // gl.texImage2D(gl.TEXTURE_2D, 0, gl.RGBA, 1024, 886, 0, gl.RGBA, gl.UNSIGNED_BYTE, asd);
                    // gl.texImage2D(gl.TEXTURE_2D, 0, gl.RGBA, gl.RGBA,gl.UNSIGNED_BYTE, imageData);

                    // alert('image data loaded');
                }
            }
        }


        descriptors.push({
            indexDescriptor,
            positionsDescriptor,
            normalsDescriptor,
            textureCoordsDescriptor,
        });
    });

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

export interface RenderMeshDescriptor {
    mesh: GlTfMesh;
    glb: Glb;
    gl: WebGLRenderingContext;
    viewMatrix: mat4;
    projectionMatrix: mat4;
}

const renderMesh = (descriptor: RenderMeshDescriptor, translationMatrix?: mat4) => {
    const gl = descriptor.gl;
    const modelMatrix = translationMatrix ? translationMatrix : mat4.create();

    // Todo: Shader compiling should be done once
    const vertexShader = compileVertexShader(gl, vShader);
    const fragmentShader = compileFragmentShader(gl, fShader);
    const program = createProgram(gl, vertexShader, fragmentShader);

    const modelMatrixLocation = gl.getUniformLocation(program, 'modelMatrix');
    const viewMatrixLocation = gl.getUniformLocation(program, 'viewMatrix');
    const projectionMatrixLocation = gl.getUniformLocation(program, 'projectionMatrix');

    const descriptors = sampleProgramDescriptor(descriptor.mesh, descriptor.glb, gl, program);

    const texture = gl.createTexture();
    gl.bindTexture(gl.TEXTURE_2D, texture);

    descriptors.forEach((programDescriptor: ProgramDescriptor): void => {
        gl.useProgram(program);

        enableAttribute(gl, programDescriptor.positionsDescriptor);
        if (programDescriptor.normalsDescriptor) {
            enableAttribute(gl, programDescriptor.normalsDescriptor);
        }
        if (programDescriptor.textureCoordsDescriptor) {
            enableAttribute(gl, programDescriptor.textureCoordsDescriptor);
        }

        gl.uniformMatrix4fv(modelMatrixLocation, false, modelMatrix);
        gl.uniformMatrix4fv(viewMatrixLocation, false, descriptor.viewMatrix);
        gl.uniformMatrix4fv(projectionMatrixLocation, false, descriptor.projectionMatrix);

        gl.drawElements(
            gl.TRIANGLES,
            programDescriptor.indexDescriptor.accessor.count,
            programDescriptor.indexDescriptor.accessor.componentType,
            programDescriptor.indexDescriptor.accessor.byteOffset || 0
        );
    });
}

export {renderMesh};