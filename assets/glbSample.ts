import {Glb, loadGlbFromUrl} from './loader/glb';
import {mat4} from 'gl-matrix';
import {compileFragmentShader, compileVertexShader, createProgram, resizeCanvas} from './utility/gl';
import {GlTfMesh} from './types/glTf';
import Camera from './Camera';
import vShader from './core/renderer/shaders/simple_v';
import fShader from './core/renderer/shaders/simple_f';
import MeshShader from "./core/renderer/MeshShader";

const sampleRender = async (mesh: GlTfMesh, glb: Glb, canvas: HTMLCanvasElement, gl: WebGLRenderingContext) => {
    const camera = Camera.new();
    camera.translate(0, 0, -7);

    const vertexShader = compileVertexShader(gl, vShader);
    const fragmentShader = compileFragmentShader(gl, fShader);
    const program = createProgram(gl, vertexShader, fragmentShader);

    const modelMatrix = mat4.create();
    const viewMatrix = camera.position();
    const projectionMatrix = createProjectionMatrix(canvas);

    const meshShader = await MeshShader.new(mesh.primitives[0], glb.buffer, glb.json.glTf, gl, program);

    const draw = () => {
        resizeCanvas(canvas);
        gl.viewport(0, 0, canvas.width, canvas.height);

        gl.clearColor(0.2, 0.2, 0.2, 1.0);
        gl.clear(gl.COLOR_BUFFER_BIT);

        gl.enable(gl.CULL_FACE);
        gl.enable(gl.DEPTH_TEST);

        mat4.rotateY(modelMatrix, modelMatrix, 0.03);

        meshShader.render(gl, modelMatrix,
            viewMatrix,
            projectionMatrix)

        window.requestAnimationFrame(draw);
    }

    draw();
}

const createProjectionMatrix = (canvas: HTMLCanvasElement): mat4 => {
    const projectionMatrix = mat4.create();
    const aspect = Math.abs(canvas.width / canvas.height);
    const degree = 60;
    const fov = (degree * Math.PI) / 180;

    mat4.perspective(projectionMatrix, fov, aspect, 1, 100.0);

    return projectionMatrix;
};

export default async function (): Promise<void> {
    const url = '/models/test/suzanne1.glb'
    const glb = await loadGlbFromUrl(url);

    const canvas = document.getElementById('surface');
    if (!(canvas instanceof HTMLCanvasElement)) {
        return;
    }
    const gl = canvas.getContext('webgl');
    if (!gl) {
        return;
    }

    const mesh = glb.json.glTf.meshes[0];
    await sampleRender(mesh, glb, canvas, gl);
}