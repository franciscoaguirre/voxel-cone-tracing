#version 430 core

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

uniform mat4 x_ortho_projection;
uniform mat4 y_ortho_projection;
uniform mat4 z_ortho_projection;
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

vec4 define_aabb(vec4 points[3]) {
    vec4 aabb;

    aabb.xy = points[0].xy;
    aabb.zw = points[0].xy;

    aabb.xy = min(points[1].xy, aabb.xy);
    aabb.zw = max(points[1].xy, aabb.zw);

    aabb.xy = min(points[2].xy, aabb.xy);
    aabb.zw = max(points[2].xy, aabb.zw);

    return aabb;
}

void main() {
    vec3 triangle_normal = normalize(cross(vertex_position[1]-vertex_position[0],
                                           vertex_position[2]-vertex_position[0]));

    int dominant_axis = biggest_component(triangle_normal);
    fragment_dominant_axis = dominant_axis;
    mat4 projection;

    if(dominant_axis == 0) {
      projection = x_ortho_projection;
    } else if(dominant_axis == 1) {
      projection = y_ortho_projection;
    } else {
      projection = z_ortho_projection;
    }

    vec4 projected_vertices[3];
    projected_vertices[0] = projection * gl_in[0].gl_Position;
    projected_vertices[1] = projection * gl_in[1].gl_Position;
    projected_vertices[2] = projection * gl_in[2].gl_Position;

    vec2 half_pixel = vec2(1.0 / voxel_dimension, 1.0 / voxel_dimension) / 2.0;
    float pl = sqrt(2) / voxel_dimension;

    vec4 aabb = define_aabb(projected_vertices);
    fragment_aabb = aabb;

    vec3 edge_first_second = vec3( projected_vertices[1].xy - projected_vertices[0].xy, 0 );
    vec3 edge_second_third = vec3( projected_vertices[2].xy - projected_vertices[1].xy, 0 );
    vec3 edge_third_first = vec3( projected_vertices[0].xy - projected_vertices[2].xy, 0 );
    vec3 n0 = cross( edge_first_second, vec3(0,0,1) ); // TODO: Why is this vec3(0,0,1) if we could project to another axis?
    vec3 n1 = cross( edge_second_third, vec3(0,0,1) );
    vec3 n2 = cross( edge_third_first, vec3(0,0,1) );

    //dilate the triangle
    projected_vertices[0].xy += pl * (
        (edge_third_first.xy / dot(edge_third_first.xy, n0.xy)) + (edge_first_second.xy/dot(edge_first_second.xy,n2.xy))
    );
    projected_vertices[1].xy += pl * (
        (edge_first_second.xy/dot(edge_first_second.xy,n1.xy)) + (edge_second_third.xy/dot(edge_second_third.xy,n0.xy))
    );
    projected_vertices[2].xy += pl * (
        (edge_second_third.xy / dot(edge_second_third.xy,n2.xy)) + (edge_third_first.xy/dot(edge_third_first.xy,n1.xy))
    );

      //gl_Position = proj * gl_in[0].gl_Position;
    gl_Position = projected_vertices[0];
    fragment_position = projected_vertices[0].xyz;
    fragment_normal = normal[0];
    fragment_texture_coordinates = tex_coordinates[0];
    EmitVertex();

    //gl_Position = proj * gl_in[1].gl_Position;
    gl_Position = projected_vertices[1];
    fragment_position = projected_vertices[1].xyz;
    fragment_normal = normal[1];
    fragment_texture_coordinates = tex_coordinates[1];
    EmitVertex();

    //gl_Position = proj * gl_in[2].gl_Position;
    gl_Position = projected_vertices[2];
    fragment_position = projected_vertices[2].xyz;
    fragment_normal = normal[2];
    fragment_texture_coordinates = tex_coordinates[2];
    EmitVertex();

    EndPrimitive();
}
