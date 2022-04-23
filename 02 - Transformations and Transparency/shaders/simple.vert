#version 460 core

layout(location=1) in vec3 position;

layout(location=2) in vec4 in_color;
layout(location=2) out vec4 out_color;

uniform mat4x4 affine_transformation_matrix;
uniform float elapsed;

void main() 
{ 
    float a = 0;
    float b = 0;
    float c = 0;
    float d = 0;
    float e = 0;
    float f = 0;

    mat4x4 identity_matrix = {
        { a + 1, d + 0, 0, 0 },
        { b + 0, e + 1, 0, 0 },
        {     0,     0, 1, 0 },
        { c + 0, f + 0, 0, 1 },
    };
    
    gl_Position = identity_matrix * vec4(position, 1.0f);

    gl_Position =  affine_transformation_matrix * vec4(position, 1.0f);
    
    // Transfer color to fragment shader
    out_color = in_color;
}