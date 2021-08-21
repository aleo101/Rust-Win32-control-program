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
//use windows::*;
// const ID_FILE_MENU_NEW: u32 = 40001;
// const ID_FILE_MENU_OPEN: u32 = 40002;
// const ID_FILE_MENU_EXIT: u32 = 40003;
// const ID_LBL_NAME: u32 = 100;
const ID_TXT_STATUS: u32 = 1;
// const ID_LBL_AGE: u32 = 102;
const ID_TXT_OPEN: u32 = 3;
const ID_BTN_RST: u32 = 4;
const ID_BTN_CNFM: u32 = 5;
const ID_TXT_TIME: u32 = 6;
const ID_BTN_ST: u32 = 7;
const ID_BTN_INCR: u32 = 8;
const ID_BTN_DECR: u32 = 9;
const IDT_TIMER1: u32 = 10;
const ID_TB_SCROLL: u32 = 11;

fn main() {
    run();
}

fn run() {
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
        //debug_assert!(handle == self.handle);
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
        static mut HWNDS: Vec<HWND> = vec![];
        let mut text = WString::new_alloc_buffer(60);
        let privilege_name_ptr = PWSTR(text.as_mut_ptr());
        static mut PERCENTAGE_TO_OPEN: i32 = 0;
        static mut TIME: i32 = 0;
        match message {
            WM_COMMAND => match wparam.0 as u32 {
                ID_BTN_RST => {
                    reset_windows(window, &HWNDS);
                    LRESULT(0)
                }
                ID_BTN_CNFM => {
                    GetWindowTextW(
                        HWNDS.get(ID_TXT_OPEN as usize).unwrap(),
                        privilege_name_ptr,
                        30,
                    );
                    let percentage_to_open_str = text.to_string();
                    PERCENTAGE_TO_OPEN = percentage_to_open_str.parse::<i32>().unwrap_or(0);
                    LRESULT(0)
                }
                ID_BTN_ST => {
                    let hhand = HWNDS.get(ID_TXT_TIME as usize).unwrap();
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
                    GetWindowTextW(
                        HWNDS.get(ID_TXT_OPEN as usize).unwrap(),
                        privilege_name_ptr,
                        30,
                    );
                    let percentage_to_open_str = text.to_string();
                    PERCENTAGE_TO_OPEN = percentage_to_open_str.parse::<i32>().unwrap_or(0);
                    let string = format!("{}", PERCENTAGE_TO_OPEN + 10);
                    let str_ref = string.as_str();
                    let mut w_str = WString::from_str(str_ref);
                    let w_str = w_str.as_mut_ptr();
                    SetWindowTextW(HWNDS.get(ID_TXT_OPEN as usize).unwrap(), PWSTR(w_str));
                    LRESULT(0)
                }
                ID_BTN_DECR => {
                    GetWindowTextW(
                        HWNDS.get(ID_TXT_OPEN as usize).unwrap(),
                        privilege_name_ptr,
                        30,
                    );
                    let percentage_to_open_str = text.to_string();
                    PERCENTAGE_TO_OPEN = percentage_to_open_str.parse::<i32>().unwrap_or(0);
                    let string = format!("{}", std::cmp::max(0, PERCENTAGE_TO_OPEN - 10));
                    let str_ref = string.as_str();
                    SetWindowTextW(
                        HWNDS.get(ID_TXT_OPEN as usize).unwrap(),
                        PWSTR(WString::from_str(str_ref).as_mut_ptr()),
                    );
                    LRESULT(0)
                }
                _ => LRESULT(0),
            },
            WM_HSCROLL => {
                if HWND(lparam.0) == *HWNDS.get(ID_TB_SCROLL as usize).unwrap() {
                    let dwPos = SendMessageW(
                        HWNDS.get(ID_TB_SCROLL as usize).unwrap(),
                        WM_USER,
                        WPARAM::default(),
                        LPARAM::default(),
                    );
                    let dw = dwPos.0;
                    let string = format!("{}", dw);
                    let str_ref = string.as_str();
                    SetWindowTextW(
                        HWNDS.get(ID_TXT_TIME as usize).unwrap(),
                        PWSTR(WString::from_str(str_ref).as_mut_ptr()),
                    );
                }
                LRESULT(0)
            }
            WM_TIMER => match wparam.0 as u32 {
                IDT_TIMER1 => {
                    GetWindowTextW(
                        HWNDS.get(ID_TXT_TIME as usize).unwrap(),
                        privilege_name_ptr,
                        30,
                    );
                    let percentage_to_open_str = text.to_string();
                    TIME = percentage_to_open_str.parse::<i32>().unwrap_or(0) - 1;
                    if TIME < 1 {
                        KillTimer(window, IDT_TIMER1 as usize);
                        EnableWindow(HWNDS.get(ID_TXT_TIME as usize).unwrap(), true);
                    }
                    let string = format!("{}", std::cmp::max(0, TIME));
                    let str_ref = string.as_str();
                    SetWindowTextW(
                        HWNDS.get(ID_TXT_TIME as usize).unwrap(),
                        PWSTR(WString::from_str(str_ref).as_mut_ptr()),
                    );
                    LRESULT(0)
                }
                _ => LRESULT(0),
            },
            WM_PARENTNOTIFY => match (wparam.0 as u64) as u16 as u32 {
                WM_CREATE => {
                    HWNDS.insert(
                        ((wparam.0 as u64) >> 16) as u16 as usize,
                        HWND(lparam.0),
                    );
                    println!(
                        "insert working: {} : {}.",
                        ((wparam.0 as u64) >> 16) as u16 as u32, 
                        HWND(lparam.0).0,
                    );
                    LRESULT(0)
                }
                _ => LRESULT(0),
            },
            WM_CREATE => {
                HWNDS = vec![HWND::default(); 10];
                AddControls(window);
                LRESULT(0)
            }
            WM_DESTROY => {
                PostQuitMessage(0);
                LRESULT(0)
            }
            _ => DefWindowProcW(window, message, wparam, lparam),
        }
        // if message == WM_NCCREATE {
        //     let cs = lparam.0 as *const CREATESTRUCTA;
        //     let this = (*cs).lpCreateParams as *mut Self;
        //     (*this).handle = window;
        //     SetWindowLong(window, GWLP_USERDATA, this as _);
        // } else {
        //     let this = GetWindowLong(window, GWLP_USERDATA) as *mut Self;
        //     if !this.is_null() {
        //         return (*this).message_handler(message, wparam, lparam);
        //     }
        // }
    }
}

// #[allow(non_snake_case)]
// #[cfg(target_pointer_width = "64")]
// unsafe fn SetWindowLong(window: HWND, index: WINDOW_LONG_PTR_INDEX, value: isize) -> isize {
//     SetWindowLongPtrA(window, index, value)
// }

// #[allow(non_snake_case)]
// #[cfg(target_pointer_width = "32")]
// unsafe fn GetWindowLong(window: HWND, index: WINDOW_LONG_PTR_INDEX) -> isize {
//     GetWindowLongA(window, index) as _
// }

// #[allow(non_snake_case)]
// #[cfg(target_pointer_width = "64")]
// unsafe fn GetWindowLong(window: HWND, index: WINDOW_LONG_PTR_INDEX) -> isize {
//     GetWindowLongPtrA(window, index)
// }

unsafe fn reset_windows(hWnd: HWND, HWNDS: &[HWND]) {
    let str_reset = "reset";
    SetWindowTextW(HWNDS.get(ID_TXT_STATUS as usize).unwrap(), str_reset);
    SetWindowTextW(HWNDS.get(ID_TXT_TIME as usize).unwrap(), str_reset);
    SetWindowTextW(HWNDS.get(ID_TXT_OPEN as usize).unwrap(), str_reset);
    let hhand = HWNDS.get(ID_TXT_TIME as usize).unwrap();
    if !IsWindowEnabled(hhand).as_bool() {
        KillTimer(hWnd, IDT_TIMER1 as usize);
        EnableWindow(hhand, true);
    }
}

unsafe fn AddControls(hWnd: HWND) {
    let name = "0";

    CreateWindowExW(
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
    CreateWindowExW(
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
    CreateWindowExW(
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
    CreateWindowExW(
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
    CreateWindowExW(
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
    CreateWindowExW(
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
    CreateWindowExW(
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
    CreateWindowExW(
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
    //CreateWindowExW(0, TRACKBAR_CLASS, ("Scrolly"), WS_VISIBLE|WS_CHILD| TBS_AUTOTICKS | TBS_ENABLESELRANGE, 210, 380, 200, 40, hWnd, HMENU(ID_TB_SCROLL), instance, std::ptr::null_mut());
    CreateTrackbar(hWnd, 10, 200, 10, 200);
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
