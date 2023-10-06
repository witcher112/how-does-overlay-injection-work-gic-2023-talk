use std::ffi::CString;
use std::mem::zeroed;
use std::ptr::copy;
use std::ptr::null_mut;
use std::sync::Mutex;
use winapi::ctypes::c_void;
use winapi::shared::dxgi::DXGI_SWAP_CHAIN_DESC;
use winapi::shared::dxgi::IDXGISwapChain;
use winapi::shared::dxgiformat::DXGI_FORMAT_B8G8R8A8_UNORM;
use winapi::shared::dxgiformat::DXGI_FORMAT_B8G8R8A8_UNORM_SRGB;
use winapi::shared::dxgiformat::DXGI_FORMAT_B8G8R8X8_UNORM_SRGB;
use winapi::shared::dxgiformat::DXGI_FORMAT_R32G32B32_FLOAT;
use winapi::shared::dxgiformat::DXGI_FORMAT_R32G32_FLOAT;
use winapi::shared::dxgiformat::DXGI_FORMAT_R8G8B8A8_UNORM_SRGB;
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
use winapi::um::d3d11::ID3D11ShaderResourceView;
use winapi::um::d3d11::ID3D11Texture2D;
use winapi::um::d3d11::ID3D11VertexShader;
use winapi::um::d3d11::D3D11_BIND_SHADER_RESOURCE;
use winapi::um::d3d11::D3D11_BIND_VERTEX_BUFFER;
use winapi::um::d3d11::D3D11_BLEND_DESC;
use winapi::um::d3d11::D3D11_BLEND_DEST_ALPHA;
use winapi::um::d3d11::D3D11_BLEND_INV_SRC_ALPHA;
use winapi::um::d3d11::D3D11_BLEND_OP_ADD;
use winapi::um::d3d11::D3D11_BLEND_SRC_ALPHA;
use winapi::um::d3d11::D3D11_BOX;
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
use winapi::um::d3d11::D3D11_SHADER_RESOURCE_VIEW_DESC;
use winapi::um::d3d11::D3D11_TEXTURE2D_DESC;
use winapi::um::d3d11::D3D11_USAGE_DEFAULT;
use winapi::um::d3d11::D3D11_USAGE_DYNAMIC;
use winapi::um::d3d11::D3D11_VIEWPORT;
use winapi::um::d3dcommon::D3D11_PRIMITIVE_TOPOLOGY_TRIANGLELIST;
use winapi::um::d3dcommon::D3D11_SRV_DIMENSION_TEXTURE2D;
use winapi::Interface;

pub use crate::*;

pub struct D3D11OverlayState {
    pub dxgi_swap_chain: *mut IDXGISwapChain,
    pub dxgi_swap_chain_back_buffer: *mut ID3D11Texture2D,
    pub dxgi_swap_chain_back_buffer_desc: D3D11_TEXTURE2D_DESC,
    pub d3d11_device: *mut ID3D11Device,
    pub d3d11_device_context: *mut ID3D11DeviceContext,
    pub d3d11_render_target_view: *mut ID3D11RenderTargetView,
    pub d3d11_blend_state: *mut ID3D11BlendState,
    pub d3d11_vertex_shader: *mut ID3D11VertexShader,
    pub d3d11_pixel_shader: *mut ID3D11PixelShader,
    pub d3d11_input_layout: *mut ID3D11InputLayout,
    pub d3d11_vertex_buffer: *mut ID3D11Buffer,
    pub d3d11_main_texture: *mut ID3D11Texture2D,
    pub d3d11_shader_main_texture_view: *mut ID3D11ShaderResourceView,
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
    pub texcoord_u: f32,
    pub texcoord_v: f32,
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
    let d3d11_input_element_1_semantic_name = CString::new("TEXCOORD").unwrap();
    let d3d11_input_elements_desc: &[D3D11_INPUT_ELEMENT_DESC] = &[
        D3D11_INPUT_ELEMENT_DESC {
            SemanticName: d3d11_input_element_0_semantic_name.as_ptr(),
            SemanticIndex: 0,
            Format: DXGI_FORMAT_R32G32B32_FLOAT,
            InputSlot: 0,
            AlignedByteOffset: 0,
            InputSlotClass: D3D11_INPUT_PER_VERTEX_DATA,
            InstanceDataStepRate: 0,
        },
        D3D11_INPUT_ELEMENT_DESC {
            SemanticName: d3d11_input_element_1_semantic_name.as_ptr(),
            SemanticIndex: 0,
            Format: DXGI_FORMAT_R32G32_FLOAT,
            InputSlot: 0,
            AlignedByteOffset: 12,
            InputSlotClass: D3D11_INPUT_PER_VERTEX_DATA,
            InstanceDataStepRate: 0,
        },
    ];

    let mut d3d11_input_layout: *mut ID3D11InputLayout = null_mut();
    let hr = d3d11_device.as_ref().unwrap().CreateInputLayout(
        d3d11_input_elements_desc.as_ptr(),
        2,
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
            position_x: -1.0,
            position_y: 1.0,
            position_z: 0.0,
            texcoord_u: 0.0,
            texcoord_v: 0.0,
        },
        D3D11OverlayVertexData {
            position_x: -1.0,
            position_y: -1.0,
            position_z: 0.0,
            texcoord_u: 0.0,
            texcoord_v: 1.0,
        },
        D3D11OverlayVertexData {
            position_x: 1.0,
            position_y: -1.0,
            position_z: 0.0,
            texcoord_u: 1.0,
            texcoord_v: 1.0,
        },
        D3D11OverlayVertexData {
            position_x: 1.0,
            position_y: -1.0,
            position_z: 0.0,
            texcoord_u: 1.0,
            texcoord_v: 1.0,
        },
        D3D11OverlayVertexData {
            position_x: 1.0,
            position_y: 1.0,
            position_z: 0.0,
            texcoord_u: 1.0,
            texcoord_v: 0.0,
        },
        D3D11OverlayVertexData {
            position_x: -1.0,
            position_y: 1.0,
            position_z: 0.0,
            texcoord_u: 0.0,
            texcoord_v: 0.0,
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

    let mut d3d11_main_texture_desc: D3D11_TEXTURE2D_DESC = std::mem::zeroed();
    d3d11_main_texture_desc.Width = dxgi_swap_chain_back_buffer_desc.Width;
    d3d11_main_texture_desc.Height = dxgi_swap_chain_back_buffer_desc.Height;
    d3d11_main_texture_desc.MipLevels = 1;
    d3d11_main_texture_desc.ArraySize = 1;
    d3d11_main_texture_desc.Format = match dxgi_swap_chain_back_buffer_desc.Format {
        DXGI_FORMAT_R8G8B8A8_UNORM_SRGB => DXGI_FORMAT_B8G8R8A8_UNORM_SRGB,
        DXGI_FORMAT_B8G8R8A8_UNORM_SRGB => DXGI_FORMAT_B8G8R8A8_UNORM_SRGB,
        DXGI_FORMAT_B8G8R8X8_UNORM_SRGB => DXGI_FORMAT_B8G8R8A8_UNORM_SRGB,
        _ => DXGI_FORMAT_B8G8R8A8_UNORM,
    };
    d3d11_main_texture_desc.SampleDesc.Count = 1;
    d3d11_main_texture_desc.Usage = D3D11_USAGE_DEFAULT;
    d3d11_main_texture_desc.BindFlags = D3D11_BIND_SHADER_RESOURCE;
    d3d11_main_texture_desc.CPUAccessFlags = D3D11_CPU_ACCESS_WRITE;

    let mut d3d11_main_texture: *mut ID3D11Texture2D = null_mut();
    let hr = d3d11_device.as_ref().unwrap().CreateTexture2D(
        &d3d11_main_texture_desc,
        null_mut(),
        &mut d3d11_main_texture,
    );
    if hr != 0 {
        panic!();
    }
    let d3d11_main_texture = d3d11_main_texture;
    let _release_d3d11_main_texture_guard = scopeguard::guard((), |_| {
        d3d11_main_texture.as_ref().unwrap().Release();
    });

    let mut d3d11_shader_main_texture_view_desc: D3D11_SHADER_RESOURCE_VIEW_DESC =
        std::mem::zeroed();
    d3d11_shader_main_texture_view_desc.Format = d3d11_main_texture_desc.Format;
    d3d11_shader_main_texture_view_desc.ViewDimension = D3D11_SRV_DIMENSION_TEXTURE2D;
    d3d11_shader_main_texture_view_desc
        .u
        .Texture2D_mut()
        .MostDetailedMip = 0;
    d3d11_shader_main_texture_view_desc
        .u
        .Texture2D_mut()
        .MipLevels = d3d11_main_texture_desc.MipLevels;

    let mut d3d11_shader_main_texture_view: *mut ID3D11ShaderResourceView = null_mut();
    let hr = d3d11_device.as_ref().unwrap().CreateShaderResourceView(
        d3d11_main_texture as *mut _,
        &d3d11_shader_main_texture_view_desc,
        &mut d3d11_shader_main_texture_view,
    );
    if hr != 0 {
        panic!();
    }
    let _release_d3d11_shader_main_texture_view_guard = scopeguard::guard((), |_| {
        d3d11_shader_main_texture_view.as_ref().unwrap().Release();
    });

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
    d3d11_main_texture.as_ref().unwrap().AddRef();
    d3d11_shader_main_texture_view.as_ref().unwrap().AddRef();
    d3d11_rasterizer_state.as_ref().unwrap().AddRef();

    *d3d11_overlay_state = Some(D3D11OverlayState {
        dxgi_swap_chain,
        dxgi_swap_chain_back_buffer,
        dxgi_swap_chain_back_buffer_desc,
        d3d11_device,
        d3d11_device_context,
        d3d11_render_target_view,
        d3d11_blend_state,
        d3d11_vertex_shader,
        d3d11_pixel_shader,
        d3d11_input_layout,
        d3d11_vertex_buffer,
        d3d11_main_texture,
        d3d11_shader_main_texture_view,
        d3d11_rasterizer_state,
        d3d11_state_block,
    });

    let mut server_message_payload_proto = proto::ServerMessagePayload::new();

    let mut on_size_changed_message_payload_proto = proto::OnSizeChangedMessagePayload::new();

    on_size_changed_message_payload_proto.set_width(dxgi_swap_chain_back_buffer_desc.Width);
    on_size_changed_message_payload_proto.set_height(dxgi_swap_chain_back_buffer_desc.Height);

    server_message_payload_proto
        .set_on_size_changed_message_payload(on_size_changed_message_payload_proto);

    send_server_message(server_message_payload_proto);

    let mut dxgi_swap_chain_desc: DXGI_SWAP_CHAIN_DESC = zeroed();
    let hr = dxgi_swap_chain
        .as_ref()
        .unwrap()
        .GetDesc(&mut dxgi_swap_chain_desc);
    if hr != 0 {
        panic!();
    }

    init_input_hook(dxgi_swap_chain_desc.OutputWindow);
}

pub unsafe fn update_d3d11_overlay(dxgi_swap_chain: *mut IDXGISwapChain) {
    init_d3d11_overlay_if_not_initialized(dxgi_swap_chain);

    let d3d11_overlay_state_guard = D3D11_OVERLAY_STATE.lock().unwrap();

    let d3d11_overlay_state = &*d3d11_overlay_state_guard;

    let d3d11_overlay_state = d3d11_overlay_state.as_ref().unwrap();

    {
        let overlay_state_guard = OVERLAY_STATE.lock().unwrap();

        let overlay_state = &*overlay_state_guard;

        if let Some(overlay_texture_info) = overlay_state.texture_info.as_ref() {
            if overlay_texture_info.width
                == d3d11_overlay_state.dxgi_swap_chain_back_buffer_desc.Width
                && overlay_texture_info.height
                    == d3d11_overlay_state.dxgi_swap_chain_back_buffer_desc.Height
            {
                let mut d3d11_box: D3D11_BOX = std::mem::zeroed();

                d3d11_box.left = 0;
                d3d11_box.right = overlay_texture_info.width;
                d3d11_box.top = 0;
                d3d11_box.bottom = overlay_texture_info.height;
                d3d11_box.front = 0;
                d3d11_box.back = 1;

                d3d11_overlay_state
                    .d3d11_device_context
                    .as_ref()
                    .unwrap()
                    .UpdateSubresource(
                        d3d11_overlay_state.d3d11_main_texture as _,
                        0,
                        &d3d11_box,
                        overlay_texture_info.bytes.as_ptr() as *const _,
                        d3d11_overlay_state.dxgi_swap_chain_back_buffer_desc.Width * 4,
                        0,
                    );
            }
        }
    }

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
        .PSSetShaderResources(0, 1, &d3d11_overlay_state.d3d11_shader_main_texture_view);

    d3d11_overlay_state
        .d3d11_device_context
        .as_ref()
        .unwrap()
        .Draw(6, 0);

    d3d11_overlay_state
        .d3d11_device_context
        .as_ref()
        .unwrap()
        .Flush();
}
