// Requires:
// - uniform mat4 projection
// - uniform mat4 view
// - out vec4 frag_color

// Takes up 8 lines -> 16 vertices
void drawCone(vec3 origin, vec3 axis, vec3 tangent, vec3 bitangent) {
    // Base
    frag_color = vec4(1, 1, 0, 1); // Cone is yellow as well
    gl_Position = projection * view * vec4(origin, 1.0);
    EmitVertex();
    // sin(75) * axis + cos(75) * tangent
    vec3 positiveTangent = 0.9659 * axis + 0.2588 * tangent;
    gl_Position = projection * view * vec4(origin + positiveTangent * 0.5, 1.0);
    EmitVertex();
    EndPrimitive();

    frag_color = vec4(1, 1, 0, 1); // Cone is yellow as well
    gl_Position = projection * view * vec4(origin, 1.0);
    EmitVertex();
    // sin(75) * axis - cos(75) * tangent
    vec3 negativeTangent = 0.9659 * axis - 0.2588 * tangent;
    gl_Position = projection * view * vec4(origin + negativeTangent * 0.5, 1.0);
    EmitVertex();
    EndPrimitive();

    frag_color = vec4(1, 1, 0, 1); // Cone is yellow as well
    gl_Position = projection * view * vec4(origin, 1.0);
    EmitVertex();
    // sin(75) * axis + cos(75) * bitangent
    vec3 positiveBitangent = 0.9659 * axis + 0.2588 * bitangent;
    gl_Position = projection * view * vec4(origin + positiveBitangent * 0.5, 1.0);
    EmitVertex();
    EndPrimitive();

    frag_color = vec4(1, 1, 0, 1); // Cone is yellow as well
    gl_Position = projection * view * vec4(origin, 1.0);
    EmitVertex();
    // sin(75) * axis - cos(75) * bitangent
    vec3 negativeBitangent = 0.9659 * axis - 0.2588 * bitangent;
    gl_Position = projection * view * vec4(origin + negativeBitangent * 0.5, 1.0);
    EmitVertex();
    EndPrimitive();

    // Top
    gl_Position = projection * view * vec4(origin + positiveTangent * 0.5, 1.0);
    EmitVertex();
    gl_Position = projection * view * vec4(origin + positiveBitangent * 0.5, 1.0);
    EmitVertex();
    EndPrimitive();

    gl_Position = projection * view * vec4(origin + positiveBitangent * 0.5, 1.0);
    EmitVertex();
    gl_Position = projection * view * vec4(origin + negativeTangent * 0.5, 1.0);
    EmitVertex();
    EndPrimitive();

    gl_Position = projection * view * vec4(origin + negativeTangent * 0.5, 1.0);
    EmitVertex();
    gl_Position = projection * view * vec4(origin + negativeBitangent * 0.5, 1.0);
    EmitVertex();
    EndPrimitive();

    gl_Position = projection * view * vec4(origin + negativeBitangent * 0.5, 1.0);
    EmitVertex();
    gl_Position = projection * view * vec4(origin + positiveTangent * 0.5, 1.0);
    EmitVertex();
    EndPrimitive();
}
