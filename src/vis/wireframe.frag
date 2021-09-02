#version 450
layout(location = 1) in vec3 v_Color;

layout(location = 0) out vec4 o_Target;


void main() {
    o_Target = vec4(v_Color, 1.0);
}
