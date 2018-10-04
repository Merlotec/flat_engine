#version 150 core

in vec2 a_Pos;
in vec2 a_Uv;

 uniform Transform {

    mat4 model_Transform;
    mat4 view_Transform;
    mat4 projection_Transform;

 };

out vec2 v_Uv;

void main() {
    v_Uv = a_Uv;
    gl_Position = projection_Transform * view_Transform * model_Transform * vec4(a_Pos, 0.0, 1.0);
}