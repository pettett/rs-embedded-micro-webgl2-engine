#version 300 es

in vec4 vertexData; // <vec2 position, vec2 texCoords>

uniform float x;
uniform float y;

out vec2 texCoords;

void main() {
    gl_Position = vec4(vertexData.xy + vec2(x, y), 0.0, 1.0);

    texCoords = vertexData.zw;
}
