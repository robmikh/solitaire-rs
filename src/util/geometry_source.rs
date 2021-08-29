use bindings::*;
use bindings::{Windows::Win32::{Foundation::E_NOTIMPL, Graphics::Direct2D::{ID2D1Factory, ID2D1Geometry}}};
use windows::implement;

#[implement(Windows::Graphics::IGeometrySource2D, Windows::Win32::System::WinRT::IGeometrySource2DInterop)]
pub struct GeometrySource {
    geometry: ID2D1Geometry,
}

#[allow(non_snake_case)]
impl GeometrySource {
    pub fn new(geometry: ID2D1Geometry) -> Self {
        Self { geometry }
    }

    pub fn GetGeometry(&self) -> windows::Result<ID2D1Geometry> {
        Ok(self.geometry.clone())
    }

    pub fn TryGetGeometryUsingFactory(&self, _: &ID2D1Factory) -> windows::Result<ID2D1Geometry> {
        E_NOTIMPL.ok()?;
        unreachable!()
    }
}