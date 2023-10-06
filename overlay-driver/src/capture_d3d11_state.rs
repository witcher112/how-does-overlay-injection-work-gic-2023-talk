use winapi::um::d3d11::ID3D11DeviceContext;
use winapi::um::d3d11::D3D11_IA_VERTEX_INPUT_RESOURCE_SLOT_COUNT;
use winapi::um::d3d11::D3D11_SIMULTANEOUS_RENDER_TARGET_COUNT;
use winapi::um::d3d11::D3D11_VIEWPORT_AND_SCISSORRECT_OBJECT_COUNT_PER_PIPELINE;

use crate::*;

pub unsafe fn capture_d3d11_state(
    d3d11_device_context: *mut ID3D11DeviceContext,
) -> D3D11StateBlock {
    let mut d3d11_state_block: D3D11StateBlock = std::mem::zeroed();

    d3d11_device_context
        .as_ref()
        .unwrap()
        .RSGetState(&mut d3d11_state_block.rasterizer_state);

    d3d11_state_block.viewports_count = D3D11_VIEWPORT_AND_SCISSORRECT_OBJECT_COUNT_PER_PIPELINE;

    d3d11_device_context.as_ref().unwrap().RSGetViewports(
        &mut d3d11_state_block.viewports_count,
        d3d11_state_block.viewports.as_mut_ptr(),
    );

    d3d11_device_context.as_ref().unwrap().OMGetRenderTargets(
        D3D11_SIMULTANEOUS_RENDER_TARGET_COUNT,
        d3d11_state_block.render_target_views.as_mut_ptr(),
        &mut d3d11_state_block.depth_stencil_view,
    );

    d3d11_device_context.as_ref().unwrap().OMGetBlendState(
        &mut d3d11_state_block.blend_state,
        &mut d3d11_state_block.blend_factors,
        &mut d3d11_state_block.blend_sample_mask,
    );

    d3d11_device_context
        .as_ref()
        .unwrap()
        .IAGetInputLayout(&mut d3d11_state_block.input_layout);

    d3d11_device_context.as_ref().unwrap().IAGetIndexBuffer(
        &mut d3d11_state_block.index_buffer,
        &mut d3d11_state_block.index_buffer_format,
        &mut d3d11_state_block.index_buffer_offset,
    );

    d3d11_device_context
        .as_ref()
        .unwrap()
        .IAGetPrimitiveTopology(&mut d3d11_state_block.primitive_topology);

    d3d11_device_context.as_ref().unwrap().IAGetVertexBuffers(
        0,
        D3D11_IA_VERTEX_INPUT_RESOURCE_SLOT_COUNT,
        d3d11_state_block.vertex_buffers.as_mut_ptr(),
        d3d11_state_block.vertex_buffers_strides.as_mut_ptr(),
        d3d11_state_block.vertex_buffers_offsets.as_mut_ptr(),
    );

    // TODO: Consider storing GSSetShader, VSSetShader, PSSetShader, PSSetShaderResources

    return d3d11_state_block;
}
