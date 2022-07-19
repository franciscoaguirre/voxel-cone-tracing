#version 430 core

layout (location = 0) out vec4 FragColor;

in flat int geom_vertex_id;
in flat vec3 fragment_normal;

void main() {
    //FragColor = vec4(256 - (geom_vertex_id % 256), (geom_vertex_id % 256), 256 - (geom_vertex_id % 256), 1.0);
    // Hecho a ojo, la verdad que funciona de milagro
    //vec3 point_of_view = vec3(0.0,0.0,-1.0);
    //float lol = abs(dot(normalize(fragment_normal), point_of_view)); 
    //FragColor = vec4(lol * normalize(vec3(0.4,0.4,0.4)), 1.0);
    FragColor = vec4(1.0,1.0,1.0,1.0);
}

//#version 430 core

//layout (location = 0) out vec4 FragColor;

//in flat int geom_vertex_id;
//in flat vec3 fragment_normal;

//void main() {
    //FragColor = vec4(256 - (geom_vertex_id % 256), (geom_vertex_id % 256), 256 - (geom_vertex_id % 256), 1.0);
     //Hecho a ojo, la verdad que funciona de milagro
    //vec3 point_of_view = vec3(0.0,0.0,1.0);
    //float diffuse_factor = dot(normalize(fragment_normal), point_of_view); 
    //FragColor = vec4(vec3(1.0,1.0,1.0) * abs(diffuse_factor), 1.0);
//}
