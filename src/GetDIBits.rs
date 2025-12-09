pub fn test_GetDIBits(bmp_h:HBITMAP) {
  // currently using deprecated, but much simpler GetBitmapBits API
  // Convert HBITMAP → BGRA bytes
  let mut bmp: BITMAP = unsafe{std::mem::zeroed()};
    // bmType:i32=0   bmPlanes:u16=№color planes (NOT!!! colors)
    // bmWidth ¦ bmHeight	:i32        	//>0 pixels
    // bmWidthBytes      	:i32        	//№bytes in each scan line. ==EVEN because OS assumes that bit values of a bitmap form an array that is word aligned
    // bmBitsPixel       	:u16        	//𝑏⁄𝑝  №𝑏 bits required to indicate the color of a pixel
    // bmBits            	:*mut c_void	//pointer to location of bit values for the bitmap. Its member must be a pointer to an array of character (1-byte) values
  let bmp_sz = unsafe { GetObjectW(bmp_h.into(), std::mem::size_of::<BITMAP>() as i32
    , Some(&mut bmp as *mut BITMAP as *mut c_void));};

  let width  = bmp.bmWidth      as usize;
  let height = bmp.bmHeight     as usize;
  let stride = bmp.bmWidthBytes as usize;
  let bpp    = bmp.bmBitsPixel;
  let buf_size = stride * height;
  let ptr_bmbits = bmp.bmBits; // !! null since we didn't use CreateDIBSection to get bmp_h
  println!("w{width} h{height} wb{stride} {bpp}𝑏⁄𝑝");

  // Get actual bits
  // 1. Deprecated API, but much simpler without DC surfaces
  // let mut cursor_pixels = vec![0u8; buf_size];
  // let bytes = unsafe{ GetBitmapBits(bmp_h, cursor_pixels.len() as i32,  cursor_pixels.as_mut_ptr() as *mut c_void,) };
  // unsafe{std::ptr::copy_nonoverlapping(bmp.bmBits as *const u8, cursor_pixels.as_mut_ptr(), buf_size);}

use windows::Win32::Graphics::Gdi::{HDC,HBITMAP,BITMAPINFO,BITMAPINFOHEADER,RGBQUAD,BI_RGB,DIB_RGB_COLORS,};
use windows::Win32::Graphics::Gdi::{CreateCompatibleDC,CreateCompatibleBitmap,DeleteDC,};
use windows::Win32::UI::WindowsAndMessaging::{GetDesktopWindow,GetSystemMetrics,};
use windows::Win32::UI::WindowsAndMessaging::{SM_CXSCREEN,SM_CYSCREEN,};
const NULL: *mut c_void = 0usize as *mut c_void;
pub type HANDLE = isize;
pub type HINSTANCE = isize;
// pub type BOOL = i32;
pub type BOOLEAN = u8;
pub type NTSTATUS = i32;
pub type LONG = i32;
pub type DWORD = u32;
pub type WORD = u16;

  // 2. GetDIBits
  // https://github.com/alexchandel/screenshot-rs/blob/23e1cc1e417d260e338a8d2e6330e19c35619160/src/lib.rs#L337
  // Enumerate monitors, getting a handle and DC for requested monitor.
  // loljk, because doing that on Windows is worse than death
  unsafe {
  let h_wnd_screen = GetDesktopWindow();
  let h_dc_screen  = GetDC(Some(h_wnd_screen));
  // let width  = GetSystemMetrics(SM_CXSCREEN);
  // let height = GetSystemMetrics(SM_CYSCREEN);

  // Create a Windows Bitmap, and copy the bits into it
  let h_dc = CreateCompatibleDC(Some(h_dc_screen));
  if h_dc == HDC(NULL) { return ();} //Err("Can't get a Windows display.");

  // let h_bmp = CreateCompatibleBitmap(h_dc_screen, width, height);
  // let h_bmp = CreateCompatibleBitmap(h_dc_screen, width.try_into().unwrap(), height.try_into().unwrap());
  // if h_bmp == HBITMAP(NULL) { return ();} //Err("Can't create a Windows buffer");}

  // ———
  let h_bmp = bmp_h;
  // ———

  // let res = SelectObject(h_dc, h_bmp);
  // if res == NULL || res == HGDI_ERROR { return ();} // return Err("Can't select Windows buffer.");

  // let res = BitBlt(h_dc, 0, 0, width, height, h_dc_screen, 0, 0, SRCCOPY|CAPTUREBLT);
  // if res == 0 { return Err("Failed to copy screen to Windows buffer");}

  // 1. If lpvBits is NULL and
  // 2. bit count member of BITMAPINFO is initialized to zero, GetDIBits fills in a BITMAPINFOHEADER structure or BITMAPCOREHEADER without the color table. This technique can be used to query bitmap attributes.
  // Get image info
  let pixel_width: usize = 4; // FIXME
  // let mut bmi = BITMAPINFO { bmiHeader: BITMAPINFOHEADER {biSize:size_of::<BITMAPINFOHEADER>() as DWORD,
  //   biWidth        	: 0, biHeight: 0, // biWidth: width as LONG, biHeight: -(height as LONG),
  //   biPlanes       	:1,
  //   biBitCount     	: 0,//8*pixel_width as WORD,
  //   biCompression  	: BI_RGB.0, // BI_RGB: BI_COMPRESSION = BI_COMPRESSION(0u32);
  //   biSizeImage    	: 0,//(((width * height) as i32) * pixel_width as c_int) as DWORD,
  //   biXPelsPerMeter	: 0, biYPelsPerMeter:0,
  //   biClrUsed      	: 0,
  //   biClrImportant 	: 0,
  //   },     bmiColors: [RGBQUAD {rgbBlue:0,rgbGreen:0,rgbRed:0,rgbReserved:0,}],    };
  let mut bmi_header = BITMAPINFOHEADER::default();
  bmi_header.biSize = size_of::<BITMAPINFOHEADER>() as DWORD; //to have space to write data back to
  let mut bmi = BITMAPINFO::default();
  bmi.bmiHeader = bmi_header;

  /* AFTER ret=1     // BITMAPINF_   BITMAPINFOHEADE_ {
    biSize: 40, // №bytes required by the structure (NOT size of color table / color masks if they are appended to the end of structure. See Remarks.
    w48 h48    Planes:1  biBitCount:32
    biCompression: 3  //BI_BITFIELDS  BI_COMPRESSION(Xu32): 0=BI_RGB   1=BI_RLE8   2=BI_RLE4   3=BI_BITFIELDS   4=BI_JPEG   5=BI_PNG
      BI_RGB       Uncompressed RGB
      BI_BITFIELDS Uncompressed RGB with color masks. Valid for 16-bpp and 32-bpp bitmaps
    biSizeImage: 9216 //??aka stride
    //uncompressed RGB formats: min stride is always the image width in bytes, rounded up to the nearest DWORD, to calculate the stride and image size, you can use the GDI_DIBWIDTHBYTES and/or GDI_DIBSIZE macros, or the following formula:
      // stride = ((((biWidth * biBitCount) + 31) & ~31) >> 3);
    biSizeImage = abs(biHeight) * stride;  //
    biXPelsPerMeter: 0, biYPelsPerMeter: 0, biClrUsed: 0, biClrImportant: 0 }
    bmiColor: [RGBQUAD{rgbBlue:0,rgbGreen:0,rgbRed:0,rgbReserved:0}]}*/

  // Create a Vec for image
  // let size: usize = (width*height) as usize * pixel_width;
  // let mut data: Vec<u8> = Vec::with_capacity(size);
  // data.set_len(size);

  let lpvBits = None; //Some(std::ptr::null_mut()),¦ Some(&mut data[0] as *mut u8 as *mut c_void);
  println!("\n      BEFORE h={height} sz={}\n  {:?}",size_of::<BITMAPINFOHEADER>(),bmi);
  // copy bits into Vec
  let ret = GetDIBits(h_dc, h_bmp,
    0, height as DWORD,
    lpvBits,
    &mut bmi     as *mut BITMAPINFO,
    DIB_RGB_COLORS);
  if ret == 0 {println!("🛑 GetDIBits got nothing = {ret}")};
  /*       	int         	i32                	GetDIBits
    hdc    	HDC         	HDC                	handle to the device context
    hbm    	HBITMAP     	HBITMAP            	handle to the bitmap; must be a compatible bitmap (DDB)
    start  	UINT        	u32                	1st  scan line  to retrieve
    cLines 	UINT        	u32                	№ of scan lines to retrieve
   ←lpvBits	LPVOID      	Option<*mut c_void>	pointer to a buffer to receive the bitmap data. If NULL, pass  dimensions/format of the bitmap to the BITMAPINFO structure pointed to by the lpbmi parameter
   ↔lpbmi  	LPBITMAPINFO	*mut BITMAPINFO    	pointer to a BITMAPINFO struct that specifies the desired format for the DIB data
    usage  	UINT        	DIB_USAGE          	format of the bmiColors member of the BITMAPINFO structure (PAL/RGB)
  */
  println!("\n      AFTER ret={ret}\n  {:?}",bmi);

  // Release native image buffers
  ReleaseDC(Some(h_wnd_screen), h_dc_screen); // don't need screen anymore
  let _ = DeleteDC(h_dc);
  let _ = DeleteObject(h_bmp.into());

  // let data = flip_rows(data, height as usize, width as usize*pixel_width);
  }

  // let dc_window: HDC = GetDC(null_mut());
  // let bitmap_size: usize = (((bitmap.bmWidth * 32 + 31) / 32) * 4 * bitmap.bmHeight) as usize;
  // println!("bitmap size: {}", bitmap_size);
  // let mut buffer: Vec<u8> = vec![0; bitmap_size];

  // let h_dib = GlobalAlloc(GHND, bitmap_size);
  // let lpbitmap = GlobalLock(h_dib);
  // println!("bitmap {:p}", lpbitmap);
  // let mut buffer: Vec<u8> = vec![0; bitmap_size];

  // let mut buffer = vec![0u8; buf_size];
  // unsafe{std::ptr::copy_nonoverlapping(bmp.bmBits as *const u8, buffer.as_mut_ptr(), buf_size);}

  let _ = unsafe{DeleteObject(bmp_h.into())};
}
