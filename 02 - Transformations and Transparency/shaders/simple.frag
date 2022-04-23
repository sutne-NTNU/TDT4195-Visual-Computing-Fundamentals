#version 460 core

layout(location=2) in vec4 in_color;

out vec4 color;
 
void main()
{
    color = in_color;
}