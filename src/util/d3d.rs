use bindings::Windows::{Graphics::DirectX::Direct3D11::IDirect3DDevice, Win32::{Graphics::{Direct3D11::{D3D11CreateDevice, D3D11_CREATE_DEVICE_BGRA_SUPPORT, D3D11_CREATE_DEVICE_FLAG, D3D11_SDK_VERSION, D3D_DRIVER_TYPE, D3D_DRIVER_TYPE_HARDWARE, D3D_DRIVER_TYPE_WARP, ID3D11Device}, Dxgi::{DXGI_ERROR_UNSUPPORTED, IDXGIDevice}}, System::WinRT::{CreateDirect3D11DeviceFromDXGIDevice, IDirect3DDxgiInterfaceAccess}}};
use windows::{Abi, IInspectable, Interface};

fn create_d3d_device_with_type(
    driver_type: D3D_DRIVER_TYPE,
    flags: D3D11_CREATE_DEVICE_FLAG,
    device: *mut Option<ID3D11Device>,
) -> windows::Result<()> {
    unsafe {
        D3D11CreateDevice(
            None,
            driver_type,
            None,
            flags,
            std::ptr::null(),
            0,
            D3D11_SDK_VERSION as u32,
            device,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
        )
    }
}

pub fn create_d3d_device() -> windows::Result<ID3D11Device> {
    let mut device = None;
    let mut hresult = create_d3d_device_with_type(
        D3D_DRIVER_TYPE_HARDWARE,
        D3D11_CREATE_DEVICE_BGRA_SUPPORT,
        &mut device,
    );
    if let Err(error) = &hresult {
        if error.code() == DXGI_ERROR_UNSUPPORTED {
            hresult = create_d3d_device_with_type(
                D3D_DRIVER_TYPE_WARP,
                D3D11_CREATE_DEVICE_BGRA_SUPPORT,
                &mut device,
            );
        }
    }
    hresult?;
    Ok(device.unwrap())
}

pub fn create_direct3d_device(d3d_device: &ID3D11Device) -> windows::Result<IDirect3DDevice> {
    let dxgi_device: IDXGIDevice = d3d_device.cast()?;
    let inspectable: IInspectable = unsafe {
        CreateDirect3D11DeviceFromDXGIDevice(dxgi_device)?
    };
    inspectable.cast()
}

pub fn get_d3d_interface_from_object<S: Interface, R: Interface + Abi>(
    object: &S,
) -> windows::Result<R> {
    let access: IDirect3DDxgiInterfaceAccess = object.cast()?;
    let object = unsafe { access.GetInterface::<R>()? };
    Ok(object)
}