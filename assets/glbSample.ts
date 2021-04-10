import {compileFragmentShader, compileVertexShader, createProgram, resizeCanvas} from './utility/gl';
import init, {Scene} from './wasm/sdx_browser_game';

const foo = {
    keyPress: {
        w: false,
        a: false,
        s: false,
        d: false,
        mouse: false
    },
    mousePos: {
        x: 0,
        y: 0,
    },
    mouseMove: {
        left: false,
        right: false,
        up: false,
        down: false
    },
}

const mouseDown = (event: MouseEvent) => {
    foo.keyPress.mouse = true;
}

const mouseUp = (event: MouseEvent) => {
    foo.keyPress.mouse = false;
}

// const mouseMove = (event: MouseEvent, descriptor: MeshRenderDescriptor) => {
//     if (!foo.keyPress.mouse) {
//         return;
//     }
//
//     const moveLeft = foo.mousePos.x - event.clientX > 0;
//     const moveRight = foo.mousePos.x - event.clientX < 0;
//     const moveUp = foo.mousePos.y - event.clientY >= 0;
//     const moveDown = foo.mousePos.y - event.clientY >= 0;
//
//     foo.mouseMove.left = moveLeft;
//     foo.mouseMove.right = moveRight;
//     foo.mouseMove.up = moveUp;
//     foo.mouseMove.down = moveDown;
//
//     foo.mousePos.x = event.clientX;
//     foo.mousePos.y = event.clientY;
// }
//
// const keyDown = (event: KeyboardEvent, descriptor: MeshRenderDescriptor) => {
//     switch (event.key) {
//         case 'w':
//             foo.keyPress.w = true;
//             break;
//         case 'a':
//             foo.keyPress.a = true;
//             break;
//         case 's':
//             foo.keyPress.s = true;
//             break;
//         case 'd':
//             foo.keyPress.d = true;
//             break;
//     }
// }
//
// const keyUp = (event: KeyboardEvent, descriptor: MeshRenderDescriptor) => {
//     switch (event.key) {
//         case 'w':
//             foo.keyPress.w = false;
//             break;
//         case 'a':
//             foo.keyPress.a = false;
//             break;
//         case 's':
//             foo.keyPress.s = false;
//             break;
//         case 'd':
//             foo.keyPress.d = false;
//             break;
//     }
// }

const sampleRender = async (gl: WebGLRenderingContext, canvas: HTMLCanvasElement) => {
    await init();

    // let scene = await Scene.from_url(gl, canvas, '/models/test/suzanne2.glb');
    let scene = await Scene.from_url(gl, canvas, '/models/test/suzanne2.glb');

    const draw = () => {
        resizeCanvas(canvas);
        gl.viewport(0, 0, canvas.width, canvas.height);

        gl.clearColor(0.2, 0.2, 0.2, 1.0);
        gl.clear(gl.COLOR_BUFFER_BIT);

        gl.enable(gl.CULL_FACE);
        gl.enable(gl.DEPTH_TEST);

        scene.render(gl);
        // if (foo.keyPress.w) {
        //     descriptor.move_camera(0.0, 0.0, 0.1);
        // }
        // if (foo.keyPress.a) {
        //     descriptor.move_camera(0.1, 0.0, 0.0);
        // }
        // if (foo.keyPress.s) {
        //     descriptor.move_camera(0.0, 0.0, -0.1);
        // }
        // if (foo.keyPress.d) {
        //     descriptor.move_camera(-0.1, 0.0, 0.0);
        // }
        // if (foo.mouseMove.up) {
        //     descriptor.turn_x(0.1);
        // }
        // if (foo.mouseMove.down) {
        //     descriptor.turn_x(0.1);
        // }

        // console.log('left', foo.mouseMove.left);
        // console.log('right', foo.mouseMove.right);
        // console.log('up', foo.mouseMove.up);
        // console.log('down', foo.mouseMove.down);

        // console.log(gl, descriptor);
        // meshShader.render(gl, descriptor);
        // descriptor.rotate_sample();

        foo.mouseMove.left = false;
        foo.mouseMove.right = false;
        foo.mouseMove.up = false;
        foo.mouseMove.down = false;

        window.requestAnimationFrame(draw);
    }

    draw();
}

export default async function (): Promise<void> {
    // const url = '/models/test/suzanne1.glb'
    // const glb = await loadGlbFromUrl(url);

    const canvas = document.getElementById('surface');
    if (!(canvas instanceof HTMLCanvasElement)) {
        return;
    }
    const gl = canvas.getContext('webgl');
    if (!gl) {
        return;
    }

    // const mesh = glb.json.glTf.meshes[0];
    await sampleRender(gl, canvas);
    // await sampleRender2(mesh, glb, canvas, gl);
}