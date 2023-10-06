use winapi::um::d3d11::ID3D11DeviceContext;
use winapi::um::d3d11::D3D11_IA_VERTEX_INPUT_RESOURCE_SLOT_COUNT;
use winapi::um::d3d11::D3D11_SIMULTANEOUS_RENDER_TARGET_COUNT;

use crate::*;

pub unsafe fn apply_d3d11_state(
    d3d11_device_context: *mut ID3D11DeviceContext,
    d3d11_state_block: &D3D11StateBlock,
) {
    d3d11_device_context
        .as_ref()
        .unwrap()
        .RSSetState(d3d11_state_block.rasterizer_state);

    d3d11_device_context.as_ref().unwrap().RSSetViewports(
        d3d11_state_block.viewports_count,
        d3d11_state_block.viewports.as_ptr(),
    );

    d3d11_device_context.as_ref().unwrap().OMSetRenderTargets(
        D3D11_SIMULTANEOUS_RENDER_TARGET_COUNT,
        d3d11_state_block.render_target_views.as_ptr(),
        d3d11_state_block.depth_stencil_view,
    );

    d3d11_device_context.as_ref().unwrap().OMSetBlendState(
        d3d11_state_block.blend_state,
        &d3d11_state_block.blend_factors,
        d3d11_state_block.blend_sample_mask,
    );

    d3d11_device_context
        .as_ref()
        .unwrap()
        .IASetInputLayout(d3d11_state_block.input_layout);

    d3d11_device_context.as_ref().unwrap().IASetIndexBuffer(
        d3d11_state_block.index_buffer,
        d3d11_state_block.index_buffer_format,
        d3d11_state_block.index_buffer_offset,
    );

    d3d11_device_context
        .as_ref()
        .unwrap()
        .IASetPrimitiveTopology(d3d11_state_block.primitive_topology);

    d3d11_device_context.as_ref().unwrap().IASetVertexBuffers(
        0,
        D3D11_IA_VERTEX_INPUT_RESOURCE_SLOT_COUNT,
        d3d11_state_block.vertex_buffers.as_ptr(),
        d3d11_state_block.vertex_buffers_strides.as_ptr(),
        d3d11_state_block.vertex_buffers_offsets.as_ptr(),
    );
}
