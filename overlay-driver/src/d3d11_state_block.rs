use winapi::shared::dxgiformat::DXGI_FORMAT;
use winapi::shared::minwindef::UINT;
use winapi::um::d3d11::ID3D11BlendState;
use winapi::um::d3d11::ID3D11Buffer;
use winapi::um::d3d11::ID3D11DepthStencilView;
use winapi::um::d3d11::ID3D11InputLayout;
use winapi::um::d3d11::ID3D11RasterizerState;
use winapi::um::d3d11::ID3D11RenderTargetView;
use winapi::um::d3d11::D3D11_IA_VERTEX_INPUT_RESOURCE_SLOT_COUNT;
use winapi::um::d3d11::D3D11_PRIMITIVE_TOPOLOGY;
use winapi::um::d3d11::D3D11_SIMULTANEOUS_RENDER_TARGET_COUNT;
use winapi::um::d3d11::D3D11_VIEWPORT;
use winapi::um::d3d11::D3D11_VIEWPORT_AND_SCISSORRECT_OBJECT_COUNT_PER_PIPELINE;

pub struct D3D11StateBlock {
    pub rasterizer_state: *mut ID3D11RasterizerState,
    pub viewports_count: UINT,
    pub viewports:
        [D3D11_VIEWPORT; D3D11_VIEWPORT_AND_SCISSORRECT_OBJECT_COUNT_PER_PIPELINE as usize],
    pub render_target_views:
        [*mut ID3D11RenderTargetView; D3D11_SIMULTANEOUS_RENDER_TARGET_COUNT as usize],
    pub depth_stencil_view: *mut ID3D11DepthStencilView,
    pub blend_state: *mut ID3D11BlendState,
    pub blend_factors: [f32; 4],
    pub blend_sample_mask: u32,
    pub input_layout: *mut ID3D11InputLayout,
    pub index_buffer: *mut ID3D11Buffer,
    pub index_buffer_format: DXGI_FORMAT,
    pub index_buffer_offset: UINT,
    pub primitive_topology: D3D11_PRIMITIVE_TOPOLOGY,
    pub vertex_buffers: [*mut ID3D11Buffer; D3D11_IA_VERTEX_INPUT_RESOURCE_SLOT_COUNT as usize],
    pub vertex_buffers_strides: [UINT; D3D11_IA_VERTEX_INPUT_RESOURCE_SLOT_COUNT as usize],
    pub vertex_buffers_offsets: [UINT; D3D11_IA_VERTEX_INPUT_RESOURCE_SLOT_COUNT as usize],
}
