#include "d3d11_shader_common.hlsl"

PShaderInput VShader(VShaderInput v_shader_input)
{
    PShaderInput p_shader_input = (PShaderInput)0;

    p_shader_input.position = v_shader_input.position;
    p_shader_input.main_texture_coord = v_shader_input.main_texture_coord;

    return p_shader_input;
}
