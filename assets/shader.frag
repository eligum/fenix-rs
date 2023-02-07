#version 450 core

in vec2 v_texcoord;

out vec4 frag_color;

uniform sampler2D color_map0;
uniform sampler2D color_map1;

void main()
{
    frag_color = mix(
        texture(color_map0, v_texcoord), 
        texture(color_map1, v_texcoord),
        0.25
    );
}
