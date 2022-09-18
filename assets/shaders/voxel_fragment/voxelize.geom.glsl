#version 460 core

layout (triangles) in;
layout (triangle_strip, max_vertices = 3) out;

in vec3 vertex_position[];
in vec3 normal[];
in vec2 tex_coordinates[];

out vec3 fragment_position;
out vec3 fragment_normal;
out vec2 fragment_texture_coordinates;
flat out int fragment_dominant_axis;
flat out vec4 fragment_aabb; 

uniform int voxel_dimension;

int biggest_component(vec3 triangle_normal) {
    float x_component = abs(triangle_normal.x);
    float y_component = abs(triangle_normal.y);
    float z_component = abs(triangle_normal.z);

    if(x_component > y_component && x_component > z_component) {
      return 0;
    }

    if(y_component > z_component) {
      return 1;
    } else {
      return 2;
    }
}

vec4 define_aabb(vec4 points[3], vec2 half_pixel) {
    vec4 aabb;

    aabb.xy = points[0].xy;
    aabb.zw = points[0].xy;

    aabb.xy = min(points[1].xy, aabb.xy);
    aabb.zw = max(points[1].xy, aabb.zw);

    aabb.xy = min(points[2].xy, aabb.xy);
    aabb.zw = max(points[2].xy, aabb.zw);

    return aabb + vec4(-half_pixel, half_pixel);
}

void main() {
    vec3 triangle_normal = normalize(
        cross(
            vertex_position[1] - vertex_position[0],
            vertex_position[2] - vertex_position[0]
        )
    );

    int dominant_axis = biggest_component(triangle_normal);
    fragment_dominant_axis = dominant_axis;
    mat4 projection;

    vec4 vertex[3];
    vertex[0] = gl_in[0].gl_Position;
    vertex[1] = gl_in[1].gl_Position;
    vertex[2] = gl_in[2].gl_Position;
    vec3 temp;

    // Project triangle to dominant plane
    if (dominant_axis == 0) {
        // x-axis is depth
        for (int i = 0; i < gl_in.length(); i++)
        {
            temp.x = vertex[i].z;
            temp.z = -vertex[i].x; 
            
            vertex[i].xz = temp.xz; 
        }
    
    } else if (dominant_axis == 1) {
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

    vec3 triangleNormal = normalize(cross(vertex[1].xyz - vertex[0].xyz, vertex[2].xyz - vertex[0].xyz));
    
    // Change triangle winding, so normal points to "camera"
    if (dot(triangleNormal, vec3(0.0, 0.0, 1.0)) < 0.0)
    {
        vec4 vertexTemp = vertex[2];
        
        vertex[2] = vertex[1];
    
        vertex[1] = vertexTemp;
    }
    vec2 half_pixel = vec2(1.0 / voxel_dimension, 1.0 / voxel_dimension);

    vec4 aabb = define_aabb(vertex, half_pixel);
    fragment_aabb = aabb;

    vec4 trianglePlane;
        
    trianglePlane.xyz = normalize(cross(vertex[1].xyz - vertex[0].xyz, vertex[2].xyz - vertex[0].xyz));
        
    trianglePlane.w = -dot(vertex[0].xyz, trianglePlane.xyz);
    
    if (trianglePlane.z == 0.0) {
        return;
    }

    vec3 plane[3];
	       
    for (int i = 0; i < gl_in.length(); i++) {
      plane[i] = cross(vertex[i].xyw, vertex[(i + 2) % 3].xyw);
		
      plane[i].z -= dot(half_pixel, abs(plane[i].xy));
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
      fragment_position = intersect[i].xyz;
      fragment_normal = normal[i];
      fragment_texture_coordinates = tex_coordinates[i];
      EmitVertex();
    }

    EndPrimitive();
}
