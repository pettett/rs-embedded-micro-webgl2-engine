#version 300 es

in vec3 position;
in vec3 normal;
in vec4 tangent;

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
out vec4 vTangent;
out vec4 vWorldPos; 
 
out vec3 fromFragmentToCamera;
 
out mat3 TBN;

void main (void) {
  vec4 worldPosition = model * vec4(position, 1.0);

  gl_Position = camera.projection * camera.view * worldPosition;

  vNormal = normal;
  vTangent = tangent;
  vWorldPos = worldPosition;
  fromFragmentToCamera = camera.pos.xyz - worldPosition.xyz;


   vec3 T = normalize(vec3(model * vec4(tangent.xyz,   0.0)));
   vec3 B = normalize(vec3(model * vec4(cross(normal, tangent.xyz) * tangent.w, 0.0)));
   vec3 N = normalize(vec3(model * vec4(normal,    0.0)));
   TBN = mat3(T, B, N);


  vUvs = uvs;


}
