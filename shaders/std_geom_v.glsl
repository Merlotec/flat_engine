#version 150 core

in vec4 a_Pos;
in vec3 a_Color;

 uniform Transform {

        mat4 model_Transform;
        mat4 view_Transform;
        mat4 projection_Transform;

    };

out vec4 v_Color;

void main() {
    v_Color = vec4(a_Color, 1.0);
    gl_Position = model_Transform * view_Transform * projection_Transform * a_Pos;
}