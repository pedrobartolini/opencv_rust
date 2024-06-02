use winapi::shared::windef::HWND;
use winapi::um::winuser::EnumWindows;
use winapi::um::winuser::GetWindowTextW;
use winapi::um::winuser::IsWindowVisible;

mod attached;

pub use attached::AttachedWindow;

pub struct Window {
   hwnd:      HWND,
   pub title: String
}

impl Window {
   pub fn attach(self) -> attached::AttachedWindow {
      attached::AttachedWindow::new(self)
   }
}

unsafe extern "system" fn enum_windows_proc(hwnd: HWND, windows: isize) -> i32 {
   const MAX_TITLE_LENGTH: usize = 100;

   let mut buffer: Vec<u16> = vec![0; MAX_TITLE_LENGTH];

   GetWindowTextW(hwnd, buffer.as_mut_ptr(), MAX_TITLE_LENGTH as i32);

   let title = String::from_utf16_lossy(&buffer).trim_matches('\0').to_string();

   if !title.is_empty() && IsWindowVisible(hwnd) == 1 {
      let windows = &mut *(windows as *mut Vec<Window>);

      windows.push(Window {
         hwnd,
         title: String::from_utf16_lossy(&buffer)
      });
   }

   1
}

pub fn get_windows() -> Vec<Window> {
   let mut windows: Vec<Window> = Vec::new();

   unsafe {
      EnumWindows(Some(enum_windows_proc), &mut windows as *mut _ as isize);
   }

   windows
}
