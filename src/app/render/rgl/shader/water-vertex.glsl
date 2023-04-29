#version 300 es

in vec2 position;
 
uniform mat4 model; 

layout(std140) uniform Camera
{
  mat4 projection;
  mat4 view;
  vec4 pos;
} camera;

 
out vec3 fromFragmentToCamera;

out vec4 clipSpace;
out vec2 textureCoords;

const float tiling = 4.0;

void main() {
    vec4 worldPosition = model * vec4(position.x, 0.0, position.y, 1.0);

    clipSpace = camera.projection * camera.view *  worldPosition;

    gl_Position = clipSpace;

    // (-0.5 < pos < 0.5) -> (0.0 < pos < 1.0)
    textureCoords = position + 0.5;
    textureCoords = textureCoords * tiling;

    fromFragmentToCamera = camera.pos.xyz - worldPosition.xyz;
}
