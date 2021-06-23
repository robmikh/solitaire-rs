fn main() {
    windows::build!(
        Windows::Foundation::Numerics::{Vector2, Vector3},
        Windows::Foundation::TimeSpan,
        Windows::Graphics::SizeInt32,
        Windows::System::DispatcherQueueController,
        Windows::UI::Composition::Desktop::DesktopWindowTarget,
        Windows::UI::Composition::{
            AnimationIterationBehavior, CompositionAnimation, CompositionBatchTypes,
            CompositionBorderMode, CompositionColorBrush, CompositionContainerShape,
            CompositionEllipseGeometry, CompositionGeometry, CompositionNineGridBrush,
            CompositionScopedBatch, CompositionShape, CompositionShapeCollection,
            CompositionSpriteShape, Compositor, ContainerVisual, ShapeVisual, SpriteVisual,
            Vector3KeyFrameAnimation, VisualCollection, CompositionRadialGradientBrush,
            CompositionColorGradientStopCollection, CompositionViewBox
        },
        Windows::UI::{Color, Colors},
        Windows::Win32::Foundation::{BOOL, HWND},
        Windows::Win32::System::WinRT::{
            ICompositorDesktopInterop, CreateDispatcherQueueController,
            DISPATCHERQUEUE_THREAD_TYPE, DISPATCHERQUEUE_THREAD_APARTMENTTYPE,
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
            ID2D1Factory1,
        }
    );
}