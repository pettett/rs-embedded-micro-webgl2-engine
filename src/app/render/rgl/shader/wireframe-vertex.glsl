#version 300 es

in vec3 position; 
 

uniform mat4 model; 


layout(std140) uniform Camera
{
  mat4 projection;
  mat4 view;
  vec4 pos;
} camera;
 
out vec4 vWorldPos; 
 
out vec3 fromFragmentToCamera;
  

void main (void) {
  vec4 worldPosition = model * vec4(position, 1.0);

  gl_Position = camera.projection * camera.view * worldPosition;
 
  vWorldPos = worldPosition;
  fromFragmentToCamera = camera.pos.xyz - worldPosition.xyz;

  
}
