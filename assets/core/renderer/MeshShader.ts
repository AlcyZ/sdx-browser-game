import {
    MeshShaderFrameBuffer,
    MeshShaderFrameBuffers,
    MeshShaderLocations,
    MeshShaderTextures
} from './types/MeshShader';
import GlTf, {GlTfAccessor, GlTfMeshPrimitive} from '../../types/glTf';
import {GlbBuffer} from '../../loader/glb';
import {setupFrameBuffers, setupTextures, setupUniformLocations} from './MeshShaderUtility';
import {mat4} from "gl-matrix";

const accessorType2Size = (accessor: GlTfAccessor): number => {
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

const enableAttribute = (gl: WebGLRenderingContext, frameBuffer: MeshShaderFrameBuffer, location: number): void => {
    gl.bindBuffer(gl.ARRAY_BUFFER, frameBuffer.buffer);
    gl.enableVertexAttribArray(location);
    gl.vertexAttribPointer(
        location,
        accessorType2Size(frameBuffer.accessor),
        frameBuffer.accessor.componentType,
        false,
        frameBuffer.bufferView.byteStride || 0,
        frameBuffer.accessor.byteOffset || 0,
    );
}

export default class MeshShader {
    private constructor(
        private readonly locations: MeshShaderLocations,
        private readonly frameBuffers: MeshShaderFrameBuffers,
        private readonly textures: MeshShaderTextures,
        private readonly program: WebGLProgram
    ) {
    }

    static async new(primitive: GlTfMeshPrimitive, glbBuffer: GlbBuffer, glTf: GlTf, gl: WebGLRenderingContext, program: WebGLProgram): Promise<MeshShader> {
        return new MeshShader(
            setupUniformLocations(gl, program),
            setupFrameBuffers(primitive, glTf, gl, glbBuffer),
            await setupTextures(primitive, glTf, gl, glbBuffer, program),
            program
        );
    }


    render(gl: WebGLRenderingContext, modelMatrix: mat4, viewMatrix: mat4, projectionMatrix: mat4) {
        gl.useProgram(this.program);

        enableAttribute(gl, this.frameBuffers.position, this.locations.attribute.position)
        if (this.frameBuffers.normal) {
            enableAttribute(gl, this.frameBuffers.normal, this.locations.attribute.normal)
        }
        if (this.frameBuffers.textureCoord) {
            enableAttribute(gl, this.frameBuffers.textureCoord, this.locations.attribute.textureCoord)
        }

        gl.uniformMatrix4fv(this.locations.uniform.modelMatrix, false, modelMatrix);
        gl.uniformMatrix4fv(this.locations.uniform.viewMatrix, false, viewMatrix);
        gl.uniformMatrix4fv(this.locations.uniform.projectionMatrix, false, projectionMatrix);

        gl.drawElements(
            gl.TRIANGLES,
            this.frameBuffers.index.accessor.count,
            this.frameBuffers.index.accessor.componentType,
            this.frameBuffers.index.accessor.byteOffset || 0
        );
    }
}
