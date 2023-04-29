#version 300 es

precision mediump float;

uniform sampler2D refractionTexture;
uniform sampler2D reflectionTexture;
uniform sampler2D dudvTexture;
uniform sampler2D normalMap;
uniform sampler2D waterDepthTexture;

vec3 sunlightColor = vec3(1.0, 1.0, 1.0);
vec3 sunlightDir = normalize(vec3(-1.0, -1.0, 0.5));

in vec3 fromFragmentToCamera;

// Changes over time, making the water look like it's moving
uniform float dudvOffset;

in vec4 clipSpace;

in vec2 textureCoords;

const float waterDistortionStrength = 0.03;
const float shineDamper = 20.0;

uniform float waterReflectivity;
uniform float fresnelStrength;

vec4 shallowWaterColor =  vec4(0.0, 0.1, 0.3, 1.0);
vec4 deepWaterColor = vec4(0.0, 0.1, 0.2, 1.0);

vec3 getNormal(vec2 textureCoords);

out vec4 fragColor;

void main() {
    // Normalized device coordinates - Between 0 and 1
    vec2 ndc = (clipSpace.xy / clipSpace.w) / 2.0 + 0.5;

    vec2 refractTexCoords = vec2(ndc.x, ndc.y);
    // Reflections are upside down
    vec2 reflectTexCoords = vec2(ndc.x, -ndc.y);

    float near = 0.1;
    float far = 50.0;

    // Get the distance from our camera to the first thing under this water fragment that a
    // ray would collide with. This might be the ground, the under water walls, a fish, or any
    // other thing under the water. This distance will depend on our camera angle.
    float cameraToFirstThingBehindWater = texture(waterDepthTexture, refractTexCoords).r;
    // Convert from our perspective transformed distance to our world distance
    float cameraToFirstThingUnderWater = 2.0 * near * far /
     (far + near - (2.0 * cameraToFirstThingBehindWater - 1.0)
      * (far - near));

    float cameraToWaterDepth = gl_FragCoord.z;
    float cameraToWaterDistance = 2.0 * near * far / (far + near - (2.0 * cameraToWaterDepth - 1.0) * (far - near));

    float angledWaterDepth = cameraToFirstThingUnderWater - cameraToWaterDistance;

    vec2 distortedTexCoords = texture(dudvTexture, vec2(textureCoords.x + dudvOffset, textureCoords.y)).rg * 0.1;
    distortedTexCoords = textureCoords + vec2(distortedTexCoords.x, distortedTexCoords.y + dudvOffset);

    // Between -1 and 1
    vec2 totalDistortion = (texture(dudvTexture, distortedTexCoords).rg * 2.0 - 1.0)
     * waterDistortionStrength;

    refractTexCoords += totalDistortion;
    reflectTexCoords += totalDistortion;

    // Prevent out distortions from sampling from the opposite side of the texture
    // NOTE: This will still cause artifacts towards the edges of the water. You can fix this by
    // making the water more transparent at the edges.
    // @see https://www.youtube.com/watch?v=qgDPSnZPGMA
    refractTexCoords = clamp(refractTexCoords, 0.001, 0.999);
    reflectTexCoords.x = clamp(reflectTexCoords.x, 0.001, 0.999);
    reflectTexCoords.y = clamp(reflectTexCoords.y, -0.999, -0.001);

    vec4 reflectColor = texture(reflectionTexture, reflectTexCoords);

    vec4 refractColor = texture(refractionTexture, refractTexCoords);

    refractColor = mix(refractColor, deepWaterColor, clamp(angledWaterDepth/10.0, 0.0, 1.0));

    vec3 toCamera = normalize(fromFragmentToCamera);

    vec3 normal = getNormal(distortedTexCoords);

    // Fresnel Effect. Looking at the water from above makes the water more transparent.
    float refractiveFactor = dot(toCamera, normal);

    // A higher fresnelStrength makes the water more reflective since the
    // refractive factor will decrease
    refractiveFactor = pow(refractiveFactor, fresnelStrength);

    vec3 reflectedLight = reflect(normalize(sunlightDir), normal);
    float specular = max(dot(reflectedLight, toCamera), 0.0);
    specular = pow(specular, shineDamper);
    vec3 specularHighlights = sunlightColor * specular * waterReflectivity;

    fragColor = mix(reflectColor, refractColor, refractiveFactor);
    // Mix in a bit of blue so that it looks like water
    fragColor = mix(fragColor, shallowWaterColor, 0.2) + vec4(specularHighlights, 0.0);
}

vec3 getNormal(vec2 textureCoords) {
    vec4 normalMapColor = texture(normalMap, textureCoords);
    float makeNormalPointUpwardsMore = 2.6;
    vec3 normal = vec3(
      normalMapColor.r * 2.0 - 1.0,
      normalMapColor.b * makeNormalPointUpwardsMore,
      normalMapColor.g * 2.0 - 1.0
    );
    normal = normalize(normal);

    return normal;
}
