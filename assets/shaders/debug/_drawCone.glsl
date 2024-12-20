// Requires:
// - uniform mat4 projection
// - uniform mat4 view
// - out vec4 frag_color

// Takes up 8 lines -> 16 vertices
void drawCone(vec3 origin, vec3 axis, float angle, float maxDistance) {
    vec3 helper = vec3(0.12, 0.32, 0.82); // Random values
    vec3 tangent = normalize(helper - dot(axis, helper) * axis);
    vec3 bitangent = cross(axis, tangent);
    float sinAngle = sin(angle);
    float cosAngle = cos(angle);
    
    // Base
    frag_color = vec4(1, 1, 0, 1); // Cone is yellow as well
    gl_Position = projection * view * vec4(origin, 1.0);
    EmitVertex();
    // sin(75) * axis + cos(75) * tangent
    vec3 positiveTangent = sinAngle * axis + cosAngle * tangent;
    gl_Position = projection * view * vec4(origin + positiveTangent * maxDistance * 2, 1.0);
    EmitVertex();
    EndPrimitive();

    frag_color = vec4(1, 1, 0, 1); // Cone is yellow as well
    gl_Position = projection * view * vec4(origin, 1.0);
    EmitVertex();
    // sin(75) * axis - cos(75) * tangent
    vec3 negativeTangent = sinAngle * axis - cosAngle * tangent;
    gl_Position = projection * view * vec4(origin + negativeTangent * maxDistance * 2, 1.0);
    EmitVertex();
    EndPrimitive();

    frag_color = vec4(1, 1, 0, 1); // Cone is yellow as well
    gl_Position = projection * view * vec4(origin, 1.0);
    EmitVertex();
    // sin(75) * axis + cos(75) * bitangent
    vec3 positiveBitangent = sinAngle * axis + cosAngle * bitangent;
    gl_Position = projection * view * vec4(origin + positiveBitangent * maxDistance * 2, 1.0);
    EmitVertex();
    EndPrimitive();

    frag_color = vec4(1, 1, 0, 1); // Cone is yellow as well
    gl_Position = projection * view * vec4(origin, 1.0);
    EmitVertex();
    // sin(75) * axis - cos(75) * bitangent
    vec3 negativeBitangent = sinAngle * axis - cosAngle * bitangent;
    gl_Position = projection * view * vec4(origin + negativeBitangent * maxDistance * 2, 1.0);
    EmitVertex();
    EndPrimitive();

    // Top
    gl_Position = projection * view * vec4(origin + positiveTangent * maxDistance * 2, 1.0);
    EmitVertex();
    gl_Position = projection * view * vec4(origin + positiveBitangent * maxDistance * 2, 1.0);
    EmitVertex();
    EndPrimitive();

    gl_Position = projection * view * vec4(origin + positiveBitangent * maxDistance * 2, 1.0);
    EmitVertex();
    gl_Position = projection * view * vec4(origin + negativeTangent * maxDistance * 2, 1.0);
    EmitVertex();
    EndPrimitive();

    gl_Position = projection * view * vec4(origin + negativeTangent * maxDistance * 2, 1.0);
    EmitVertex();
    gl_Position = projection * view * vec4(origin + negativeBitangent * maxDistance * 2, 1.0);
    EmitVertex();
    EndPrimitive();

    gl_Position = projection * view * vec4(origin + negativeBitangent * maxDistance * 2, 1.0);
    EmitVertex();
    gl_Position = projection * view * vec4(origin + positiveTangent * maxDistance * 2, 1.0);
    EmitVertex();
    EndPrimitive();
}
