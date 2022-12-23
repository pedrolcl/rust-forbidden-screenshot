use windows::{
    core::*, 
    Win32::Foundation::*, 
    Win32::Graphics::Gdi::*,
    Win32::System::LibraryLoader::GetModuleHandleA, 
    Win32::UI::WindowsAndMessaging::*,
};

fn main() -> Result<()> {
    unsafe {
        let instance = GetModuleHandleA(None)?;
        debug_assert!(instance.0 != 0);

        let window_class = s!("window");

        let wc = WNDCLASSA {
            hCursor: LoadCursorW(None, IDC_ARROW)?,
            hInstance: instance,
            lpszClassName: window_class,

            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(wndproc),
            ..Default::default()
        };

        let atom = RegisterClassA(&wc);
        debug_assert!(atom != 0);

        let win = CreateWindowExA(WINDOW_EX_STYLE::default(), window_class, s!("Confidential window"), WS_OVERLAPPEDWINDOW | WS_VISIBLE, CW_USEDEFAULT, CW_USEDEFAULT, 320, 240, None, None, instance, None);

        SetWindowDisplayAffinity(win, WDA_MONITOR);

        let mut message = MSG::default();

        while GetMessageA(&mut message, HWND(0), 0, 0).into() {
            DispatchMessageA(&message);
        }

        Ok(())
    }
}

extern "system" fn wndproc(window: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    unsafe {
        match message {
            WM_CREATE => {
                println!("WM_CREATE");
                // let affinity = WDA_MONITOR; WDA_NONE
                // SetWindowDisplayAffinity(window, affinity);
                LRESULT(0)
            }
            WM_PAINT => {
                println!("WM_PAINT");
                let mut msg =  String::from("You cannot take screenshots of this!");
                let mut ps = PAINTSTRUCT::default();
                let psp = &mut ps as *mut PAINTSTRUCT;
                let rectp = &mut ps.rcPaint as *mut RECT;
                let hdc = BeginPaint(window, psp);
                let brush = CreateSolidBrush(COLORREF(0x0000F0F0)); // yellow
                // All painting occurs here, between BeginPaint and EndPaint.
                FillRect(hdc, &ps.rcPaint, brush);
                DrawTextA(hdc, msg.as_bytes_mut(), rectp, DT_SINGLELINE | DT_CENTER | DT_VCENTER);
                EndPaint(window, &ps);
                LRESULT(0)
            }
            WM_DESTROY => {
                println!("WM_DESTROY");
                PostQuitMessage(0);
                LRESULT(0)
            }
            _ => DefWindowProcA(window, message, wparam, lparam),
        }
    }
}
