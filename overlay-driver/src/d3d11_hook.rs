use std::ffi::c_void;
use std::ffi::CString;
use std::ptr::null_mut;
use std::sync::Mutex;
use widestring::WideCString;
use winapi::shared::dxgi::IDXGIAdapter;
use winapi::shared::dxgi::IDXGIFactory1;
use winapi::shared::dxgi::IDXGISwapChain;
use winapi::shared::dxgi::DXGI_SWAP_CHAIN_DESC;
use winapi::shared::dxgi::DXGI_SWAP_EFFECT_DISCARD;
use winapi::shared::dxgiformat::DXGI_FORMAT_R8G8B8A8_UNORM_SRGB;
use winapi::shared::dxgitype::DXGI_MODE_DESC;
use winapi::shared::dxgitype::DXGI_MODE_SCALING_CENTERED;
use winapi::shared::dxgitype::DXGI_MODE_SCANLINE_ORDER_UNSPECIFIED;
use winapi::shared::dxgitype::DXGI_RATIONAL;
use winapi::shared::dxgitype::DXGI_SAMPLE_DESC;
use winapi::shared::dxgitype::DXGI_USAGE_RENDER_TARGET_OUTPUT;
use winapi::shared::guiddef::REFIID;
use winapi::shared::minwindef::FALSE;
use winapi::shared::minwindef::HMODULE;
use winapi::shared::minwindef::TRUE;
use winapi::shared::minwindef::UINT;
use winapi::shared::ntdef::HRESULT;
use winapi::shared::windef::RECT;
use winapi::um::d3d11::ID3D11Device;
use winapi::um::d3d11::ID3D11DeviceContext;
use winapi::um::d3d11::D3D11_SDK_VERSION;
use winapi::um::d3dcommon::D3D_DRIVER_TYPE;
use winapi::um::d3dcommon::D3D_DRIVER_TYPE_UNKNOWN;
use winapi::um::d3dcommon::D3D_FEATURE_LEVEL;
use winapi::um::libloaderapi::FreeLibrary;
use winapi::um::libloaderapi::GetProcAddress;
use winapi::um::libloaderapi::LoadLibraryW;
use winapi::um::winuser::CreateWindowExW;
use winapi::um::winuser::DestroyWindow;
use winapi::um::winuser::GetClientRect;
use winapi::um::winuser::CW_USEDEFAULT;
use winapi::um::winuser::WS_OVERLAPPEDWINDOW;
use winapi::Interface;

use crate::update_d3d11_overlay;

type CreateDXGIFactory1 = unsafe extern "system" fn(REFIID, *mut *mut c_void) -> HRESULT;

type D3D11CreateDeviceAndSwapChain = unsafe extern "system" fn(
    *mut IDXGIAdapter,
    D3D_DRIVER_TYPE,
    HMODULE,
    UINT,
    *const D3D_FEATURE_LEVEL,
    UINT,
    UINT,
    *const DXGI_SWAP_CHAIN_DESC,
    *mut *mut IDXGISwapChain,
    *mut *mut ID3D11Device,
    *mut D3D_FEATURE_LEVEL,
    *mut *mut ID3D11DeviceContext,
) -> HRESULT;

pub type DXGISwapChainPresentMethod =
    unsafe extern "system" fn(*mut IDXGISwapChain, UINT, UINT) -> HRESULT;

lazy_static::lazy_static! {
    pub static ref DXGI_SWAP_CHAIN_PRESENT_METHOD_HOOK: Mutex<Option<detour::RawDetour>> = Mutex::new(None);
}

pub unsafe fn init_d3d11_hook() {
    let dxgi_library_name = WideCString::from_str("DXGI.DLL").unwrap();
    let dxgi_library_handle = LoadLibraryW(dxgi_library_name.as_ptr());
    if dxgi_library_handle.is_null() {
        panic!();
    }
    let _free_dxgi_library_guard = scopeguard::guard((), |_| {
        FreeLibrary(dxgi_library_handle);
    });

    let d3d11_library_name = WideCString::from_str("D3D11.DLL").unwrap();
    let d3d11_library_handle = LoadLibraryW(d3d11_library_name.as_ptr());
    if d3d11_library_handle.is_null() {
        panic!();
    }
    let _free_d3d11_library_guard = scopeguard::guard((), |_| {
        FreeLibrary(d3d11_library_handle);
    });

    let create_dxgi_factory_1_name = CString::new("CreateDXGIFactory1").unwrap();
    let create_dxgi_factory_1: CreateDXGIFactory1 = std::mem::transmute(GetProcAddress(
        dxgi_library_handle,
        create_dxgi_factory_1_name.as_ptr(),
    ));

    let d3d11_create_device_and_swap_chain_name =
        CString::new("D3D11CreateDeviceAndSwapChain").unwrap();
    let d3d11_create_device_and_swap_chain: D3D11CreateDeviceAndSwapChain =
        std::mem::transmute(GetProcAddress(
            d3d11_library_handle,
            d3d11_create_device_and_swap_chain_name.as_ptr(),
        ));

    let mut dxgi_factory_1: *mut c_void = null_mut();
    let hr = create_dxgi_factory_1(&IDXGIFactory1::uuidof(), &mut dxgi_factory_1);
    if hr != 0 {
        panic!();
    }
    let dxgi_factory_1 = dxgi_factory_1 as *mut IDXGIFactory1;
    let _release_dxgi_factory_1_guard = scopeguard::guard((), |_| {
        dxgi_factory_1.as_ref().unwrap().Release();
    });

    let mut dxgi_adapter: *mut IDXGIAdapter = null_mut();
    let hr = dxgi_factory_1
        .as_ref()
        .unwrap()
        .EnumAdapters(0, &mut dxgi_adapter);
    if hr != 0 {
        panic!();
    }
    let dxgi_adapter = dxgi_adapter;
    let _release_dxgi_adapter_guard = scopeguard::guard((), |_| {
        dxgi_adapter.as_ref().unwrap().Release();
    });

    let d3d11_device_window_class_name = WideCString::from_str("STATIC").unwrap();
    let d3d11_device_window_name = WideCString::from_str("ID3D11DeviceWnd").unwrap();
    let d3d11_device_window_handle = CreateWindowExW(
        0,
        d3d11_device_window_class_name.as_ptr(),
        d3d11_device_window_name.as_ptr(),
        WS_OVERLAPPEDWINDOW,
        CW_USEDEFAULT,
        CW_USEDEFAULT,
        640,
        480,
        null_mut(),
        null_mut(),
        null_mut(),
        null_mut(),
    );
    let _destroy_d3d11_device_window_guard = scopeguard::guard((), |_| {
        DestroyWindow(d3d11_device_window_handle);
    });

    let mut d3d11_device_window_handle_client_rect: RECT = std::mem::zeroed();
    let result = GetClientRect(
        d3d11_device_window_handle,
        &mut d3d11_device_window_handle_client_rect,
    );
    if result == FALSE {
        panic!();
    }

    let dxgi_mode_desc = DXGI_MODE_DESC {
        Width: (d3d11_device_window_handle_client_rect.right
            - d3d11_device_window_handle_client_rect.left) as u32,
        Height: (d3d11_device_window_handle_client_rect.bottom
            - d3d11_device_window_handle_client_rect.top) as u32,
        RefreshRate: DXGI_RATIONAL {
            Numerator: 60,
            Denominator: 1,
        },
        Format: DXGI_FORMAT_R8G8B8A8_UNORM_SRGB,
        ScanlineOrdering: DXGI_MODE_SCANLINE_ORDER_UNSPECIFIED,
        Scaling: DXGI_MODE_SCALING_CENTERED,
    };

    let dxgi_sample_desc = DXGI_SAMPLE_DESC {
        Count: 1,
        Quality: 0,
    };

    let dxgi_swap_chain_desc = DXGI_SWAP_CHAIN_DESC {
        BufferDesc: dxgi_mode_desc,
        SampleDesc: dxgi_sample_desc,
        BufferUsage: DXGI_USAGE_RENDER_TARGET_OUTPUT,
        BufferCount: 2,
        OutputWindow: d3d11_device_window_handle,
        Windowed: TRUE,
        SwapEffect: DXGI_SWAP_EFFECT_DISCARD,
        Flags: 0,
    };

    let mut dxgi_swap_chain: *mut IDXGISwapChain = null_mut();
    let mut d3d11_device: *mut ID3D11Device = null_mut();
    let mut d3d_feature_level: D3D_FEATURE_LEVEL = 0;
    let mut d3d11_device_context: *mut ID3D11DeviceContext = null_mut();
    let hr = d3d11_create_device_and_swap_chain(
        dxgi_adapter,
        D3D_DRIVER_TYPE_UNKNOWN,
        null_mut(),
        0,
        null_mut(),
        0,
        D3D11_SDK_VERSION,
        &dxgi_swap_chain_desc,
        &mut dxgi_swap_chain,
        &mut d3d11_device,
        &mut d3d_feature_level,
        &mut d3d11_device_context,
    );
    if hr != 0 {
        panic!();
    }
    let dxgi_swap_chain = dxgi_swap_chain;
    let d3d11_device = d3d11_device;
    let d3d11_device_context = d3d11_device_context;
    let _release_dxgi_swap_chain_guard = scopeguard::guard((), |_| {
        dxgi_swap_chain.as_ref().unwrap().Release();
    });
    let _release_d3d11_device_guard = scopeguard::guard((), |_| {
        d3d11_device.as_ref().unwrap().Release();
    });
    let _release_d3d11_device_context_guard = scopeguard::guard((), |_| {
        d3d11_device_context.as_ref().unwrap().Release();
    });

    let dxgi_swap_chain_present_method: DXGISwapChainPresentMethod = dxgi_swap_chain
        .as_ref()
        .unwrap()
        .lpVtbl
        .as_ref()
        .unwrap()
        .Present;

    let dxgi_swap_chain_present_method_hook = detour::RawDetour::new(
        dxgi_swap_chain_present_method as *const (),
        dxgi_swap_chain_present_method_hook_detour as *const (),
    )
    .unwrap();

    let mut dxgi_swap_chain_present_method_hook_guard =
        DXGI_SWAP_CHAIN_PRESENT_METHOD_HOOK.lock().unwrap();

    *dxgi_swap_chain_present_method_hook_guard = Some(dxgi_swap_chain_present_method_hook);

    let dxgi_swap_chain_present_method_hook = &mut *dxgi_swap_chain_present_method_hook_guard;

    let dxgi_swap_chain_present_method_hook = dxgi_swap_chain_present_method_hook.as_mut().unwrap();

    dxgi_swap_chain_present_method_hook.enable().unwrap();
}

pub unsafe extern "system" fn dxgi_swap_chain_present_method_hook_detour(
    dxgi_swap_chain: *mut IDXGISwapChain,
    sync_interval: UINT,
    flags: UINT,
) -> HRESULT {
    let dxgi_swap_chain_present_method_hook_guard =
        DXGI_SWAP_CHAIN_PRESENT_METHOD_HOOK.lock().unwrap();

    let dxgi_swap_chain_present_method_hook = &*dxgi_swap_chain_present_method_hook_guard;

    let dxgi_swap_chain_present_method_hook = dxgi_swap_chain_present_method_hook.as_ref().unwrap();

    let dxgi_swap_chain_present_method_hook_trampoline: DXGISwapChainPresentMethod =
        std::mem::transmute(dxgi_swap_chain_present_method_hook.trampoline());

    update_d3d11_overlay(dxgi_swap_chain);

    return dxgi_swap_chain_present_method_hook_trampoline(
        dxgi_swap_chain,
        sync_interval,
        flags,
    );
}
