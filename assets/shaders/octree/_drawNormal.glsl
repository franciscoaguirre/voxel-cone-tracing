// Requires:
// - mat4 projection
// - mat4 view
// - mat4 model

const float MAGNITUDE = 0.1;

void drawNormal(vec3 start, vec3 normal) {
    vec4 startPosition = view * model * vec4(start, 1.0);
    mat3 normalMatrix = mat3(transpose(inverse(view * model)));
    vec3 fixedNormal = normalize(vec3(vec4(normalMatrix * normal, 0.0)));

    gl_Position = projection * startPosition;
    EmitVertex();
    gl_Position = projection * (startPosition + vec4(fixedNormal, 0.0) * MAGNITUDE);
    EmitVertex();
    EndPrimitive();
}
