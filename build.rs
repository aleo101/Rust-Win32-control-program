#![allow(non_snake_case, unused_crate_dependencies, unused_imports)]
fn main() {
    windows::build! {
        Windows::Win32::System::Com::{CoCreateInstance, CoInitializeEx,},
        Windows::Win32::{
            Foundation::{DXGI_STATUS_OCCLUDED, HINSTANCE, HWND, LPARAM, LRESULT, PSTR, WPARAM},
            Graphics::{
                Direct2D::{
                    CLSID_D2D1Shadow, D2D1CreateFactory, ID2D1Bitmap1, ID2D1Device,
                    ID2D1DeviceContext, ID2D1Effect, ID2D1Factory1, ID2D1SolidColorBrush,
                },
                Direct3D11::{D3D11CreateDevice, ID3D11Device, D3D11_SDK_VERSION},
                Dxgi::{
                    CreateDXGIFactory1, IDXGIAdapter, IDXGIDevice, IDXGIFactory2, IDXGISwapChain1,
                    DXGI_ERROR_UNSUPPORTED, DXGI_FORMAT, DXGI_PRESENT_TEST,
                    DXGI_USAGE_RENDER_TARGET_OUTPUT,
                },
                Gdi::{BeginPaint, EndPaint, PAINTSTRUCT},
            },
            System::{
                LibraryLoader::GetModuleHandleW,
                Performance::{QueryPerformanceCounter, QueryPerformanceFrequency},
                SystemInformation::GetLocalTime,
            },
            UI::{
                WindowsAndMessaging::{
                    CreateWindowExW, DefWindowProcA, DispatchMessageA, GetMessageW, GetWindowLongA,
                    GetWindowLongPtrA, LoadCursorW, PeekMessageA, PostQuitMessage, RegisterClassExW,
                    SetWindowLongA, SetWindowTextW, SetWindowLongPtrA, GetWindowTextA, SendMessageW, ShowWindow, KillTimer, SetTimer, CREATESTRUCTA, CW_USEDEFAULT, IDC_HAND, MSG,
                    SIZE_MINIMIZED, WINDOW_LONG_PTR_INDEX, WM_DESTROY, WM_USER, *,
                    WM_CREATE, WM_QUIT, WNDCLASSA, WNDCLASSEXW, WM_COMMAND, WM_HSCROLL, WM_TIMER, WM_PARENTNOTIFY,
                },
                KeyboardAndMouseInput::{IsWindowEnabled, EnableWindow},
                Controls::{InitCommonControls, TBS_AUTOTICKS, TBS_ENABLESELRANGE, TBM_SETRANGE, TBM_SETPAGESIZE,
                    TBM_SETTICFREQ,TBM_SETSEL,TBM_SETPOS,},
            },
        },
    };
}
