#Requires AutoHotKey 2.1-alpha.18

; Add this library to your ‘../lib’ folder
; See ‘mouse_sz_example.ahk’
/* use mouse_sz_lib via a loader class
  static mouse_sz := mouse_sz_lib_loader.load()
  , CI    	:= mouse_sz.get_mcursor_sz_ci    .Bind(mouse_sz)
  , CIbox 	:= mouse_sz.get_mcursor_sz_ci_box.Bind(mouse_sz)
  , DXD   	:= mouse_sz.get_mcursor_sz_dx    .Bind(mouse_sz)
  , DXDbox	:= mouse_sz.get_mcursor_sz_dx_box.Bind(mouse_sz)

  cur_pos := DXDbox()
*/


class ffi_Struct { ; Various structs / typed properties for FFI libraries
  class iPoint {
    x : i32
    y : i32
  }

  class cur_box {
    ptl : ffi_Struct.iPoint
    pbr : ffi_Struct.iPoint
    hs  : ffi_Struct.iPoint
  }
}

class Coord { ; Cursor coordinate type
  static Mon := 0 ; Absolute monitor pixel position
  static Box := 1 ; Relative to the cursor icon box
  static NoShadow := 0 ; Do not adjust for shadow
  static Shadow := 1 ; Adjust for shadow
}

class mouse_sz_lib_loader {
  static m := Map('c',0,
    'libPath','lib\',
    'libName','mptrsz'  )
  static load() {
    if mouse_sz_lib_loader.m['c'] == 0 {
       mouse_sz_lib_loader.m['c'] := Cmouse_sz_lib(mouse_sz_lib_loader.m['libPath'], mouse_sz_lib_loader.m['libName'])
    }
    return mouse_sz_lib_loader.m['c']
  }
  static unload(t) {
    mouse_sz_lib_loader.m['c'].unload()
    mouse_sz_lib_loader.m['c'] := 0
  }
}
class Cmouse_sz_lib { ; Various win32 API constants from a memory-mapped file
  __new(libPath,libNm) {
    this.libPath  	:= libPath
     ,this.libNm  	:= libNm
     ,this.ℯsz    	:= 200 ; max limit of error size to get
     ,this.hModule	:= 0
     ,this.lib𝑓_ci	:= this.libNm '\' 'get_mcursor_sz_ci'
     ,this.lib𝑓_dx	:= this.libNm '\' 'get_mcursor_sz_dx'
     ,this.CI     	:= DllCall.Bind(this.lib𝑓_ci, ffi_Struct.cur_box,unset, 'UInt',Coord.Mon, 'UInt',Coord.Shadow) ; screen coordinates
     ,this.CIbox  	:= DllCall.Bind(this.lib𝑓_ci, ffi_Struct.cur_box,unset, 'UInt',Coord.Box, 'UInt',Coord.Shadow) ; cursor icon box coordinates
     ,this.DXD    	:= DllCall.Bind(this.lib𝑓_dx, ffi_Struct.cur_box,unset, 'UInt',Coord.Mon)
     ,this.DXDbox 	:= DllCall.Bind(this.lib𝑓_dx, ffi_Struct.cur_box,unset, 'UInt',Coord.Box)
     ,this.free   	:= DllCall.Bind(this.libNm '\dealloc_lib_string', 'Int',unset, 'Int')
     ; fn get_mcursor_sz_dx(mut cur_box:cur_box, coord:i8) -> *mut WideChar

     ,this.dealloc_lib_string	:= this.free

    if this.hModule == 0 {
      this.load()
    ;   msgbox('loaded in new `n:' this.lib𝑓_ci '`n:' this.lib𝑓_dx '`n@' this.libPath '`n:' this.libNm)
    ; } else {
    ;   msgbox(this.hModule)
    }
  }
  get_mcursor_sz_ci() {
    𝑓:=this.CI
    return this.get_cursor_sz(𝑓)
  }
  get_mcursor_sz_ci_box() {
    𝑓:=this.CIbox
    return this.get_cursor_sz(𝑓)
  }
  get_mcursor_sz_dx() {
    𝑓:=this.DXD
    return this.get_cursor_sz(𝑓)
  }
  get_mcursor_sz_dx_box() {
    𝑓:=this.DXDbox
    return this.get_cursor_sz(𝑓)
  }
  get_cursor_sz(𝑓) {
    cFree := this.free
    cur_box := ffi_Struct.cur_box()
    if (ptr𝑒 := 𝑓(cur_box)) {
      ℯmsg	:= StrGet(ptr𝑒,,'UTF-16')
      cFree(ptr𝑒)
      ; throw ValueError(ℯmsg, -2) ; -1 AHK bug? corrupts display '✗ Value', when calling getKey(𝑓,key) directly, displays fine
      ; ToolTip(0,'errror : ' ℯmsg,5)
      return false ; client can fallback to regular cursor position disregarding cursor size
    } else {
      return cur_box
    }
  }
  load() {
    hModule	:= DllCall("LoadLibrary", "Str",this.libPath this.libNm '.dll', "Ptr")  ; Avoids the need for DllCall in the loop to load the library
    this.hModule:=hModule
    ; msgbox('loaded = ' this.libPath this.libNm '`ninto address: ' this.hModule)
  }
  unload() {
    DllCall("FreeLibrary", "Ptr",this.hModule)  ; to conserve memory, the DLL may be unloaded after using it
    this.hModule:=0
  }
  ;__delete() { ; uncomment to autodelete
  ;  if this.hModule != 0 {
  ;    this.unload()
  ;    ; msgbox('__delete')
  ;  } else {
  ;    ; msgbox('__delete already no hModule')
  ;  }
  ;}
}
