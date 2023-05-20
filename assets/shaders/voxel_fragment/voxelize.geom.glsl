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
out vec2 edgeNormal;
out vec2 semiDiagonal;

flat out int frag_dominantAxis;
flat out vec4 frag_aabb; 

uniform layout(binding = 3, r32f) imageBuffer debug;

uniform int voxelDimension;
uniform mat4 axisProjections[3];

bool lineIntersection(vec2 p1, vec2 p2, vec2 q1, vec2 q2, out vec2 intersection);
vec2 normalToSemiDiagonal(vec2 normal);
float zFromPlaneAndPoint(vec2 point, vec4 plane, float defaultValue);

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

bool lineIntersection(vec2 p1, vec2 p2, vec2 q1, vec2 q2, out vec2 intersection);
vec2 normalToSemiDiagonal(vec2 normal);

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

    vec4 temp;
    // Project triangle to dominant plane
    if (dominantAxis == 0) {
        // x-axis is depth
        for (int i = 0; i < gl_in.length(); i++)
        {
            temp.x = vertex[i].z;
            temp.z = vertex[i].x; 

            vertex[i].xz = temp.xz; 
        }

    } else if (dominantAxis == 1) {
        // y-axis is depth
        for (int i = 0; i < gl_in.length(); i++)
        {
            temp.y = vertex[i].z;
            temp.z = vertex[i].y;

            vertex[i].yz = temp.yz; 
        }
    } else {
        // z-axis is depth, which is usual case so do nothing
    }

  
    // Project triangle to dominant plane
    //for (int i = 0; i < gl_in.length(); i++) {
      //vertex[i] = axisProjections[dominantAxis] * vertex[i];
    //}

    vec3 projectedTriangleNormal = normalize(cross(vertex[1].xyz - vertex[0].xyz, vertex[2].xyz - vertex[0].xyz));
    vec4 trianglePlane;
    trianglePlane.xyz = projectedTriangleNormal;
    trianglePlane.w = -dot(projectedTriangleNormal, vertex[0].xyz);

    float normalMultiplier = 1.0;
    if (dot(projectedTriangleNormal, vec3(0, 0, 1)) > 0.0) {
        normalMultiplier = -1.0;
    }

    // vec2(2.0 / voxelDimension) is the pixel size, as coordinates go from -1 to 1 (length 2), so a half pixel is half of that
    vec2 halfPixel = vec2(1.0 / voxelDimension);
    // vec2 halfPixel = vec2(0.5);

    vec4 aabb = defineAabb(vertex, halfPixel);
    frag_aabb = aabb;

    vec3 expandedVertex[3];
    for (int i = 0; i < 3; i++) {
        vec2 currentEdge = vertex[(i + 1) % 3].xy - vertex[i].xy;
        vec2 previousEdge = vertex[i].xy - vertex[(i + 2) % 3].xy;

        vec2 currentNormal = normalize(vec2(-currentEdge.y, currentEdge.x)) * normalMultiplier;
        vec2 currentSemiDiagonal = normalToSemiDiagonal(currentNormal);
        vec2 previousNormal = normalize(vec2(-previousEdge.y, previousEdge.x)) * normalMultiplier;
        vec2 previousSemiDiagonal = normalToSemiDiagonal(previousNormal);

        vec2 currentExpanded1 = vertex[i].xy + currentSemiDiagonal * halfPixel;
        vec2 currentExpanded2 = vertex[(i + 1) % 3].xy + currentSemiDiagonal * halfPixel;
        vec2 previousExpanded1 = vertex[(i + 2) % 3].xy + previousSemiDiagonal * halfPixel;
        vec2 previousExpanded2 = vertex[i].xy + previousSemiDiagonal * halfPixel;

        vec2 intersection;
        if (lineIntersection(currentExpanded1, currentExpanded2, previousExpanded1, previousExpanded2, intersection)) {
            expandedVertex[i].xy = intersection;
            expandedVertex[i].z = zFromPlaneAndPoint(intersection, trianglePlane, vertex[i].z);

        } else {
            // We f***** up
            expandedVertex[i] = vertex[i].xyz;
        }
        
        // Debug values
        edgeNormal = currentNormal;
        semiDiagonal = currentSemiDiagonal;

        gl_Position = vec4(expandedVertex[i], 1.0);
        Out.position = expandedVertex[i];
        Out.normal = In[i].normal;
        Out.textureCoordinates = In[i].textureCoordinates;
        EmitVertex();
    }

    EndPrimitive();
}

vec2 normalToSemiDiagonal(vec2 normal) {
    vec2 signVec = sign(normal);
    vec2 semiDiagonal;

    semiDiagonal.x = signVec.x == 0.0 ? 1.0 : signVec.x;
    semiDiagonal.y = signVec.y == 0.0 ? 1.0 : signVec.y;
    
    return normalize(semiDiagonal);
}

bool lineIntersection(vec2 p1, vec2 p2, vec2 q1, vec2 q2, out vec2 intersection) {
    vec2 r = p2 - p1;
    vec2 s = q2 - q1;
    float rxs = cross(vec3(r, 0.0), vec3(s, 0.0)).z;

    if (abs(rxs) < 1e-6) {
        // Lines are parallel or coincident
        return false;
    }

    vec2 pq = q1 - p1;
    float pqxr = cross(vec3(pq, 0.0), vec3(r, 0.0)).z;
    float pqxs = cross(vec3(pq, 0.0), vec3(s, 0.0)).z;

    float t = pqxs / rxs;
    float u = pqxr / rxs;

    intersection = p1 + t * r;

    return true;
}

float zFromPlaneAndPoint(vec2 point, vec4 plane, float defaultValue) {
  if (plane.z == 0.0) {
    return defaultValue;
  }
  return (point.x * plane.x + point.y * plane.y + plane.w) / -plane.z;
}
