<p align="center">
A Windows DLL library that calculates the true size of the mouse pointer,
<br>
accounting for the screen resolution scaling, Accessibility "size" multiplier, and pointer shadow.
</p>
<p align="center">  
(the icon itself, not the nominal box that contains it where I-beam = 🮰Arrow)
</p>
<p align="center">  
AutoHotkey example included
</p>


## Introduction

There are a few ways to get the true pointer size on Windows, though all of them are deficient in various ways.

One direct way is to get the [SM_CXCURSOR | …Y…](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getsystemmetrics) system metric. However, that's not the actually used pointer, but the default one, and also not the size of the pointer, but the box containing it, so a  **32⋅32** square (for <150% scaled screen resolution, the box scales as [1.5, 2, 3, 4](https://devblogs.microsoft.com/oldnewthing/20210819-00/?p=105572)).

Another more convoluted one is to [GetCursorInfo](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getcursorinfo) with its monochrome mask and color mask/values that represent the current pointer, but that's also not the actual size, but the size of the nominal box, so  **32⋅32** for a non-squared I-beam (though dpi-adjusted, so **48⋅48** for 150% scaled screen resolution), and also fails further by ignoring that:
  - you have increased the pointer size by **⋅2** in **Settings**🢖**Accessibility**🢖**Mouse pointer and touch**🢖**Size** (→ **72⋅72**[^1])
  - you have added pointer shadow in **Settings**🢖**Accessibility**🢖**Mouse pointer and touch**🢖**Enable mouse pointer shadow** (→ ≈**81⋅79**[^2])

But at least you can parse the bitmasks and extract the mouse pointer from its box.

A more precise convoluted Windows API [DX Duplication](https://learn.microsoft.com/en-us/windows/win32/api/dxgi1_2/ns-dxgi1_2-dxgi_outdupl_pointer_shape_info) takes the full screenshot of the whole screen(!) and then extracts the mouse pointer, wasting time and memory in the process (it's orders of magnitude slower than the `GetCursorInfo`), but at least you get halfway there — screen scale, accessibility size, and pointer shadow are all accounted for in the screenshot, though you still only get the nominal box, including empty pixels (though you can parse the masks and exclude them).

So this library parses the pointer bitmasks from each of the APIs to get the actual pointer instead of its box, and for the incomplete `GetCursorInfo` partially completes it adjusting for the Accessibility multiplier (but not the shadow) to get the actual pointer size.

A summary of the various options and their limitations:

|                          	| SM 	| CI 	| CI 	| DX	| DX 	| Comment                                          	|
|--------------------------	|----	|----	|----	|---	|--- 	|--------------------------------------------------	|
|                          	|    	| ≝  	| dll	| ≝ 	| dll	|                                                  	|
| Current pointer, not ≝   	| −  	| ✓  	| ✓  	| ✓ 	| ✓  	|                                                  	|
| Pointer, not its box     	| −  	| −  	| ✓  	| − 	| ✓  	|                                                  	|
| 🖵 Screen resolution scale	| ✓  	| ✓  	| ✓  	| ✓ 	| ✓  	| at set 1.5, 2, 3, 4 factors                      	|
| ♿ Accessibility size     	| −  	| −  	| ≈✓ 	| ✓ 	| ✓  	| ≈ off by a few pixels                            	|
| ❏ Shadow                 	| −  	| −  	| −  	| ✓ 	| ✓  	|                                                  	|
| Performance              	| ✓  	| ✓  	| ✓  	| ✗ 	| ✗  	| DX is orders of magnitude slower/uses more memory	|

## Install

(for AutoHotkey)
  - Copy the `mouse_sz.ahk` AutoHotkey library to your `…\Autohotkey\lib` library folder
  - Copy the `mptrsz.dll` Windows library to the same folder (or adjust path in `mouse_sz.ahk` to point to another location)

## Use

The library provides 2 functions `get_mcursor_sz_ci` and `get_mcursor_sz_dx` that can report values in nominal or screen coordinates using either the GetCursorInfo or the DX duplication (screenshot) APIs.

See `mouse_sz_example.ahk` that uses the `mouse_sz.ahk` library to show the nominal+screen coordinate sizes of the current mouse pointer using both, also showing the time each operation took.

You can then use this information to, e.g., show a tooltip that doesn't overlap with the bigger accessibility-sized pointer.

## Known issues
  - Precise shadow size for the `get_mcursor_sz_ci` method is not calculated by this library
  - I don't know whether it's possible to take the screenshot of only the pointer and not the whole screen with the DX duplication API, and anyway the Rust library used doesn't [support it](https://github.com/DiscreteTom/rusty-duplication/issues/10), maybe this could improve performance

## Credits

[^1]: Accessibility increases the cursor size by ½ of its height with each level
[^2]: Precise shadow size is not calculated by this library
