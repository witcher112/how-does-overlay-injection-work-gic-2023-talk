use std::ffi::CString;
use std::ptr::copy;
use std::ptr::null_mut;
use std::sync::Mutex;
use winapi::ctypes::c_void;
use winapi::shared::dxgi::IDXGISwapChain;
use winapi::shared::dxgiformat::DXGI_FORMAT_R32G32B32_FLOAT;
use winapi::shared::minwindef::FALSE;
use winapi::shared::minwindef::TRUE;
use winapi::um::d3d11::ID3D11BlendState;
use winapi::um::d3d11::ID3D11Buffer;
use winapi::um::d3d11::ID3D11Device;
use winapi::um::d3d11::ID3D11DeviceContext;
use winapi::um::d3d11::ID3D11InputLayout;
use winapi::um::d3d11::ID3D11PixelShader;
use winapi::um::d3d11::ID3D11RasterizerState;
use winapi::um::d3d11::ID3D11RenderTargetView;
use winapi::um::d3d11::ID3D11Texture2D;
use winapi::um::d3d11::ID3D11VertexShader;
use winapi::um::d3d11::D3D11_BIND_VERTEX_BUFFER;
use winapi::um::d3d11::D3D11_BLEND_DESC;
use winapi::um::d3d11::D3D11_BLEND_DEST_ALPHA;
use winapi::um::d3d11::D3D11_BLEND_INV_SRC_ALPHA;
use winapi::um::d3d11::D3D11_BLEND_OP_ADD;
use winapi::um::d3d11::D3D11_BLEND_SRC_ALPHA;
use winapi::um::d3d11::D3D11_BUFFER_DESC;
use winapi::um::d3d11::D3D11_COLOR_WRITE_ENABLE_ALL;
use winapi::um::d3d11::D3D11_CPU_ACCESS_WRITE;
use winapi::um::d3d11::D3D11_CULL_NONE;
use winapi::um::d3d11::D3D11_FILL_SOLID;
use winapi::um::d3d11::D3D11_INPUT_ELEMENT_DESC;
use winapi::um::d3d11::D3D11_INPUT_PER_VERTEX_DATA;
use winapi::um::d3d11::D3D11_MAPPED_SUBRESOURCE;
use winapi::um::d3d11::D3D11_MAP_WRITE_DISCARD;
use winapi::um::d3d11::D3D11_RASTERIZER_DESC;
use winapi::um::d3d11::D3D11_TEXTURE2D_DESC;
use winapi::um::d3d11::D3D11_USAGE_DYNAMIC;
use winapi::um::d3d11::D3D11_VIEWPORT;
use winapi::um::d3dcommon::D3D11_PRIMITIVE_TOPOLOGY_TRIANGLELIST;
use winapi::Interface;

pub use crate::*;

pub struct D3D11OverlayState {
    pub dxgi_swap_chain: *mut IDXGISwapChain,
    pub dxgi_swap_chain_back_buffer: *mut ID3D11Texture2D,
    pub d3d11_device: *mut ID3D11Device,
    pub d3d11_device_context: *mut ID3D11DeviceContext,
    pub d3d11_render_target_view: *mut ID3D11RenderTargetView,
    pub d3d11_blend_state: *mut ID3D11BlendState,
    pub d3d11_vertex_shader: *mut ID3D11VertexShader,
    pub d3d11_pixel_shader: *mut ID3D11PixelShader,
    pub d3d11_input_layout: *mut ID3D11InputLayout,
    pub d3d11_vertex_buffer: *mut ID3D11Buffer,
    pub d3d11_rasterizer_state: *mut ID3D11RasterizerState,
    pub d3d11_state_block: D3D11StateBlock,
}

unsafe impl Send for D3D11OverlayState {}

lazy_static::lazy_static! {
  pub static ref D3D11_OVERLAY_STATE: Mutex<Option<D3D11OverlayState>> = Mutex::new(None);
}

#[allow(dead_code)]
struct D3D11OverlayVertexData {
    pub position_x: f32,
    pub position_y: f32,
    pub position_z: f32,
}

pub unsafe fn init_d3d11_overlay_if_not_initialized(dxgi_swap_chain: *mut IDXGISwapChain) {
    let mut d3d11_overlay_state_guard = D3D11_OVERLAY_STATE.lock().unwrap();

    let d3d11_overlay_state = &mut *d3d11_overlay_state_guard;

    if d3d11_overlay_state.is_some() {
        return;
    }

    let mut d3d11_device: *mut c_void = null_mut();
    let hr = dxgi_swap_chain
        .as_ref()
        .unwrap()
        .GetDevice(&ID3D11Device::uuidof(), &mut d3d11_device);
    if hr != 0 {
        panic!();
    }
    let d3d11_device = d3d11_device as *mut ID3D11Device;
    let _release_d3d11_device_guard = scopeguard::guard((), |_| {
        d3d11_device.as_ref().unwrap().Release();
    });

    let mut d3d11_device_context: *mut ID3D11DeviceContext = null_mut();
    d3d11_device
        .as_ref()
        .unwrap()
        .GetImmediateContext(&mut d3d11_device_context);
    let d3d11_device_context = d3d11_device_context;
    let _release_d3d11_device_context_guard = scopeguard::guard((), |_| {
        d3d11_device_context.as_ref().unwrap().Release();
    });

    let d3d11_retaining_state_block = capture_d3d11_state(d3d11_device_context);
    let _release_d3d11_retaining_state_block_guard = scopeguard::guard((), |_| {
        release_d3d11_state(&d3d11_retaining_state_block);
    });
    let _apply_d3d11_retaining_state_block_guard = scopeguard::guard((), |_| {
        apply_d3d11_state(d3d11_device_context, &d3d11_retaining_state_block);
    });

    let mut dxgi_swap_chain_back_buffer: *mut c_void = null_mut();
    let hr = dxgi_swap_chain.as_ref().unwrap().GetBuffer(
        0,
        &ID3D11Texture2D::uuidof(),
        &mut dxgi_swap_chain_back_buffer,
    );
    if hr != 0 {
        panic!();
    }
    let dxgi_swap_chain_back_buffer = dxgi_swap_chain_back_buffer as *mut ID3D11Texture2D;
    let _release_dxgi_swap_chain_back_buffer_guard = scopeguard::guard((), |_| {
        dxgi_swap_chain_back_buffer.as_ref().unwrap().Release();
    });

    let mut dxgi_swap_chain_back_buffer_desc: D3D11_TEXTURE2D_DESC = std::mem::zeroed();
    dxgi_swap_chain_back_buffer
        .as_ref()
        .unwrap()
        .GetDesc(&mut dxgi_swap_chain_back_buffer_desc);

    let mut d3d11_viewport: D3D11_VIEWPORT = std::mem::zeroed();
    d3d11_viewport.Width = dxgi_swap_chain_back_buffer_desc.Width as f32;
    d3d11_viewport.Height = dxgi_swap_chain_back_buffer_desc.Height as f32;
    d3d11_viewport.MinDepth = 0.0;
    d3d11_viewport.MaxDepth = 1.0;
    d3d11_viewport.TopLeftX = 0.0;
    d3d11_viewport.TopLeftY = 0.0;

    d3d11_device_context
        .as_ref()
        .unwrap()
        .RSSetViewports(1, &d3d11_viewport);

    let mut d3d11_render_target_view: *mut ID3D11RenderTargetView = null_mut();
    let hr = d3d11_device.as_ref().unwrap().CreateRenderTargetView(
        dxgi_swap_chain_back_buffer as *mut _,
        null_mut(),
        &mut d3d11_render_target_view,
    );
    if hr != 0 {
        panic!();
    }
    let d3d11_render_target_view = d3d11_render_target_view;
    let _release_d3d11_render_target_view_guard = scopeguard::guard((), |_| {
        d3d11_render_target_view.as_ref().unwrap().Release();
    });

    d3d11_device_context.as_ref().unwrap().OMSetRenderTargets(
        1,
        &d3d11_render_target_view,
        null_mut(),
    );

    let mut d3d11_blend_desc: D3D11_BLEND_DESC = std::mem::zeroed();
    d3d11_blend_desc.RenderTarget[0].BlendEnable = TRUE;
    d3d11_blend_desc.RenderTarget[0].SrcBlend = D3D11_BLEND_SRC_ALPHA;
    d3d11_blend_desc.RenderTarget[0].DestBlend = D3D11_BLEND_INV_SRC_ALPHA;
    d3d11_blend_desc.RenderTarget[0].BlendOp = D3D11_BLEND_OP_ADD;
    d3d11_blend_desc.RenderTarget[0].SrcBlendAlpha = D3D11_BLEND_SRC_ALPHA;
    d3d11_blend_desc.RenderTarget[0].DestBlendAlpha = D3D11_BLEND_DEST_ALPHA;
    d3d11_blend_desc.RenderTarget[0].BlendOpAlpha = D3D11_BLEND_OP_ADD;
    d3d11_blend_desc.RenderTarget[0].RenderTargetWriteMask = D3D11_COLOR_WRITE_ENABLE_ALL as u8;

    let mut d3d11_blend_state: *mut ID3D11BlendState = null_mut();
    let hr = d3d11_device
        .as_ref()
        .unwrap()
        .CreateBlendState(&d3d11_blend_desc, &mut d3d11_blend_state);
    if hr != 0 {
        panic!();
    }
    let d3d11_blend_state = d3d11_blend_state;
    let _release_d3d11_blend_state_guard = scopeguard::guard((), |_| {
        d3d11_blend_state.as_ref().unwrap().Release();
    });

    d3d11_device_context.as_ref().unwrap().OMSetBlendState(
        d3d11_blend_state,
        &std::mem::zeroed(),
        0xffff_ffff,
    );

    let mut d3d11_vertex_shader: *mut ID3D11VertexShader = null_mut();
    let hr = d3d11_device.as_ref().unwrap().CreateVertexShader(
        D3D11_VERTEX_SHADER_OBJ_BYTES.as_ptr() as *const c_void,
        D3D11_VERTEX_SHADER_OBJ_BYTES.len(),
        null_mut(),
        &mut d3d11_vertex_shader,
    );
    if hr != 0 {
        panic!();
    }
    let d3d11_vertex_shader = d3d11_vertex_shader;
    let _release_d3d11_vertex_shader_guard = scopeguard::guard((), |_| {
        d3d11_vertex_shader.as_ref().unwrap().Release();
    });

    let d3d11_pixel_shader_bytes_ptr = D3D11_PIXEL_SHADER_OBJ_BYTES.as_ptr() as _;
    let d3d11_pixel_shader_bytes_len = D3D11_PIXEL_SHADER_OBJ_BYTES.len();

    let mut d3d11_pixel_shader: *mut ID3D11PixelShader = null_mut();
    let hr = d3d11_device.as_ref().unwrap().CreatePixelShader(
        d3d11_pixel_shader_bytes_ptr,
        d3d11_pixel_shader_bytes_len,
        null_mut(),
        &mut d3d11_pixel_shader,
    );
    if hr != 0 {
        panic!();
    }
    let d3d11_pixel_shader = d3d11_pixel_shader;
    let _release_d3d11_pixel_shader_guard = scopeguard::guard((), |_| {
        d3d11_pixel_shader.as_ref().unwrap().Release();
    });

    d3d11_device_context
        .as_ref()
        .unwrap()
        .VSSetShader(d3d11_vertex_shader, null_mut(), 0);
    d3d11_device_context
        .as_ref()
        .unwrap()
        .PSSetShader(d3d11_pixel_shader, null_mut(), 0);

    let d3d11_input_element_0_semantic_name = CString::new("POSITION").unwrap();
    let d3d11_input_elements_desc: &[D3D11_INPUT_ELEMENT_DESC] = &[D3D11_INPUT_ELEMENT_DESC {
        SemanticName: d3d11_input_element_0_semantic_name.as_ptr(),
        SemanticIndex: 0,
        Format: DXGI_FORMAT_R32G32B32_FLOAT,
        InputSlot: 0,
        AlignedByteOffset: 0,
        InputSlotClass: D3D11_INPUT_PER_VERTEX_DATA,
        InstanceDataStepRate: 0,
    }];

    let mut d3d11_input_layout: *mut ID3D11InputLayout = null_mut();
    let hr = d3d11_device.as_ref().unwrap().CreateInputLayout(
        d3d11_input_elements_desc.as_ptr(),
        1,
        D3D11_VERTEX_SHADER_OBJ_BYTES.as_ptr() as *const c_void,
        D3D11_VERTEX_SHADER_OBJ_BYTES.len(),
        &mut d3d11_input_layout,
    );
    if hr != 0 {
        panic!();
    }
    let d3d11_input_layout = d3d11_input_layout;
    let _release_d3d11_input_layout_guard = scopeguard::guard((), |_| {
        d3d11_input_layout.as_ref().unwrap().Release();
    });

    d3d11_device_context
        .as_ref()
        .unwrap()
        .IASetInputLayout(d3d11_input_layout);

    d3d11_device_context
        .as_ref()
        .unwrap()
        .IASetPrimitiveTopology(D3D11_PRIMITIVE_TOPOLOGY_TRIANGLELIST);

    let d3d11_vertex_buffer_data: &[D3D11OverlayVertexData] = &[
        D3D11OverlayVertexData {
            position_x: 0.0,
            position_y: 0.5,
            position_z: 0.0,
        },
        D3D11OverlayVertexData {
            position_x: -0.5,
            position_y: -0.5,
            position_z: 0.0,
        },
        D3D11OverlayVertexData {
            position_x: 0.5,
            position_y: -0.5,
            position_z: 0.0,
        },
    ];

    let d3d11_vertex_buffer_data_size = std::mem::size_of_val(d3d11_vertex_buffer_data);

    let mut d3d11_vertex_buffer_desc: D3D11_BUFFER_DESC = std::mem::zeroed();
    d3d11_vertex_buffer_desc.Usage = D3D11_USAGE_DYNAMIC;
    d3d11_vertex_buffer_desc.ByteWidth = d3d11_vertex_buffer_data_size as u32;
    d3d11_vertex_buffer_desc.BindFlags = D3D11_BIND_VERTEX_BUFFER;
    d3d11_vertex_buffer_desc.CPUAccessFlags = D3D11_CPU_ACCESS_WRITE;

    let mut d3d11_vertex_buffer: *mut ID3D11Buffer = null_mut();
    let hr = d3d11_device.as_ref().unwrap().CreateBuffer(
        &d3d11_vertex_buffer_desc,
        null_mut(),
        &mut d3d11_vertex_buffer,
    );
    if hr != 0 {
        panic!();
    }
    let d3d11_vertex_buffer = d3d11_vertex_buffer;
    let _release_d3d11_vertex_buffer_guard = scopeguard::guard((), |_| {
        d3d11_vertex_buffer.as_ref().unwrap().Release();
    });

    let mut d3d11_mapped_vertex_buffer: D3D11_MAPPED_SUBRESOURCE = std::mem::zeroed();
    let hr = d3d11_device_context.as_ref().unwrap().Map(
        d3d11_vertex_buffer as *mut _,
        0,
        D3D11_MAP_WRITE_DISCARD,
        0,
        &mut d3d11_mapped_vertex_buffer,
    );
    if hr != 0 {
        panic!();
    }
    let d3d11_mapped_vertex_buffer = d3d11_mapped_vertex_buffer;

    copy(
        d3d11_vertex_buffer_data.as_ptr() as *const c_void,
        d3d11_mapped_vertex_buffer.pData,
        d3d11_vertex_buffer_data_size,
    );

    d3d11_device_context
        .as_ref()
        .unwrap()
        .Unmap(d3d11_vertex_buffer as *mut _, 0);

    let d3d11_vertex_buffer_stride = std::mem::size_of::<D3D11OverlayVertexData>() as u32;
    let d3d11_vertex_buffer_offset = 0;

    d3d11_device_context.as_ref().unwrap().IASetVertexBuffers(
        0,
        1,
        &d3d11_vertex_buffer,
        &d3d11_vertex_buffer_stride,
        &d3d11_vertex_buffer_offset,
    );

    let mut d3d11_rasterizer_desc: D3D11_RASTERIZER_DESC = std::mem::zeroed();
    d3d11_rasterizer_desc.CullMode = D3D11_CULL_NONE;
    d3d11_rasterizer_desc.FillMode = D3D11_FILL_SOLID;
    d3d11_rasterizer_desc.FrontCounterClockwise = TRUE;
    d3d11_rasterizer_desc.DepthBias = FALSE;
    d3d11_rasterizer_desc.DepthBiasClamp = 0.0;
    d3d11_rasterizer_desc.SlopeScaledDepthBias = 0.0;
    d3d11_rasterizer_desc.DepthClipEnable = TRUE;
    d3d11_rasterizer_desc.ScissorEnable = FALSE;
    d3d11_rasterizer_desc.MultisampleEnable = FALSE;
    d3d11_rasterizer_desc.AntialiasedLineEnable = TRUE;

    let mut d3d11_rasterizer_state: *mut ID3D11RasterizerState = null_mut();
    let hr = d3d11_device
        .as_ref()
        .unwrap()
        .CreateRasterizerState(&d3d11_rasterizer_desc, &mut d3d11_rasterizer_state);
    if hr != 0 {
        panic!();
    }
    let d3d11_rasterizer_state = d3d11_rasterizer_state;
    let _release_d3d11_rasterizer_state_guard = scopeguard::guard((), |_| {
        d3d11_rasterizer_state.as_ref().unwrap().Release();
    });

    d3d11_device_context
        .as_ref()
        .unwrap()
        .RSSetState(d3d11_rasterizer_state);

    let d3d11_state_block = capture_d3d11_state(d3d11_device_context);

    dxgi_swap_chain.as_ref().unwrap().AddRef();
    d3d11_device.as_ref().unwrap().AddRef();
    d3d11_device_context.as_ref().unwrap().AddRef();
    d3d11_render_target_view.as_ref().unwrap().AddRef();
    d3d11_blend_state.as_ref().unwrap().AddRef();
    d3d11_vertex_shader.as_ref().unwrap().AddRef();
    d3d11_pixel_shader.as_ref().unwrap().AddRef();
    d3d11_input_layout.as_ref().unwrap().AddRef();
    d3d11_vertex_buffer.as_ref().unwrap().AddRef();
    d3d11_rasterizer_state.as_ref().unwrap().AddRef();

    *d3d11_overlay_state = Some(D3D11OverlayState {
        dxgi_swap_chain,
        dxgi_swap_chain_back_buffer,
        d3d11_device,
        d3d11_device_context,
        d3d11_render_target_view,
        d3d11_blend_state,
        d3d11_vertex_shader,
        d3d11_pixel_shader,
        d3d11_input_layout,
        d3d11_vertex_buffer,
        d3d11_rasterizer_state,
        d3d11_state_block,
    });
}

pub unsafe fn update_d3d11_overlay(dxgi_swap_chain: *mut IDXGISwapChain) {
    init_d3d11_overlay_if_not_initialized(dxgi_swap_chain);

    let d3d11_overlay_state_guard = D3D11_OVERLAY_STATE.lock().unwrap();

    let d3d11_overlay_state = &*d3d11_overlay_state_guard;

    let d3d11_overlay_state = d3d11_overlay_state.as_ref().unwrap();

    let d3d11_retaining_state_block = capture_d3d11_state(d3d11_overlay_state.d3d11_device_context);
    let _release_d3d11_retaining_state_block_guard = scopeguard::guard((), |_| {
        release_d3d11_state(&d3d11_retaining_state_block);
    });
    let _apply_d3d11_retaining_state_block_guard = scopeguard::guard((), |_| {
        apply_d3d11_state(
            d3d11_overlay_state.d3d11_device_context,
            &d3d11_retaining_state_block,
        );
    });

    apply_d3d11_state(
        d3d11_overlay_state.d3d11_device_context,
        &d3d11_overlay_state.d3d11_state_block,
    );

    d3d11_overlay_state
        .d3d11_device_context
        .as_ref()
        .unwrap()
        .GSSetShader(null_mut(), null_mut(), 0);

    d3d11_overlay_state
        .d3d11_device_context
        .as_ref()
        .unwrap()
        .VSSetShader(d3d11_overlay_state.d3d11_vertex_shader, null_mut(), 0);

    d3d11_overlay_state
        .d3d11_device_context
        .as_ref()
        .unwrap()
        .PSSetShader(d3d11_overlay_state.d3d11_pixel_shader, null_mut(), 0);

    d3d11_overlay_state
        .d3d11_device_context
        .as_ref()
        .unwrap()
        .Draw(3, 0);

    d3d11_overlay_state
        .d3d11_device_context
        .as_ref()
        .unwrap()
        .Flush();
}
