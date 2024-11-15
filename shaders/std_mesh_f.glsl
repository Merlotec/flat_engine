#version 150 core

    in vec2 v_Uv;

    out vec4 Target0;

    uniform sampler2D t_Texture;

    void main() {

        vec4 tex = texture(t_Texture, v_Uv);

        float blend = dot(v_Uv-vec2(0.5,0.5), v_Uv-vec2(0.5,0.5));

        Target0 = mix(tex, vec4(0.0,0.0,0.0,0.0), blend*1.0);

    }