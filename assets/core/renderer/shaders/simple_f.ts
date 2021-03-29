export default `#version 100

precision mediump float;

varying vec2 fTextureCoords;

uniform sampler2D uTexture;

void main() {
    // gl_FragColor = vec4(0.8, 0.0, 0.0, 1.0);
    gl_FragColor = texture2D(uTexture, fTextureCoords);
}`;