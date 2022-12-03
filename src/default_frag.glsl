#version 330 core

out vec4 o_color;

in vec2 a_uvs;

uniform vec4 u_color;
uniform sampler2D u_sampler;

void main() {
    o_color = texture(u_sampler, a_uvs) * u_color;
}
