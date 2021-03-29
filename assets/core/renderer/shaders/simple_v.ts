export default `#version 100

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