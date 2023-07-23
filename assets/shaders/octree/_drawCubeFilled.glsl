// Requires:
// out vec4 frag_nodeColor

// Box defined similar to our aabb, with just a lower and upper bound
vec4 positionBoundByBox(float x, float y, float z, float w, vec3 lowerBound, vec3 upperBound) {
  return vec4(clamp(vec3(x, y, z), lowerBound, upperBound), w);
}

void createZPositiveFace(vec4 center, float halfNodeSize, mat4 canonizationMatrix, vec3 lowerBound, vec3 upperBound) {
    vec4 position;
    frag_normal = vec3(0,0,1);

    position = positionBoundByBox(
        center.x - halfNodeSize,
        center.y - halfNodeSize,
        center.z + halfNodeSize,
        center.w,
        lowerBound,
        upperBound
    );
    gl_Position = canonizationMatrix * position;
    EmitVertex();

    position = positionBoundByBox(
        center.x - halfNodeSize,
        center.y + halfNodeSize,
        center.z + halfNodeSize,
        center.w,
        lowerBound,
        upperBound
    );
    gl_Position = canonizationMatrix * position;
    EmitVertex();

    position = positionBoundByBox(
        center.x + halfNodeSize,
        center.y - halfNodeSize,
        center.z + halfNodeSize,
        center.w,
        lowerBound,
        upperBound
    );
    gl_Position = canonizationMatrix * position;
    EmitVertex();

    position = positionBoundByBox(
        center.x + halfNodeSize,
        center.y + halfNodeSize,
        center.z + halfNodeSize,
        center.w,
        lowerBound,
        upperBound
    );
    gl_Position = canonizationMatrix * position;
    EmitVertex();
}

void createXPositiveFace(vec4 center, float halfNodeSize, mat4 canonizationMatrix, vec3 lowerBound, vec3 upperBound) {
    vec4 position;
    frag_normal = vec3(1,0,0);

    position = positionBoundByBox(
        center.x + halfNodeSize,
        center.y - halfNodeSize,
        center.z - halfNodeSize,
        center.w,
        lowerBound,
        upperBound
    );
    gl_Position = canonizationMatrix * position;
    EmitVertex();

    position = positionBoundByBox(
        center.x + halfNodeSize,
        center.y + halfNodeSize,
        center.z - halfNodeSize,
        center.w,
        lowerBound,
        upperBound
    );
    gl_Position = canonizationMatrix * position;
    EmitVertex();
}

void createZNegativeFace(vec4 center, float halfNodeSize, mat4 canonizationMatrix, vec3 lowerBound, vec3 upperBound) {
    vec4 position;
    frag_normal = vec3(0,0,-1);

    position = positionBoundByBox(
        center.x - halfNodeSize,
        center.y - halfNodeSize,
        center.z - halfNodeSize,
        center.w,
        lowerBound,
        upperBound
    );
    gl_Position = canonizationMatrix * position;
    EmitVertex();

    position = positionBoundByBox(
        center.x - halfNodeSize,
        center.y + halfNodeSize,
        center.z - halfNodeSize,
        center.w,
        lowerBound,
        upperBound
    );
    gl_Position = canonizationMatrix * position;
    EmitVertex();
}

void createXNegativeFace(vec4 center, float halfNodeSize, mat4 canonizationMatrix, vec3 lowerBound, vec3 upperBound) {
    vec4 position;
    frag_normal = vec3(-1,0,0);

    position = positionBoundByBox(
        center.x - halfNodeSize,
        center.y - halfNodeSize,
        center.z + halfNodeSize,
        center.w,
        lowerBound,
        upperBound
    );
    gl_Position = canonizationMatrix * position;
    EmitVertex();

    position = positionBoundByBox(
        center.x - halfNodeSize,
        center.y + halfNodeSize,
        center.z + halfNodeSize,
        center.w,
        lowerBound,
        upperBound
    );
    gl_Position = canonizationMatrix * position;
    EmitVertex();
}

void createYPositiveFace(vec4 center, float halfNodeSize, mat4 canonizationMatrix, vec3 lowerBound, vec3 upperBound) {
    vec4 position;
    frag_normal = vec3(0,1,0);

    position = positionBoundByBox(
        center.x - halfNodeSize,
        center.y + halfNodeSize,
        center.z - halfNodeSize,
        center.w,
        lowerBound,
        upperBound
    );
    gl_Position = canonizationMatrix * position;
    EmitVertex();

    position = positionBoundByBox(
        center.x + halfNodeSize,
        center.y + halfNodeSize,
        center.z - halfNodeSize,
        center.w,
        lowerBound,
        upperBound
    );
    gl_Position = canonizationMatrix * position;
    EmitVertex();

    EmitVertex(); // To start from scratch

    position = positionBoundByBox(
        center.x + halfNodeSize,
        center.y + halfNodeSize,
        center.z + halfNodeSize,
        center.w,
        lowerBound,
        upperBound
    );
    gl_Position = canonizationMatrix * position;
    EmitVertex();

    position = positionBoundByBox(
        center.x - halfNodeSize,
        center.y + halfNodeSize,
        center.z + halfNodeSize,
        center.w,
        lowerBound,
        upperBound
    );
    gl_Position = canonizationMatrix * position;
    EmitVertex();
}

void createYNegativeFace(vec4 center, float halfNodeSize, mat4 canonizationMatrix, vec3 lowerBound, vec3 upperBound) {
    vec4 position;
    frag_normal = vec3(0,-1,0);

    position = positionBoundByBox(
        center.x - halfNodeSize,
        center.y - halfNodeSize,
        center.z + halfNodeSize,
        center.w,
        lowerBound,
        upperBound
    );
    gl_Position = canonizationMatrix * position;
    EmitVertex();
    EmitVertex(); // To start from here

    position = positionBoundByBox(
        center.x - halfNodeSize,
        center.y - halfNodeSize,
        center.z - halfNodeSize,
        center.w,
        lowerBound,
        upperBound
    );
    gl_Position = canonizationMatrix * position;
    EmitVertex();

    position = positionBoundByBox(
        center.x + halfNodeSize,
        center.y - halfNodeSize,
        center.z + halfNodeSize,
        center.w,
        lowerBound,
        upperBound
    );
    gl_Position = canonizationMatrix * position;
    EmitVertex();

    position = positionBoundByBox(
        center.x + halfNodeSize,
        center.y - halfNodeSize,
        center.z - halfNodeSize,
        center.w,
        lowerBound,
        upperBound
    );
    gl_Position = canonizationMatrix * position;
    EmitVertex();
}

void drawCubeFilled(vec4 center, float halfNodeSize, mat4 canonizationMatrix, vec4 color, vec3 lowerBound, vec3 upperBound) {
    frag_nodeColor = color;
    createZPositiveFace(center, halfNodeSize, canonizationMatrix, lowerBound, upperBound);
    createXPositiveFace(center, halfNodeSize, canonizationMatrix, lowerBound, upperBound);
    createZNegativeFace(center, halfNodeSize, canonizationMatrix, lowerBound, upperBound);
    createXNegativeFace(center, halfNodeSize, canonizationMatrix, lowerBound, upperBound);

    createYPositiveFace(center, halfNodeSize, canonizationMatrix, lowerBound, upperBound);

    EmitVertex(); // To start from scratch

    createYNegativeFace(center, halfNodeSize, canonizationMatrix, lowerBound, upperBound);

    EndPrimitive();
}
void drawCubeFilled(vec4 center, float halfNodeSize, mat4 canonizationMatrix, vec4 color) {
  drawCubeFilled(center, halfNodeSize, canonizationMatrix, color, vec3(-10000), vec3(10000));
}
