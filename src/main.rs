#![allow(incomplete_features)]
#![allow(non_snake_case, unused_crate_dependencies, unused_imports)]
mod bindings {
    windows::include_bindings!();
}

use std::{collections::HashMap, mem::size_of};

use bindings::Windows::Win32::{
    Foundation::*, System::LibraryLoader::*, UI::Controls::*, UI::KeyboardAndMouseInput::*,
    UI::WindowsAndMessaging::*,
};
use std::str;
use winsafe::WString;

const ID_TXT_STATUS: u32 = 1;
const ID_TXT_OPEN: u32 = 3;
const ID_BTN_RST: u32 = 4;
const ID_BTN_CNFM: u32 = 5;
const ID_TXT_TIME: u32 = 6;
const ID_BTN_ST: u32 = 7;
const ID_BTN_INCR: u32 = 8;
const ID_BTN_DECR: u32 = 9;
const IDT_TIMER1: u32 = 10;
const ID_TB_SCROLL: u32 = 11;
static mut HANDLE_OPEN: HWND = HWND(0);
static mut HANDLE_RST: HWND = HWND(0);
static mut HANDLE_CNFM: HWND = HWND(0);
static mut HANDLE_TIME: HWND = HWND(0);
static mut HANDLE_ST: HWND = HWND(0);
static mut HANDLE_INCR: HWND = HWND(0);
static mut HANDLE_DECR: HWND = HWND(0);
static mut HANDLE_STATUS: HWND = HWND(0);
static mut HANDLE_TB: HWND = HWND(0);

fn main() {
    unsafe {
        let instance = GetModuleHandleW(None);
        debug_assert!(instance.0 != 0);

        let wc = WNDCLASSEXW {
            cbSize: size_of::<WNDCLASSEXW>() as u32,
            style: CS_HREDRAW | CS_VREDRAW,
            hCursor: LoadCursorW(None, IDC_HAND),
            hInstance: instance,
            lpszClassName: PWSTR(WString::from_str("window").as_mut_ptr()),
            lpfnWndProc: Some(wndproc),
            ..Default::default()
        };

        let atom = RegisterClassExW(&wc);
        debug_assert!(atom != 0);

        let handle = CreateWindowExW(
            WINDOW_EX_STYLE(0),
            "window",
            "Class test",
            WS_OVERLAPPEDWINDOW,
            100,
            100,
            1000,
            500,
            HWND::NULL,
            HMENU::NULL,
            instance,
            std::ptr::null_mut(),
        );

        debug_assert!(handle.0 != 0);
        let mut message = MSG::default();
        ShowWindow(handle, SW_SHOW);
        while GetMessageW(&mut message, HWND(0), 0, 0).into() {
            TranslateMessage(&mut message);
            DispatchMessageW(&mut message);
        }
    }
}

extern "system" fn wndproc(window: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    unsafe {
        let mut text = WString::new_alloc_buffer(60);
        let privilege_name_ptr = PWSTR(text.as_mut_ptr());
        static mut PERCENTAGE_TO_OPEN: i32 = 0;
        static mut TIME: i32 = 0;
        match message {
            WM_COMMAND => match wparam.0 as u32 {
                ID_BTN_RST => {
                    reset_windows(window);
                    LRESULT(0)
                }
                ID_BTN_CNFM => {
                    GetWindowTextW(HANDLE_OPEN, privilege_name_ptr, 30);
                    SetWindowTextW(HANDLE_TIME, privilege_name_ptr);
                    let percentage_to_open_str = text.to_string();
                    PERCENTAGE_TO_OPEN = percentage_to_open_str.parse::<i32>().unwrap_or(0);
                    LRESULT(0)
                }
                ID_BTN_ST => {
                    let hhand = HANDLE_TIME;
                    if IsWindowEnabled(hhand).as_bool() {
                        EnableWindow(hhand, false);
                        SetTimer(window, IDT_TIMER1 as usize, 1000, None);
                    } else {
                        KillTimer(window, IDT_TIMER1 as usize);
                        EnableWindow(hhand, true);
                    }
                    LRESULT(0)
                }
                ID_BTN_INCR => {
                    GetWindowTextW(HANDLE_OPEN, privilege_name_ptr, 30);
                    let percentage_to_open_str = text.to_string();
                    PERCENTAGE_TO_OPEN = percentage_to_open_str.parse::<i32>().unwrap_or(0);
                    let string = format!("{}", PERCENTAGE_TO_OPEN + 10);
                    let str_ref = string.as_str();
                    let mut w_str = WString::from_str(str_ref);
                    let w_str = w_str.as_mut_ptr();
                    SetWindowTextW(HANDLE_OPEN, PWSTR(w_str));
                    LRESULT(0)
                }
                ID_BTN_DECR => {
                    GetWindowTextW(HANDLE_OPEN, privilege_name_ptr, 30);
                    let percentage_to_open_str = text.to_string();
                    PERCENTAGE_TO_OPEN = percentage_to_open_str.parse::<i32>().unwrap_or(0);
                    let string = format!("{}", std::cmp::max(0, PERCENTAGE_TO_OPEN - 10));
                    let str_ref = string.as_str();
                    SetWindowTextW(HANDLE_OPEN, str_ref);
                    LRESULT(0)
                }
                _ => LRESULT(0),
            },
            WM_HSCROLL => {
                if HWND(lparam.0) == HANDLE_TB {
                    let dwPos =
                        SendMessageW(HANDLE_TB, WM_USER, WPARAM::default(), LPARAM::default());
                    let dw = dwPos.0;
                    let string = format!("{}", dw);
                    let str_ref = string.as_str();
                    SetWindowTextW(HANDLE_TIME, str_ref);
                }
                LRESULT(0)
            }
            WM_TIMER => match wparam.0 as u32 {
                IDT_TIMER1 => {
                    GetWindowTextW(HANDLE_TIME, privilege_name_ptr, 30);
                    let percentage_to_open_str = text.to_string();
                    TIME = percentage_to_open_str.parse::<i32>().unwrap_or(0) - 1;
                    if TIME < 1 {
                        KillTimer(window, IDT_TIMER1 as usize);
                        EnableWindow(HANDLE_TIME, true);
                        SetWindowTextW(HANDLE_TIME, "0");
                    } else {
                        let string = TIME.to_string();
                        let str_ref = string.as_str();
                        SetWindowTextW(HANDLE_TIME, str_ref);
                    }
                    LRESULT(0)
                }
                _ => LRESULT(0),
            },

            WM_CREATE => {
                AddControls(window);
                LRESULT(0)
            }
            WM_DESTROY => {
                PostQuitMessage(0);
                LRESULT(0)
            }
            _ => DefWindowProcW(window, message, wparam, lparam),
        }
    }
}

unsafe fn reset_windows(hWnd: HWND) {
    let str_reset = "reset";
    SetWindowTextW(HANDLE_STATUS, str_reset);
    SetWindowTextW(HANDLE_TIME, "0");
    SetWindowTextW(HANDLE_OPEN, str_reset);
    let hhand = HANDLE_TIME;
    if !IsWindowEnabled(hhand).as_bool() {
        KillTimer(hWnd, IDT_TIMER1 as usize);
        EnableWindow(hhand, true);
    }
}

unsafe fn AddControls(hWnd: HWND) {
    let name = "0";

    HANDLE_RST = CreateWindowExW(
        WS_EX_RIGHT,
        "Button",
        "Reset",
        WS_VISIBLE | WS_CHILD,
        10,
        10,
        90,
        80,
        hWnd,
        HMENU(ID_BTN_RST as isize),
        GetModuleHandleW(None),
        std::ptr::null_mut(),
    );
    HANDLE_CNFM = CreateWindowExW(
        WS_EX_RIGHT,
        "Button",
        "Confirm",
        WS_VISIBLE | WS_CHILD,
        10,
        100,
        90,
        80,
        hWnd,
        HMENU(ID_BTN_CNFM as isize),
        GetModuleHandleW(None),
        std::ptr::null_mut(),
    );
    HANDLE_ST = CreateWindowExW(
        WS_EX_RIGHT,
        "Button",
        "Set Time",
        WS_VISIBLE | WS_CHILD,
        10,
        190,
        90,
        80,
        hWnd,
        HMENU(ID_BTN_ST as isize),
        GetModuleHandleW(None),
        std::ptr::null_mut(),
    );
    HANDLE_STATUS = CreateWindowExW(
        WS_EX_CLIENTEDGE,
        "edit",
        name,
        WS_VISIBLE | WS_CHILD,
        110,
        10,
        500,
        80,
        hWnd,
        HMENU(ID_TXT_STATUS as isize),
        GetModuleHandleW(None),
        std::ptr::null_mut(),
    );
    HANDLE_OPEN = CreateWindowExW(
        WS_EX_CLIENTEDGE,
        "edit",
        name,
        WS_VISIBLE | WS_CHILD,
        110,
        100,
        500,
        80,
        hWnd,
        HMENU(ID_TXT_OPEN as isize),
        GetModuleHandleW(None),
        std::ptr::null_mut(),
    );
    HANDLE_TIME = CreateWindowExW(
        WS_EX_CLIENTEDGE,
        "edit",
        name,
        WS_VISIBLE | WS_CHILD,
        110,
        190,
        500,
        80,
        hWnd,
        HMENU(ID_TXT_TIME as isize),
        GetModuleHandleW(None),
        std::ptr::null_mut(),
    );
    HANDLE_DECR = CreateWindowExW(
        WS_EX_RIGHT,
        "Button",
        "decr",
        WS_VISIBLE | WS_CHILD,
        110,
        280,
        90,
        80,
        hWnd,
        HMENU(ID_BTN_DECR as isize),
        GetModuleHandleW(None),
        std::ptr::null_mut(),
    );
    HANDLE_INCR = CreateWindowExW(
        WS_EX_RIGHT,
        "Button",
        "incr",
        WS_VISIBLE | WS_CHILD,
        210,
        280,
        90,
        80,
        hWnd,
        HMENU(ID_BTN_INCR as isize),
        GetModuleHandleW(None),
        std::ptr::null_mut(),
    );
    HANDLE_TB = CreateTrackbar(hWnd, 10, 200, 10, 200);
}

unsafe fn CreateTrackbar(hwndDlg: HWND, iMin: u32, iMax: u32, iSelMin: u32, iSelMax: u32) -> HWND {
    InitCommonControls();

    let hwndTrack: HWND = CreateWindowExW(
        WINDOW_EX_STYLE(0),  // no extended styles
        "msctls_trackbar32", // class name
        "Trackbar Control",  // title (caption)
        WS_CHILD
            | WS_VISIBLE
            | WINDOW_STYLE { 0: TBS_AUTOTICKS }
            | WINDOW_STYLE {
                0: TBS_ENABLESELRANGE,
            }, // style
        310,
        280, // position
        200,
        30,                           // size
        hwndDlg,                      // parent window
        HMENU(ID_TB_SCROLL as isize), // control identifier
        GetModuleHandleW(None),       // instance
        std::ptr::null_mut(),         // no WM_CREATE parameter
    );
    SendMessageW(
        hwndTrack,
        TBM_SETRANGE,
        WPARAM(1),
        LPARAM(make_long(iMin, iMax) as isize),
    );
    SendMessageW(hwndTrack, TBM_SETPAGESIZE, WPARAM(0), LPARAM(10));

    SendMessageW(hwndTrack, TBM_SETTICFREQ, WPARAM(10), LPARAM(0));
    SendMessageW(
        hwndTrack,
        TBM_SETSEL,
        WPARAM(0), // redraw flag
        LPARAM(make_long(iSelMin, iSelMax) as isize),
    );
    SendMessageW(
        hwndTrack,
        TBM_SETPOS,
        WPARAM(1), // redraw flag
        LPARAM(iSelMin as isize),
    );

    hwndTrack
}

pub fn make_long(a: u32, b: u32) -> i64 {
    (((((a as u64) & 0xffff) as u16) as u64) | ((((b as u64) & 0xffff) as u16) as u64) << 16) as i64
}
