#version 460 core

out vec4 color;

layout(location=2) in vec4 in_color;

layout(location=3) in vec3 normal;

vec3 light_direction = normalize(vec3(0.8, -0.5, 0.6));
float max_shadow = 0.3;

void main()
{
    // Determine brightness of vertexbased on its normal
    float light = max(max_shadow, dot(normal, -light_direction));

    // Apply this light to vertex color
    color = vec4(in_color.rgb * light, in_color.a);
}