#include "d3d11_shader_common.hlsl"

PShaderInput VShader(VShaderInput v_shader_input)
{
    PShaderInput p_shader_input = (PShaderInput)0;

    p_shader_input.position = v_shader_input.position;

    return p_shader_input;
}
