// Requires:
// out vec4 frag_nodeColor

void createZPositiveFace(vec4 center, float halfNodeSize, mat4 canonizationMatrix) {
    vec4 position;
    frag_normal = vec3(0,0,1);

    position = vec4(
        center.x - halfNodeSize,
        center.y - halfNodeSize,
        center.z + halfNodeSize,
        center.w
    );
    gl_Position = canonizationMatrix * position;
    EmitVertex();

    position = vec4(
        center.x - halfNodeSize,
        center.y + halfNodeSize,
        center.z + halfNodeSize,
        center.w
    );
    gl_Position = canonizationMatrix * position;
    EmitVertex();

    position = vec4(
        center.x + halfNodeSize,
        center.y - halfNodeSize,
        center.z + halfNodeSize,
        center.w
    );
    gl_Position = canonizationMatrix * position;
    EmitVertex();

    position = vec4(
        center.x + halfNodeSize,
        center.y + halfNodeSize,
        center.z + halfNodeSize,
        center.w
    );
    gl_Position = canonizationMatrix * position;
    EmitVertex();
}

void createXPositiveFace(vec4 center, float halfNodeSize, mat4 canonizationMatrix) {
    vec4 position;
    frag_normal = vec3(1,0,0);

    position = vec4(
        center.x + halfNodeSize,
        center.y - halfNodeSize,
        center.z - halfNodeSize,
        center.w
    );
    gl_Position = canonizationMatrix * position;
    EmitVertex();

    position = vec4(
        center.x + halfNodeSize,
        center.y + halfNodeSize,
        center.z - halfNodeSize,
        center.w
    );
    gl_Position = canonizationMatrix * position;
    EmitVertex();
}

void createZNegativeFace(vec4 center, float halfNodeSize, mat4 canonizationMatrix) {
    vec4 position;
    frag_normal = vec3(0,0,-1);

    position = vec4(
        center.x - halfNodeSize,
        center.y - halfNodeSize,
        center.z - halfNodeSize,
        center.w
    );
    gl_Position = canonizationMatrix * position;
    EmitVertex();

    position = vec4(
        center.x - halfNodeSize,
        center.y + halfNodeSize,
        center.z - halfNodeSize,
        center.w
    );
    gl_Position = canonizationMatrix * position;
    EmitVertex();
}

void createXNegativeFace(vec4 center, float halfNodeSize, mat4 canonizationMatrix) {
    vec4 position;
    frag_normal = vec3(-1,0,0);

    position = vec4(
        center.x - halfNodeSize,
        center.y - halfNodeSize,
        center.z + halfNodeSize,
        center.w
    );
    gl_Position = canonizationMatrix * position;
    EmitVertex();

    position = vec4(
        center.x - halfNodeSize,
        center.y + halfNodeSize,
        center.z + halfNodeSize,
        center.w
    );
    gl_Position = canonizationMatrix * position;
    EmitVertex();
}

void createYPositiveFace(vec4 center, float halfNodeSize, mat4 canonizationMatrix) {
    vec4 position;
    frag_normal = vec3(0,1,0);

    position = vec4(
        center.x - halfNodeSize,
        center.y + halfNodeSize,
        center.z - halfNodeSize,
        center.w
    );
    gl_Position = canonizationMatrix * position;
    EmitVertex();

    position = vec4(
        center.x + halfNodeSize,
        center.y + halfNodeSize,
        center.z - halfNodeSize,
        center.w
    );
    gl_Position = canonizationMatrix * position;
    EmitVertex();

    EmitVertex(); // To start from scratch

    position = vec4(
        center.x + halfNodeSize,
        center.y + halfNodeSize,
        center.z + halfNodeSize,
        center.w
    );
    gl_Position = canonizationMatrix * position;
    EmitVertex();

    position = vec4(
        center.x - halfNodeSize,
        center.y + halfNodeSize,
        center.z + halfNodeSize,
        center.w
    );
    gl_Position = canonizationMatrix * position;
    EmitVertex();
}

void createYNegativeFace(vec4 center, float halfNodeSize, mat4 canonizationMatrix) {
    vec4 position;
    frag_normal = vec3(0,-1,0);

    position = vec4(
        center.x - halfNodeSize,
        center.y - halfNodeSize,
        center.z + halfNodeSize,
        center.w
    );
    gl_Position = canonizationMatrix * position;
    EmitVertex();
    EmitVertex(); // To start from here

    position = vec4(
        center.x - halfNodeSize,
        center.y - halfNodeSize,
        center.z - halfNodeSize,
        center.w
    );
    gl_Position = canonizationMatrix * position;
    EmitVertex();

    position = vec4(
        center.x + halfNodeSize,
        center.y - halfNodeSize,
        center.z + halfNodeSize,
        center.w
    );
    gl_Position = canonizationMatrix * position;
    EmitVertex();

    position = vec4(
        center.x + halfNodeSize,
        center.y - halfNodeSize,
        center.z - halfNodeSize,
        center.w
    );
    gl_Position = canonizationMatrix * position;
    EmitVertex();
}

void drawCubeFilled(vec4 center, float halfNodeSize, mat4 canonizationMatrix, vec4 color) {
    frag_nodeColor = color;
    createZPositiveFace(center, halfNodeSize, canonizationMatrix);
    createXPositiveFace(center, halfNodeSize, canonizationMatrix);
    createZNegativeFace(center, halfNodeSize, canonizationMatrix);
    createXNegativeFace(center, halfNodeSize, canonizationMatrix);

    createYPositiveFace(center, halfNodeSize, canonizationMatrix);

    EmitVertex(); // To start from scratch

    createYNegativeFace(center, halfNodeSize, canonizationMatrix);

    EndPrimitive();
}
