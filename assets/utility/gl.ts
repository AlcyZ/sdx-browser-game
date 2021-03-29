const compileVertexShader = (gl: WebGLRenderingContext, shaderSource: string): WebGLShader => {
	const shader = gl.createShader(gl.VERTEX_SHADER);
	if (!shader) {
		throw new Error('Could not create vertex shader');
	}
	
	gl.shaderSource(shader, shaderSource);
	gl.compileShader(shader);
	
	const success = gl.getShaderParameter(shader, gl.COMPILE_STATUS);
	if (!success) {
		throw `could not compile vertex shader: ${gl.getShaderInfoLog(shader)}`;
	}
	
	return shader;
}

const compileFragmentShader = (gl: WebGLRenderingContext, shaderSource: string): WebGLShader => {
	const shader = gl.createShader(gl.FRAGMENT_SHADER);
	if (!shader) {
		throw new Error('Could not create fragment shader');
	}
	
	gl.shaderSource(shader, shaderSource);
	gl.compileShader(shader);
	
	const success = gl.getShaderParameter(shader, gl.COMPILE_STATUS);
	if (!success) {
		throw `could not compile fragment shader: ${gl.getShaderInfoLog(shader)}`;
	}
	
	return shader;
}

const createProgram = (gl: WebGLRenderingContext, vertexShader: WebGLShader, fragmentShader: WebGLShader): WebGLProgram => {
	const program = gl.createProgram();
	if (!program) {
		throw new Error('Could not webgl program');
	}
	
	gl.attachShader(program, vertexShader);
	gl.attachShader(program, fragmentShader);
	
	gl.linkProgram(program);
	
	const success = gl.getProgramParameter(program, gl.LINK_STATUS);
	if (!success) {
		throw (`program failed to link: ${gl.getProgramInfoLog(program)}`);
	}
	
	return program;
}

const resizeCanvas = (canvas: HTMLCanvasElement): void => {
	const displayWidth = canvas.clientWidth;
	const displayHeight = canvas.clientHeight;
	
	const needResize = canvas.width !== displayWidth ||
		canvas.height !== displayHeight;
	
	if (needResize) {
		canvas.width = displayWidth;
		canvas.height = displayHeight;
	}
}

export {compileVertexShader, compileFragmentShader, createProgram, resizeCanvas};
