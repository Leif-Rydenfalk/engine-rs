#version 460

layout (location = 0) in vec4 pos;
layout (location = 1) in vec4 color;


layout (location = 0) out vec4 o_color;
void main() {
    o_color = color;
    gl_Position = vec4(pos.xy * 0.1 * float(gl_InstanceIndex), float(gl_InstanceIndex) * 0.1, 1.0) + vec4(float(gl_InstanceIndex) * 0.1, 0.0, 0.0, 0.0);
}