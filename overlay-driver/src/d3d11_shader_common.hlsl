struct VShaderInput
{
    float4 position : POSITION;
    float2 main_texture_coord : TEXCOORD;
};

struct PShaderInput
{
    float4 position : SV_POSITION;
    float2 main_texture_coord : TEXCOORD0;
};
