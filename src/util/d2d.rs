use bindings::Windows::Win32::Graphics::{Direct2D::{D2D1CreateFactory, D2D1_FACTORY_OPTIONS, D2D1_FACTORY_TYPE_SINGLE_THREADED, ID2D1Device, ID2D1Factory1}, Direct3D11::ID3D11Device, Dxgi::IDXGIDevice};
use windows::{Abi, Interface};

pub fn create_d2d_factory() -> windows::Result<ID2D1Factory1> {
    let options = D2D1_FACTORY_OPTIONS::default();
    let mut result = None;
    unsafe {
        D2D1CreateFactory(
            D2D1_FACTORY_TYPE_SINGLE_THREADED, 
            &ID2D1Factory1::IID,
            &options,
        result.set_abi()
        )?;
    }
    Ok(result.unwrap())
}

pub fn create_d2d_device(factory: &ID2D1Factory1, device: &ID3D11Device) -> windows::Result<ID2D1Device> {
    let dxgi_device: IDXGIDevice = device.cast()?;
    unsafe { factory.CreateDevice(&dxgi_device) }
}

