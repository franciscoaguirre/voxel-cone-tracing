/// 5 vertices
void create_z_positive_face(vec4 center, vec3 dimensions, mat4 canonizationMatrix, vec4 color) {
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
void create_x_negative_face(vec4 center, vec3 dimensions, mat4 canonizationMatrix, vec4 color) {
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

void create_y_positive_face(vec4 center, vec3 dimensions, mat4 canonizationMatrix, vec4 color) {
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

void create_z_negative_face(vec4 center, vec3 dimensions, mat4 canonizationMatrix, vec4 color) {
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

void create_y_negative_face(vec4 center, vec3 dimensions, mat4 canonizationMatrix, vec4 color) {
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

void drawCube(vec4 center, float halfNodeSize, mat4 canonizationMatrix, vec4 color) {
    frag_nodeColor = color;
    create_z_positive_face(center, vec3(halfNodeSize), canonizationMatrix, color);
    create_x_negative_face(center, vec3(halfNodeSize), canonizationMatrix, color);
    create_y_positive_face(center, vec3(halfNodeSize), canonizationMatrix, color);
    create_z_negative_face(center, vec3(halfNodeSize), canonizationMatrix, color);
    create_y_negative_face(center, vec3(halfNodeSize), canonizationMatrix, color);
    EndPrimitive();
}

void drawCube(vec4 center, vec3 dimensions, mat4 canonizationMatrix, vec4 color) {
    frag_nodeColor = color;
    create_z_positive_face(center, dimensions, canonizationMatrix, color);
    create_x_negative_face(center, dimensions, canonizationMatrix, color);
    create_y_positive_face(center, dimensions, canonizationMatrix, color);
    create_z_negative_face(center, dimensions, canonizationMatrix, color);
    create_y_negative_face(center, dimensions, canonizationMatrix, color);
    EndPrimitive();
}
