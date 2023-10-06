use winapi::um::d3d11::D3D11_SIMULTANEOUS_RENDER_TARGET_COUNT;

use crate::*;

pub unsafe fn release_d3d11_state(d3d11_state_block: &D3D11StateBlock) {
    if !d3d11_state_block.rasterizer_state.is_null() {
        d3d11_state_block
            .rasterizer_state
            .as_ref()
            .unwrap()
            .Release();
    }

    for d3d11_render_target_view_index in 0..D3D11_SIMULTANEOUS_RENDER_TARGET_COUNT {
        let d3d11_render_target_view = d3d11_state_block
            .render_target_views
            .get(d3d11_render_target_view_index as usize)
            .unwrap();

        if !d3d11_render_target_view.is_null() {
            d3d11_render_target_view.as_ref().unwrap().Release();
        }
    }

    if !d3d11_state_block.depth_stencil_view.is_null() {
        d3d11_state_block
            .depth_stencil_view
            .as_ref()
            .unwrap()
            .Release();
    }

    if !d3d11_state_block.blend_state.is_null() {
        d3d11_state_block.blend_state.as_ref().unwrap().Release();
    }

    if !d3d11_state_block.input_layout.is_null() {
        d3d11_state_block.input_layout.as_ref().unwrap().Release();
    }

    if !d3d11_state_block.index_buffer.is_null() {
        d3d11_state_block.index_buffer.as_ref().unwrap().Release();
    }

    for d3d11_vertex_buffer_index in 0..D3D11_SIMULTANEOUS_RENDER_TARGET_COUNT {
        let d3d11_vertex_buffer = d3d11_state_block
            .vertex_buffers
            .get(d3d11_vertex_buffer_index as usize)
            .unwrap();

        if !d3d11_vertex_buffer.is_null() {
            d3d11_vertex_buffer.as_ref().unwrap().Release();
        }
    }
}
