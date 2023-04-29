#version 300 es

precision mediump float;

in vec2 texCoords;

uniform sampler2D u_texture;
out vec4 fragColor;
void main() {
    fragColor = texture( u_texture, vec2(texCoords.s, texCoords.t) );
}
