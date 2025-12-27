#Requires AutoHotKey 2.1-alpha.18
  F4::{
    Tooltip "AutoHotKey Reloading!"
    sleep(500)
    SetTimer () => ToolTip(), -500 ; killed on Reload, but helpful if reload fails
    Reload
    }


#Include <mouse_sz>

F3::show_🖰Mouse_cursor_size()

show_🖰Mouse_cursor_size() {
  static mouse_sz := mouse_sz_lib_loader.load()
  , CI    	:= mouse_sz.get_mcursor_sz_ci    .Bind(mouse_sz)
  , CIbox 	:= mouse_sz.get_mcursor_sz_ci_box.Bind(mouse_sz)
  , DXD   	:= mouse_sz.get_mcursor_sz_dx    .Bind(mouse_sz)
  , DXDbox	:= mouse_sz.get_mcursor_sz_dx_box.Bind(mouse_sz)

  iters := 1
  xΔ := 220
  🕐w := 10
  id:=4
  x:=-xΔ

  🕐1 := preciseTΔ() ; 0.337
  loop iters {
    cur_pos := DXDbox()
  }
  🕐2 := preciseTΔ()
  (cur_pos=0)?'':(ToolTip('DXD`tx`ty'
    '`n'  '↖`t' cur_pos.ptl.x '`t' cur_pos.ptl.y
    '`n'  '↘`t' cur_pos.pbr.x '`t' cur_pos.pbr.y
    '`n' 'hs`t' cur_pos.hs.x  '`t' cur_pos.hs.y
    '`n' '↔↕`t' cur_pos.pbr.x - cur_pos.ptl.x '`t' cur_pos.pbr.y - cur_pos.ptl.y
    '`n' 'icon box ' format(" 🕐Δ{:.3f}",(🕐2-🕐1)/iters)
    ,x+=xΔ,0,id+=1)  )

  🕐1 := preciseTΔ()
  loop iters {
    cur_pos := CIbox() ; 0.002
  }
  🕐2 := preciseTΔ()
  (cur_pos=0)?'':(ToolTip('CI `tx`ty'
    '`n'  '↖`t' cur_pos.ptl.x '`t' cur_pos.ptl.y
    '`n'  '↘`t' cur_pos.pbr.x '`t' cur_pos.pbr.y
    '`n' 'hs`t' cur_pos.hs.x  '`t' cur_pos.hs.y
    '`n' '↔↕`t' cur_pos.pbr.x - cur_pos.ptl.x '`t' cur_pos.pbr.y - cur_pos.ptl.y
    '`n' 'icon box ' format(" 🕐Δ{:.3f}",(🕐2-🕐1)/iters)
    ,x+=xΔ,0,id+=1)  )


  🕐1 := preciseTΔ() ; 0.337
  loop iters {
    cur_pos := DXD()
  }
  🕐2 := preciseTΔ()
  (cur_pos=0)?'':(ToolTip('DXD`tx`ty'
    '`n'  '↖`t' cur_pos.ptl.x '`t' cur_pos.ptl.y
    '`n'  '↘`t' cur_pos.pbr.x '`t' cur_pos.pbr.y
    '`n' 'hs`t' cur_pos.hs.x  '`t' cur_pos.hs.y
    '`n' '↔↕`t' cur_pos.pbr.x - cur_pos.ptl.x '`t' cur_pos.pbr.y - cur_pos.ptl.y
    '`n' 'screen ' format(" 🕐Δ{:.3f}",(🕐2-🕐1)/iters)
    ,x+=xΔ,0,id+=1)  )


  🕐1 := preciseTΔ()
  loop iters {
    cur_pos := CI() ; 0.002
  }
  🕐2 := preciseTΔ()
  (cur_pos=0)?'':(ToolTip('CI `tx`ty'
    '`n'  '↖`t' cur_pos.ptl.x '`t' cur_pos.ptl.y
    '`n'  '↘`t' cur_pos.pbr.x '`t' cur_pos.pbr.y
    '`n' 'hs`t' cur_pos.hs.x  '`t' cur_pos.hs.y
    '`n' '↔↕`t' cur_pos.pbr.x - cur_pos.ptl.x '`t' cur_pos.pbr.y - cur_pos.ptl.y
    '`n' 'screen ' format(" 🕐Δ{:.3f}",(🕐2-🕐1)/iters)
    ,x+=xΔ,0,id+=1)  )
}



preciseTΔ(n:=3) {
  static start := nativeFunc.GetSystemTimePreciseAsFileTime()
  t := round(     nativeFunc.GetSystemTimePreciseAsFileTime() - start,n)
  return t
}
class nativeFunc {
  static GetSystemTimePreciseAsFileTime() {
    /* learn.microsoft.com/en-us/windows/win32/api/sysinfoapi/nf-sysinfoapi-getsystemtimepreciseasfiletime
    retrieves the current system date and time with the highest possible level of precision (<1us)
    FILETIME structure contains a 64-bit value representing the number of 100-nanosecond intervals since January 1, 1601 (UTC)
    100 ns  ->  0.1 µs  ->  0.001 ms  ->  0.00001 s
    1     sec  ->  1000 ms  ->  1000000 µs
    0.1   sec  ->   100 ms  ->   100000 µs
    0.001 sec  ->    10 ms  ->    10000 µs
    */
    static interval2sec := (10 * 1000 * 1000) ; 100ns * 10 → µs * 1000 → ms * 1000 → sec
    DllCall("GetSystemTimePreciseAsFileTime", "int64*",&ft:=0)
    return ft / interval2sec
  }
}
