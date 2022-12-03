#version 330 core

layout (location = 0) in vec3 a_position;
layout (location = 1) in vec2 a_uvs;

out vec2 o_uvs;

uniform mat4 u_projection;
uniform mat4 u_model;

void main() {
    o_uvs = a_uvs;
    gl_Position = u_projection * u_model * vec4(a_position.x, a_position.y, a_position.z, 1.0);
}
