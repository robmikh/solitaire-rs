fn main() {
    windows::build!(
        Windows::Foundation::Numerics::{Vector2, Vector3},
        Windows::Foundation::TimeSpan,
        Windows::Graphics::{SizeInt32, IGeometrySource2D},
        Windows::Graphics::DirectX::Direct3D11::IDirect3DDevice,
        Windows::System::DispatcherQueueController,
        Windows::Storage::Streams::IRandomAccessStream,
        Windows::UI::Composition::Desktop::DesktopWindowTarget,
        Windows::UI::Composition::{
            AnimationIterationBehavior, CompositionAnimation, CompositionBatchTypes,
            CompositionBorderMode, CompositionColorBrush, CompositionContainerShape,
            CompositionEllipseGeometry, CompositionGeometry, CompositionNineGridBrush,
            CompositionScopedBatch, CompositionShape, CompositionShapeCollection,
            CompositionSpriteShape, Compositor, ContainerVisual, ShapeVisual, SpriteVisual,
            Vector3KeyFrameAnimation, VisualCollection, CompositionRadialGradientBrush,
            CompositionColorGradientStopCollection, CompositionViewBox, CompositionLinearGradientBrush,
            CompositionRectangleGeometry, CompositionPath, CompositionRoundedRectangleGeometry,
        },
        Windows::UI::{Color, Colors},
        Windows::Win32::Foundation::{BOOL, HWND, E_NOTIMPL, E_NOINTERFACE, RO_E_CLOSED},
        Windows::Win32::Storage::StructuredStorage::{IStream, STATSTG},
        Windows::Win32::System::WinRT::{
            ICompositorDesktopInterop, CreateDispatcherQueueController,
            DISPATCHERQUEUE_THREAD_TYPE, DISPATCHERQUEUE_THREAD_APARTMENTTYPE,
            RoInitialize, IDirect3DDxgiInterfaceAccess, CreateDirect3D11DeviceFromDXGIDevice,
            IGeometrySource2DInterop,
        },
        Windows::Win32::Graphics::Direct3D11::{
            D3D11CreateDevice,
            D3D_DRIVER_TYPE,
            D3D11_CREATE_DEVICE_FLAG,
            D3D11_SDK_VERSION,
            ID3D11Device,
        },
        Windows::Win32::Graphics::Dxgi::{
            IDXGIDevice,
            DXGI_ERROR_UNSUPPORTED,
        },
        Windows::Win32::Graphics::Direct2D::{
            ID2D1Factory1, D2D1_FACTORY_OPTIONS, D2D1CreateFactory, ID2D1Device, ID2D1DeviceContext,
            ID2D1DeviceContext5, D2D_SIZE_F, ID2D1SvgDocument, ID2D1SvgElement, D2D1_SVG_VIEWBOX,
            D2D1_SVG_PAINT_TYPE, ID2D1SvgPaint, D2D_MATRIX_3X2_F, ID2D1SvgPathData, ID2D1PathGeometry1,
        }
    );
}