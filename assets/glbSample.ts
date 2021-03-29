import {Glb, loadGlbFromUrl} from './loader/glb';
import {mat4} from 'gl-matrix';
import {resizeCanvas} from './utility/gl';
import {GlTfMesh} from './types/glTf';
import Camera from './Camera';
import {renderMesh, RenderMeshDescriptor} from './core/renderer/mesh';

const sampleRender = (mesh: GlTfMesh, glb: Glb, canvas: HTMLCanvasElement, gl: WebGLRenderingContext) => {
    const camera = Camera.new();
    camera.translate(0, 0, -7);

    const viewMatrix = camera.position();
    const projectionMatrix = createProjectionMatrix(canvas);

    let posX = 0;
    let time = 0;


    const draw = (): void => {
        resizeCanvas(canvas);
        gl.viewport(0, 0, canvas.width, canvas.height);

        const position = mat4.create();
        mat4.translate(position, position, [posX, 0, 0])

        const renderMeshDescriptor: RenderMeshDescriptor = {
            gl,
            glb,
            mesh,
            viewMatrix,
            projectionMatrix
        }
        renderMesh(renderMeshDescriptor, position);

        time += 1 / 60;
        posX = Math.sin(time) * 7;

        // window.requestAnimationFrame(draw);
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
    const url = '/models/test/suzanne.glb'
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
    sampleRender(mesh, glb, canvas, gl);
}