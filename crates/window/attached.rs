use opencv::core::Mat;
use opencv::core::MatTraitConst;
use opencv::core::CV_8UC3;
use winapi::ctypes::c_void;
use winapi::shared::minwindef::WPARAM;
use winapi::shared::windef::HDC__;
use winapi::shared::windef::HWND;
use winapi::shared::windef::RECT;
use winapi::um::wingdi::BitBlt;
use winapi::um::wingdi::CreateCompatibleBitmap;
use winapi::um::wingdi::CreateCompatibleDC;
use winapi::um::wingdi::DeleteObject;
use winapi::um::wingdi::GetDIBits;
use winapi::um::wingdi::SelectObject;
use winapi::um::wingdi::BITMAPINFO;
use winapi::um::wingdi::BITMAPINFOHEADER;
use winapi::um::wingdi::BI_RGB;
use winapi::um::wingdi::DIB_RGB_COLORS;
use winapi::um::wingdi::RGBQUAD;
use winapi::um::wingdi::SRCCOPY;
use winapi::um::winuser::GetAsyncKeyState;
use winapi::um::winuser::GetDC;
use winapi::um::winuser::GetForegroundWindow;
use winapi::um::winuser::GetWindowRect;
use winapi::um::winuser::IsWindow;
use winapi::um::winuser::SendMessageW;
use winapi::um::winuser::WM_KEYDOWN;
use winapi::um::winuser::WM_KEYUP;

#[derive(Clone)]
pub struct AttachedWindow {
   pub title: String,

   pub hwnd: HWND,

   screen_dc: *mut HDC__,
   memory_dc: *mut HDC__
}

unsafe impl Send for AttachedWindow {}

impl AttachedWindow {
   pub fn new(window: super::Window) -> Self {
      unsafe {
         let screen_dc = GetDC(window.hwnd);
         let memory_dc = CreateCompatibleDC(screen_dc);

         Self {
            screen_dc,
            memory_dc,
            title: window.title,
            hwnd: window.hwnd
         }
      }
   }

   pub fn is_alive(&self) -> bool {
      unsafe { IsWindow(self.hwnd) != 0 }
   }

   pub fn size(&self) -> (i32, i32) {
      let mut rect: RECT = RECT {
         left:   0,
         top:    0,
         right:  0,
         bottom: 0
      };

      unsafe { GetWindowRect(self.hwnd, &mut rect) };

      (rect.right - rect.left, rect.bottom - rect.top)
   }

   pub fn is_focused(&self) -> bool {
      unsafe { GetForegroundWindow() == self.hwnd }
   }

   pub fn send_key(&self, key_code: i32) {
      unsafe {
         SendMessageW(self.hwnd, WM_KEYDOWN, key_code as WPARAM, 0);
         SendMessageW(self.hwnd, WM_KEYUP, key_code as WPARAM, 0);
      }
   }

   pub fn key_state(&self, key_code: i32) -> bool {
      unsafe { (GetAsyncKeyState(key_code) as u16 & 0x8000u16) != 0 && self.is_focused() }
   }

   pub fn capture_screen(&self) -> Mat {
      let (width, height) = self.size();

      unsafe {
         let bitmap = CreateCompatibleBitmap(self.screen_dc, width, height);
         SelectObject(self.memory_dc, bitmap as *mut c_void);
         BitBlt(self.memory_dc, 0, 0, width, height, self.screen_dc, 0, 0, SRCCOPY);

         let bitmap_header = BITMAPINFOHEADER {
            biSize:          std::mem::size_of::<BITMAPINFOHEADER>() as u32,
            biWidth:         width,
            biHeight:        -height,
            biPlanes:        1,
            biBitCount:      24,
            biCompression:   BI_RGB,
            biSizeImage:     0,
            biXPelsPerMeter: 0,
            biYPelsPerMeter: 0,
            biClrUsed:       0,
            biClrImportant:  0
         };

         const EMPTY_COLOR: [RGBQUAD; 1] = [RGBQUAD {
            rgbBlue:     0,
            rgbGreen:    0,
            rgbRed:      0,
            rgbReserved: 0
         }];

         let mut bitmap_info = BITMAPINFO {
            bmiHeader: bitmap_header,
            bmiColors: EMPTY_COLOR
         };

         let mat = Mat::new_rows_cols(height, width, CV_8UC3).expect("Error creating mat for screenshot.");

         GetDIBits(self.memory_dc, bitmap, 0, height as u32, mat.data() as *mut c_void, &mut bitmap_info, DIB_RGB_COLORS);

         DeleteObject(bitmap as *mut c_void);

         mat
      }
   }
}
