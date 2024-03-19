#shader vertex

#version 460 core

layout (location = 0) in vec3 position;
layout (location = 1) in vec2 textureCoordinates;

out VertexData {
    vec2 textureCoordinates;
} Out;

vec2 scaleAndBias(vec2 p) {
    return 0.5f * p + 0.5f;
}

void main() {
    Out.textureCoordinates = scaleAndBias(position.xy);
    gl_Position = vec4(position, 1);
}

#shader fragment

#version 460 core

#define STEP_LENGTH 0.005f
#define INV_STEP_LENGTH (1.0f / STEP_LENGTH)

uniform sampler2D textureBack;
uniform sampler2D textureFront;
uniform sampler3D voxelsTexture;
uniform vec3 cameraPosition;
uniform int mipmapLevel = 0;

in VertexData {
    vec2 textureCoordinates;
} In;

out vec4 color;

vec3 scaleAndBias(vec3 p) {
    return 0.5f * p + 0.5f;
}

bool isInsideCube(vec3 p, float e) {
    return abs(p.x) < 1 + e && abs(p.y) < 1 + e && abs(p.z) < 1 + e;
}

void main() {
    const vec3 origin = isInsideCube(cameraPosition, 0.2f)
        ? cameraPosition
        : texture(textureFront, In.textureCoordinates).xyz;
    vec3 direction = texture(textureBack, In.textureCoordinates).xyz - origin;
    const uint numberOfSteps = uint(INV_STEP_LENGTH * length(direction));
    direction = normalize(direction);

    // Trace
    color = vec4(0);
    for (uint step = 0; step < numberOfSteps && color.a < 0.99f; ++step) {
        const vec3 currentPoint = origin + STEP_LENGTH * step * direction;
        vec3 coordinate = scaleAndBias(currentPoint);
        vec4 currentSample = textureLod(voxelsTexture, coordinate, mipmapLevel);
        color += (1.0f - color.a) * currentSample;
    }
    color.rgb = pow(color.rgb, vec3(1.0 / 2.2)); // Gamma correction
}
