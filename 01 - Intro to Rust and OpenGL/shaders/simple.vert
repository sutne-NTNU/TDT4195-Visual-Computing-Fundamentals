#version 450 core

in vec3 position;

void main() 
{ 
    // normal
    gl_Position = vec4( position, 1.0f ); 

    // mirrored horizontally and vertically
    gl_Position = vec4( -position.x, -position.y, position.z, 1.0f ); 
}