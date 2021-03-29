import {Glb, loadGlbFromUrl} from './loader/glb';
import {mat4} from 'gl-matrix';
import {compileFragmentShader, compileVertexShader, createProgram, resizeCanvas} from './utility/gl';
import {GlTfMesh} from './types/glTf';
import Camera from './Camera';
import {render, RenderDescriptor, setupProgramDescriptor} from './core/renderer/meshSample';
import vShader from './core/renderer/shaders/simple_v';
import fShader from './core/renderer/shaders/simple_f';

const sampleRender = async (mesh: GlTfMesh, glb: Glb, canvas: HTMLCanvasElement, gl: WebGLRenderingContext) => {
    const camera = Camera.new();
    camera.translate(0, 0, -7);

    const vertexShader = compileVertexShader(gl, vShader);
    const fragmentShader = compileFragmentShader(gl, fShader);
    const program = createProgram(gl, vertexShader, fragmentShader);

    const viewMatrix = camera.position();
    const projectionMatrix = createProjectionMatrix(canvas);

    const setupRenderDescriptor = {
        mesh,
        glb,
        gl,
        program
    };
    const programDescriptors = await setupProgramDescriptor(setupRenderDescriptor);
    const modelMatrixLocation = gl.getUniformLocation(program, 'modelMatrix');
    const viewMatrixLocation = gl.getUniformLocation(program, 'viewMatrix');
    const projectionMatrixLocation = gl.getUniformLocation(program, 'projectionMatrix');

    let time = 0;

    const modelMatrix = mat4.create();

    const renderDescriptors: RenderDescriptor = {
        gl,
        modelMatrix,
        modelMatrixLocation,
        program,
        programDescriptors,
        projectionMatrix,
        projectionMatrixLocation,
        viewMatrix,
        viewMatrixLocation

    };

    const draw = (): void => {
        resizeCanvas(canvas);
        gl.viewport(0, 0, canvas.width, canvas.height);

        gl.enable(gl.CULL_FACE);
        gl.enable(gl.DEPTH_TEST);

        mat4.rotateY(modelMatrix, modelMatrix, 0.03);

        const renderDescriptor: RenderDescriptor = {
            ...renderDescriptors,
            modelMatrix,
        };
        render(renderDescriptor);

        window.requestAnimationFrame(draw);

        time += 1 / 60;
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
    sampleRender(mesh, glb, canvas, gl);
}