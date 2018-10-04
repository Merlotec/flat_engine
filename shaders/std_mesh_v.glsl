#version 150 core

    in vec3 a_Pos;

    in vec2 a_Uv;

    out vec2 v_Uv;

    uniform Transform {

        mat4 model_Transform;
        mat4 view_Transform;
        mat4 projection_Transform;

    };

    void main() {

        v_Uv = a_Uv;

        gl_Position = model_Transform * view_Transform * projection_Transform * vec4(a_Pos, 1.0);

    }