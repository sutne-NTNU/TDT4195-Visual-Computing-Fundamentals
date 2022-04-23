#version 460 core

layout(location=1) in vec3 position;

layout(location=2) in vec4 in_color;
layout(location=2) out vec4 out_color;

layout(location=3) in vec3 in_normal;
layout(location=3) out vec3 out_normal;

uniform mat4x4 mvp_transformation_matrix;
uniform mat4x4 model_transformation_matrix;

void main() 
{ 
    // Transform vertices
    gl_Position =  mvp_transformation_matrix * vec4(position, 1.0f);
    
    // Transform and pass normals to fragment shader
    out_normal = normalize(mat3(model_transformation_matrix) * in_normal);

    // Pass color to fragment shader
    out_color = in_color;
}