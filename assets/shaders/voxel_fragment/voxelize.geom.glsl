#version 460 core

layout (triangles) in;
layout (triangle_strip, max_vertices = 3) out;

in vec3 geom_vertexPosition[];
in vec3 geom_normal[];
in vec2 geom_texCoordinates[];

out vec3 frag_position;
out vec3 frag_normal;
out vec2 frag_texCoordinates;
flat out int frag_dominantAxis;
flat out vec4 frag_aabb; 

uniform int voxelDimension;

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
    vec3 triangleNormal = normalize(
        cross(
            geom_vertexPosition[1] - geom_vertexPosition[0],
            geom_vertexPosition[2] - geom_vertexPosition[0]
        )
    );

    int dominantAxis = biggestComponent(triangleNormal);
    frag_dominantAxis = dominantAxis;
    mat4 projection;

    vec4 vertex[3];
    vertex[0] = gl_in[0].gl_Position;
    vertex[1] = gl_in[1].gl_Position;
    vertex[2] = gl_in[2].gl_Position;
    vec3 temp;

    // Project triangle to dominant plane
    if (dominantAxis == 0) {
        // x-axis is depth
        for (int i = 0; i < gl_in.length(); i++)
        {
            temp.x = vertex[i].z;
            temp.z = -vertex[i].x; 
            
            vertex[i].xz = temp.xz; 
        }
    
    } else if (dominantAxis == 1) {
        // y-axis is depth
        for (int i = 0; i < gl_in.length(); i++)
        {
            temp.y = vertex[i].z;
            temp.z = -vertex[i].y;
            
            vertex[i].yz = temp.yz; 
        }
    } else {
        // z-axis is depth, which is usual case so do nothing
    }

    vec3 projectedTriangleNormal = normalize(cross(vertex[1].xyz - vertex[0].xyz, vertex[2].xyz - vertex[0].xyz));

    // Change triangle winding, so normal points to "camera"
    if (dot(projectedTriangleNormal, vec3(0.0, 0.0, 1.0)) < 0.0)
    {
        vec4 vertexTemp = vertex[2];
        vertex[2] = vertex[1];
        vertex[1] = vertexTemp;
    }
    vec2 halfPixel = vec2(1.0 / voxelDimension, 1.0 / voxelDimension);

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
      frag_position = intersect[i].xyz;
      frag_normal = geom_normal[i];
      frag_texCoordinates = geom_texCoordinates[i];
      EmitVertex();
    }

    EndPrimitive();
}
