void create_z_positive_face(vec4 center, float halfNodeSize, mat4 canonizationMatrix, vec4 color) {
    vec4 position;

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
        center.x - halfNodeSize,
        center.y - halfNodeSize,
        center.z + halfNodeSize,
        center.w
    );
    gl_Position = canonizationMatrix * position;
    EmitVertex();
}

void create_x_negative_face(vec4 center, float halfNodeSize, mat4 canonizationMatrix, vec4 color) {
    vec4 position;

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

    position = vec4(
        center.x - halfNodeSize,
        center.y + halfNodeSize,
        center.z + halfNodeSize,
        center.w
    );
    gl_Position = canonizationMatrix * position;
    EmitVertex();
}

void create_y_positive_face(vec4 center, float halfNodeSize, mat4 canonizationMatrix, vec4 color) {
    vec4 position;

    position = vec4(
        center.x + halfNodeSize,
        center.y + halfNodeSize,
        center.z + halfNodeSize,
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

    position = vec4(
        center.x - halfNodeSize,
        center.y + halfNodeSize,
        center.z - halfNodeSize,
        center.w
    );
    gl_Position = canonizationMatrix * position;
    EmitVertex();
}

void create_z_negative_face(vec4 center, float halfNodeSize, mat4 canonizationMatrix, vec4 color) {
    vec4 position;

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

void create_y_negative_face(vec4 center, float halfNodeSize, mat4 canonizationMatrix, vec4 color) {
    vec4 position;

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
        center.y - halfNodeSize,
        center.z + halfNodeSize,
        center.w
    );
    gl_Position = canonizationMatrix * position;
    EmitVertex();
}

void drawCube(vec4 center, float halfNodeSize, mat4 canonizationMatrix, vec4 color) {
    frag_nodeColor = color;
    create_z_positive_face(center, halfNodeSize, canonizationMatrix, color);
    create_x_negative_face(center, halfNodeSize, canonizationMatrix, color);
    create_y_positive_face(center, halfNodeSize, canonizationMatrix, color);
    create_z_negative_face(center, halfNodeSize, canonizationMatrix, color);
    create_y_negative_face(center, halfNodeSize, canonizationMatrix, color);
    EndPrimitive();
}
