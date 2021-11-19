use windows as Windows;
use windows::core::{implement, Result};
use windows::Win32::{
    Foundation::E_NOTIMPL,
    Graphics::Direct2D::{ID2D1Factory, ID2D1Geometry},
};

#[implement(
    Windows::Graphics::IGeometrySource2D,
    Windows::Win32::System::WinRT::Graphics::Direct2D::IGeometrySource2DInterop
)]
pub struct GeometrySource {
    geometry: ID2D1Geometry,
}

#[allow(non_snake_case)]
impl GeometrySource {
    pub fn new(geometry: ID2D1Geometry) -> Self {
        Self { geometry }
    }

    pub fn GetGeometry(&self) -> Result<ID2D1Geometry> {
        Ok(self.geometry.clone())
    }

    pub fn TryGetGeometryUsingFactory(&self, _: &Option<ID2D1Factory>) -> Result<ID2D1Geometry> {
        E_NOTIMPL.ok()?;
        unreachable!()
    }
}
