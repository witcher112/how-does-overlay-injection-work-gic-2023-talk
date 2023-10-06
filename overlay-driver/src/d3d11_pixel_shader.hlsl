#include "d3d11_shader_common.hlsl"

float4 PShader(PShaderInput p_shader_input) : SV_Target
{
    return float4(1.0, 0.0, 0.0, 1.0);
}
