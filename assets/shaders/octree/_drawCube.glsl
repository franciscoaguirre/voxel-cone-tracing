/// 5 vertices
void createZPositiveFace(vec4 center, vec3 dimensions, mat4 canonizationMatrix, vec4 color) {
    vec4 position;
    
    position = vec4(
        center.x - dimensions.x,
        center.y - dimensions.y,
        center.z + dimensions.z,
        center.w
    );
    gl_Position = canonizationMatrix * position;
    EmitVertex();

    position = vec4(
        center.x - dimensions.x,
        center.y + dimensions.y,
        center.z + dimensions.z,
        center.w
    );
    gl_Position = canonizationMatrix * position;
    EmitVertex();

    position = vec4(
        center.x + dimensions.x,
        center.y + dimensions.y,
        center.z + dimensions.z,
        center.w
    );
    gl_Position = canonizationMatrix * position;
    EmitVertex();

    position = vec4(
        center.x + dimensions.x,
        center.y - dimensions.y,
        center.z + dimensions.z,
        center.w
    );
    gl_Position = canonizationMatrix * position;
    EmitVertex();
    position = vec4(
        center.x - dimensions.x,
        center.y - dimensions.y,
        center.z + dimensions.z,
        center.w
    );
    gl_Position = canonizationMatrix * position;
    EmitVertex();
}

/// 3 vertices
void createXNegativeFace(vec4 center, vec3 dimensions, mat4 canonizationMatrix, vec4 color) {
    vec4 position;

    position = vec4(
        center.x - dimensions.x,
        center.y - dimensions.y,
        center.z - dimensions.z,
        center.w
    );
    gl_Position = canonizationMatrix * position;
    EmitVertex();

    position = vec4(
        center.x - dimensions.x,
        center.y + dimensions.y,
        center.z - dimensions.z,
        center.w
    );
    gl_Position = canonizationMatrix * position;
    EmitVertex();

    position = vec4(
        center.x - dimensions.x,
        center.y + dimensions.y,
        center.z + dimensions.z,
        center.w
    );
    gl_Position = canonizationMatrix * position;
    EmitVertex();
}

void createYPositiveFace(vec4 center, vec3 dimensions, mat4 canonizationMatrix, vec4 color) {
    vec4 position;

    position = vec4(
        center.x + dimensions.x,
        center.y + dimensions.y,
        center.z + dimensions.z,
        center.w
    );
    gl_Position = canonizationMatrix * position;
    EmitVertex();

    position = vec4(
        center.x + dimensions.x,
        center.y + dimensions.y,
        center.z - dimensions.z,
        center.w
    );
    gl_Position = canonizationMatrix * position;
    EmitVertex();

    position = vec4(
        center.x - dimensions.x,
        center.y + dimensions.y,
        center.z - dimensions.z,
        center.w
    );
    gl_Position = canonizationMatrix * position;
    EmitVertex();
}

void createZNegativeFace(vec4 center, vec3 dimensions, mat4 canonizationMatrix, vec4 color) {
    vec4 position;

    position = vec4(
        center.x - dimensions.x,
        center.y - dimensions.y,
        center.z - dimensions.z,
        center.w
    );
    gl_Position = canonizationMatrix * position;
    EmitVertex();

    position = vec4(
        center.x + dimensions.x,
        center.y - dimensions.y,
        center.z - dimensions.z,
        center.w
    );
    gl_Position = canonizationMatrix * position;
    EmitVertex();

    position = vec4(
        center.x + dimensions.x,
        center.y + dimensions.y,
        center.z - dimensions.z,
        center.w
    );
    gl_Position = canonizationMatrix * position;
    EmitVertex();
}

void createYNegativeFace(vec4 center, vec3 dimensions, mat4 canonizationMatrix, vec4 color) {
    vec4 position;

    position = vec4(
        center.x + dimensions.x,
        center.y - dimensions.y,
        center.z - dimensions.z,
        center.w
    );
    gl_Position = canonizationMatrix * position;
    EmitVertex();

    position = vec4(
        center.x + dimensions.x,
        center.y - dimensions.y,
        center.z + dimensions.z,
        center.w
    );
    gl_Position = canonizationMatrix * position;
    EmitVertex();
}

void drawCube(vec4 center, vec3 dimensions, mat4 canonizationMatrix, vec4 color) {
    frag_nodeColor = color;
    createZPositiveFace(center, dimensions, canonizationMatrix, color);
    createXNegativeFace(center, dimensions, canonizationMatrix, color);
    createYPositiveFace(center, dimensions, canonizationMatrix, color);
    createZNegativeFace(center, dimensions, canonizationMatrix, color);
    createYNegativeFace(center, dimensions, canonizationMatrix, color);
    EndPrimitive();
}

void drawCube(vec4 center, float halfNodeSize, mat4 canonizationMatrix, vec4 color) {
    drawCube(center, vec3(halfNodeSize), canonizationMatrix, color);
}
