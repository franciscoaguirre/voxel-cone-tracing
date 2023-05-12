// Requires:
// - uniform mat4 projection
// - uniform mat4 view
// - out vec4 frag_color

const float SIN_ANGLE = 0.9659;
const float COS_ANGLE = 0.2588;
const float MAGNITUDE = 0.01;

// Takes up 8 lines -> 16 vertices
void drawCone(vec3 origin, vec3 axis, float angle) {
    vec3 helper = axis + vec3(0.1, 0, 0); // Random values
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
    gl_Position = projection * view * vec4(origin + positiveTangent * MAGNITUDE * 2, 1.0);
    EmitVertex();
    EndPrimitive();

    frag_color = vec4(1, 1, 0, 1); // Cone is yellow as well
    gl_Position = projection * view * vec4(origin, 1.0);
    EmitVertex();
    // sin(75) * axis - cos(75) * tangent
    vec3 negativeTangent = sinAngle * axis - cosAngle * tangent;
    gl_Position = projection * view * vec4(origin + negativeTangent * MAGNITUDE * 2, 1.0);
    EmitVertex();
    EndPrimitive();

    frag_color = vec4(1, 1, 0, 1); // Cone is yellow as well
    gl_Position = projection * view * vec4(origin, 1.0);
    EmitVertex();
    // sin(75) * axis + cos(75) * bitangent
    vec3 positiveBitangent = sinAngle * axis + cosAngle * bitangent;
    gl_Position = projection * view * vec4(origin + positiveBitangent * MAGNITUDE * 2, 1.0);
    EmitVertex();
    EndPrimitive();

    frag_color = vec4(1, 1, 0, 1); // Cone is yellow as well
    gl_Position = projection * view * vec4(origin, 1.0);
    EmitVertex();
    // sin(75) * axis - cos(75) * bitangent
    vec3 negativeBitangent = sinAngle * axis - cosAngle * bitangent;
    gl_Position = projection * view * vec4(origin + negativeBitangent * MAGNITUDE * 2, 1.0);
    EmitVertex();
    EndPrimitive();

    // Top
    gl_Position = projection * view * vec4(origin + positiveTangent * MAGNITUDE * 2, 1.0);
    EmitVertex();
    gl_Position = projection * view * vec4(origin + positiveBitangent * MAGNITUDE * 2, 1.0);
    EmitVertex();
    EndPrimitive();

    gl_Position = projection * view * vec4(origin + positiveBitangent * MAGNITUDE * 2, 1.0);
    EmitVertex();
    gl_Position = projection * view * vec4(origin + negativeTangent * MAGNITUDE * 2, 1.0);
    EmitVertex();
    EndPrimitive();

    gl_Position = projection * view * vec4(origin + negativeTangent * MAGNITUDE * 2, 1.0);
    EmitVertex();
    gl_Position = projection * view * vec4(origin + negativeBitangent * MAGNITUDE * 2, 1.0);
    EmitVertex();
    EndPrimitive();

    gl_Position = projection * view * vec4(origin + negativeBitangent * MAGNITUDE * 2, 1.0);
    EmitVertex();
    gl_Position = projection * view * vec4(origin + positiveTangent * MAGNITUDE * 2, 1.0);
    EmitVertex();
    EndPrimitive();
}
