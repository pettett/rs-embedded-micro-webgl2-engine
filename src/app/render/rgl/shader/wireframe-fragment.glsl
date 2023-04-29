#version 300 es

precision mediump float;
 
in vec4 vWorldPos;
 
in vec3 fromFragmentToCamera;
  

uniform vec4 clipPlane;
 
out vec4 fragColor;


void main(void) {


    if (dot(vWorldPos, clipPlane) < 0.0  ) {
        discard;
    }
  

    fragColor = vec4(1,0,0,1);

}
