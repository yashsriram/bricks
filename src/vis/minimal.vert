#version 450

layout(location = 0) in vec3 Vertex_Position;
layout(location = 1) in vec4 Vertex_Color;

layout(location = 1) out vec4 v_Color;

layout(set = 0, binding = 0) uniform CameraViewProj {
    mat4 ViewProj;
};

layout(set = 1, binding = 0) uniform Transform {
    mat4 Model;
};

void main() {
    v_Color = Vertex_Color;
    vec3 v_Position = (Model * vec4(Vertex_Position, 1.0)).xyz;
    gl_Position = ViewProj * vec4(v_Position, 1.0);
}
