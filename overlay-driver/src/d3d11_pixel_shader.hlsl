#include "d3d11_shader_common.hlsl"

Texture2D main_texture;
SamplerState main_texture_sampler
{
    Filter = MIN_MAG_MIP_LINEAR;
    AddressU = Wrap;
    AddressV = Wrap;
};


float4 PShader(PShaderInput p_shader_input) : SV_Target
{
    return main_texture.Sample(main_texture_sampler, p_shader_input.main_texture_coord).rgba;
}
