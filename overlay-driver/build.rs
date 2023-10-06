const D3D11_VERTEX_SHADER_SRC_FILE_PATH: &str =
    "src/d3d11_vertex_shader.hlsl";
const D3D11_PIXEL_SHADER_SRC_FILE_PATH: &str =
    "src/d3d11_pixel_shader.hlsl";

const FXC_EXE_FILE_PATH: &str = "bin/x86/fxc.exe";

enum DirectX11WindowsHookShaderType {
    VertexShader,
    PixelShader,
}

fn compile_directx_11_windows_hook_shader(
    directx_11_windows_hook_shader_type: DirectX11WindowsHookShaderType,
) {
    let out_dir_path = std::env::var("OUT_DIR").unwrap();
    let win_sdk_info = find_winsdk::SdkInfo::find(find_winsdk::SdkVersion::V8_1)
        .unwrap()
        .unwrap();
    let fxc_exe_file_path = win_sdk_info.installation_folder().join(FXC_EXE_FILE_PATH);

    let directx_11_windows_hook_shader_src_file_path =
        std::path::Path::new(match directx_11_windows_hook_shader_type {
            DirectX11WindowsHookShaderType::VertexShader => {
                D3D11_VERTEX_SHADER_SRC_FILE_PATH
            }
            DirectX11WindowsHookShaderType::PixelShader => {
                D3D11_PIXEL_SHADER_SRC_FILE_PATH
            }
        });
    let directx_11_windows_hook_shader_obj_file_path =
        std::path::Path::new(&out_dir_path).join(match directx_11_windows_hook_shader_type {
            DirectX11WindowsHookShaderType::VertexShader => {
                "d3d11_vertex_shader.obj"
            }
            DirectX11WindowsHookShaderType::PixelShader => {
                "d3d11_pixel_shader.obj"
            }
        });

    let fxc_exe_output = std::process::Command::new(fxc_exe_file_path)
        .arg(match directx_11_windows_hook_shader_type {
            DirectX11WindowsHookShaderType::VertexShader => "/Tvs_5_0",
            DirectX11WindowsHookShaderType::PixelShader => "/Tps_5_0",
        })
        .arg("/O3")
        .arg("/E")
        .arg(match directx_11_windows_hook_shader_type {
            DirectX11WindowsHookShaderType::VertexShader => "VShader",
            DirectX11WindowsHookShaderType::PixelShader => "PShader",
        })
        .arg("/Fo")
        .arg(directx_11_windows_hook_shader_obj_file_path)
        .arg(directx_11_windows_hook_shader_src_file_path)
        .output()
        .unwrap();

    if !fxc_exe_output.status.success() {
        panic!("fxc.exe: {:?}", fxc_exe_output);
    }
    println!("fxc.exe: {:?}", fxc_exe_output);
}

fn compile_directx_11_windows_hook_shaders() {
    compile_directx_11_windows_hook_shader(DirectX11WindowsHookShaderType::VertexShader);
    compile_directx_11_windows_hook_shader(DirectX11WindowsHookShaderType::PixelShader);
}

pub fn main() {
    compile_directx_11_windows_hook_shaders();
}
