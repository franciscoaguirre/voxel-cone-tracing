#version 460 core

layout (triangles) in;
layout (triangle_strip, max_vertices = 3) out;

in VertexData {
    vec3 position;
    vec3 normal;
    vec2 textureCoordinates;
} In[3];

out VoxelData {
    vec3 position;
    vec3 normal;
    vec2 textureCoordinates;
} Out;

flat out int frag_dominantAxis;
flat out vec4 frag_aabb; 

uniform int voxelDimension;
uniform mat4 axisProjections[3];

// 0 means x, 1 means y, 2 means z
int biggestComponent(vec3 triangleNormal) {
    float xComponent = abs(triangleNormal.x);
    float yComponent = abs(triangleNormal.y);
    float zComponent = abs(triangleNormal.z);

    if (xComponent > yComponent && xComponent > zComponent) {
      return 0;
    }

    if (yComponent > zComponent) {
      return 1;
    } else {
      return 2;
    }
}

vec4 defineAabb(vec4 points[3], vec2 halfPixel) {
    vec4 aabb;

    aabb.xy = points[0].xy;
    aabb.zw = points[0].xy;

    aabb.xy = min(points[1].xy, aabb.xy);
    aabb.zw = max(points[1].xy, aabb.zw);

    aabb.xy = min(points[2].xy, aabb.xy);
    aabb.zw = max(points[2].xy, aabb.zw);

    return aabb + vec4(-halfPixel, halfPixel);
}

void main() {
    // TODO: Check if it's better to use the model normals.
    // We could use any vertex normal or average all of them.
    vec3 triangleNormal = normalize(
        cross(
            In[1].position - In[0].position,
            In[2].position - In[0].position
        )
    );

    int dominantAxis = biggestComponent(triangleNormal);
    frag_dominantAxis = dominantAxis;
    mat4 projection;

    vec4 vertex[3];
    vertex[0] = vec4(In[0].position, 1.0);
    vertex[1] = vec4(In[1].position, 1.0);
    vertex[2] = vec4(In[2].position, 1.0);
  
    // Project triangle to dominant plane
    for (int i = 0; i < gl_in.length(); i++) {
      vertex[i] = axisProjections[dominantAxis] * vertex[i];
    }

    vec3 projectedTriangleNormal = normalize(cross(vertex[1].xyz - vertex[0].xyz, vertex[2].xyz - vertex[0].xyz));

    vec3 normals[3];
    vec2 tex_coordinates[3];
    // Change triangle winding, so normal points to "camera"
    if (dot(projectedTriangleNormal, vec3(0.0, 0.0, 1.0)) < 0.0)
    {
        vec4 vertexTemp = vertex[2];
        vertex[2] = vertex[1];
        vertex[1] = vertexTemp;

        normals[0] = In[0].normal;
        normals[1] = In[2].normal;
        normals[2] = In[1].normal;

        tex_coordinates[0] = In[0].textureCoordinates;
        tex_coordinates[1] = In[2].textureCoordinates;
        tex_coordinates[2] = In[1].textureCoordinates;
    }
    // vec2(2.0 / voxelDimension) is the pixel size, as coordinates go from -1 to 1 (length 2), so a half pixel is half of that
    vec2 halfPixel = vec2(1.0 / voxelDimension);

    vec4 aabb = defineAabb(vertex, halfPixel);
    frag_aabb = aabb;

    vec4 trianglePlane;

    trianglePlane.xyz = normalize(cross(vertex[1].xyz - vertex[0].xyz, vertex[2].xyz - vertex[0].xyz));

    trianglePlane.w = -dot(vertex[0].xyz, trianglePlane.xyz);

    if (trianglePlane.z == 0.0) {
        return;
    }

    vec3 plane[3];

    for (int i = 0; i < gl_in.length(); i++) {
      plane[i] = cross(vertex[i].xyw, vertex[(i + 2) % 3].xyw);
      plane[i].z -= dot(halfPixel, abs(plane[i].xy));
    }
        
    vec3 intersect[3];

    for (int i = 0; i < gl_in.length(); i++) {
        intersect[i] = cross(plane[i], plane[(i+1) % 3]);

        if (intersect[i].z == 0.0) {
            return;
        }

        intersect[i] /= intersect[i].z; 
    }

    for (int i = 0; i < 3; i++) {
      gl_Position.xyw = intersect[i];
        
      // Calculate the new z-Coordinate derived from a point on a plane
      gl_Position.z = -(trianglePlane.x * intersect[i].x + trianglePlane.y * intersect[i].y + trianglePlane.w) / trianglePlane.z; 
      Out.position = intersect[i].xyz;
      Out.normal = normals[i];
      Out.textureCoordinates = tex_coordinates[i];
      EmitVertex();
    }

    EndPrimitive();
}
