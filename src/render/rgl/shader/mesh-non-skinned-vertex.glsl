#version 300 es

in vec3 position;
in vec3 normal;

in vec2 uvs;
out vec2 vUvs;

uniform mat4 model; 


layout(std140) uniform Camera
{
  mat4 projection;
  mat4 view;
  vec4 pos;
} camera;

out vec3 vNormal;
out vec3 vWorldPos;
out vec4 worldPosition;
 
out vec3 fromFragmentToCamera;

void main (void) {
  worldPosition = model * vec4(position, 1.0);

  gl_Position = camera.projection * camera.view * worldPosition;

  vNormal = normal;
  vWorldPos = worldPosition.xyz;
  fromFragmentToCamera = camera.pos.xyz - worldPosition.xyz;

  vUvs = uvs;
}
