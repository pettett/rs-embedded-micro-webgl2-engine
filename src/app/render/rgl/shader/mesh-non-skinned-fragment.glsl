#version 300 es

precision mediump float;

in vec3 vNormal;
in vec4 vTangent;
in vec4 vWorldPos;

in vec2 vUvs;

in vec3 fromFragmentToCamera;
 
in mat3 TBN;

uniform vec4 clipPlane;

in vec3 sunlightDirTangent;

float shininess = 0.4;

vec3 sunlightColor = vec3(1.0, 1.0, 1.0);
vec3 sunlightDir = normalize(vec3(-1.0, -1.0, 0.5));

uniform sampler2D meshTexture;
uniform sampler2D meshNormal;

out vec4 fragColor;

 


void main(void) {


    vec4 textureColor = texture(meshTexture, vUvs);


    if (dot(vWorldPos, clipPlane) < 0.0 || textureColor.a < 0.9) {
        discard;
    }

    vec3 textureNormal = normalize(texture(meshNormal, vUvs).xyz * 2.0 - 1.0);
    vec3 normal = TBN * textureNormal.xyz;

    vec3 ambient = vec3(0.24725, 0.1995, 0.0745);

    float diff = max(dot(normal, -sunlightDir), 0.0);
    vec3 diffuse = diff * sunlightColor;

    vec3 reflectDir = reflect(-sunlightDir, normal);
    float spec = pow(max(dot(normalize(fromFragmentToCamera), reflectDir), 0.0), 32.0);
    vec3 specular = shininess * spec * vec3(0.628281, 0.555802, 0.366065);

    vec4 lighting = vec4(ambient + diffuse + specular, 1.0);

    fragColor = textureColor * lighting;

}
