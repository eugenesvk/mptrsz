# bug
  - why is that when it's actually visible??? Acc15 global 🖰 𝑏map DX: no mouse pointer shape captured: DX Duplication error: Pointer is Hidden, so has no size! although CI gets the info

# Minor
  - calculate a point closest to each corner for precise tooltip positioning at the central point

# Bug
  - manual accessibility scaling isn't precise, maybe due to rounding (official rounds to the even number of pixels?)
  - (can't) how to get mask size with shadow like DXGI does?
    - any way to approximately calculate ourselves?
    + ?not needed, not a true size for tooltips without a shadow dxgi: how to ignore shadow and get only the   size of the cursor itself?

# Sources
  - [incomplete official guide](https://devblogs.microsoft.com/oldnewthing/20210820-00/?p=105593) ignores Acc, doesn't show how to iterate bitmasks
  - [get-correct-cursor-image-from-windows-api](https://stackoverflow.com/questions/70553039/get-correct-cursor-image-from-windows-api)
  - [GetCursorInfo only return a 32 x 32 bitmap for compatibility reasons](https://www.autohotkey.com/boards/viewtopic.php?t=75867)
  - [test cursor types](http://elektronotdienst-nuernberg.de/bugs/cursor.html) even those that windows doesn't show like grab hand

# Misc
!! Add detection of to `measure` non DXGI of masked 
  - 24b no   α, but 🆭 = 32bit (in α channel) vs    ColorMasked
  - 32b with α, but 🆭 = 32bit                      Color
  - and update printout similar to full color
  - use `1VertLine 32b 𝑡Color 1px BWRGB α 48px` to test

# Mask tests
! there is no MAsked color in nonDXGI, it's regular mono bitmask AND and color bitmask XOR
!bug: `i_color_0%white (24inv→32)` shows 0% white in 32b as though it's in 24b and is inverted, but 32b has no inversion, how to differentiate?
Logic
  - if 𝑏mask is 1 (no cursor, base=screen), then color 𝑏map:
    - 32𝑏: should be all 0s since it's not a mask and can't have α=🆭=1=invert and can't have α=🆭=0=replace (if it did, 𝑏map AND would be 0, not 1) and can't have α>0 transparency since again 𝑏map AND would be 0
      - ! so if any pixel is NOT 0,0,0,0, then it's 24𝑏 masked
      - ! if 255,255,255,0 = color masked since a transparent color would be 0,0,0,0
    - 24𝑏: color       : same logic as 32𝑏
    - 24𝑏: color masked: should only have data for inverted pixels or b,g,r,0 to XOR screen pixel
      - inverted: AND=1 unchanged, but 255,255,255,0 with α=0 since it's 24𝑏, not DXGI, which hacks 24𝑏 to add a 0/255 mask in the α channel, so this means 1₈ XOR Δ¡ inverted
      - ? any other value possible outside of inverted? 1,2,3,0 or 1,2,3,255
      - ! if 255,255,255,0 = color masked
    + !!! if 255,255,255,0 = color masked
    - ✗✗✗ NO, 0% white cursor is NOT inverted colors despite the fact the AND=1 and XOR=255,255,255,0
    - so only if α>0 anywhere is this a proper 32𝑏?
      - very logical, I got confused by color masked in DXGI!!!
      - but how does the OS detect this? does it iterate over every pixel??? guess it's taken from cursor file
        - but then iconinfo API should detect 24b vs 32b

24𝑏 color invert             : ⋀1≝   ⊻Δ¡255,255,255,0  
32𝑏 color 0%white            : ⋀1≝   ⊻Δ 255,255,255,0  
  /                                     ↑ not a mask, so doesn't Δ🗘 invert, NO, it DOES invert!
  but where is this difference recorded on Windows??? how is ↑ different from 24𝑏 with a mask?
  !!! it's NOT different, the effect is identical: inverted color
32𝑏 color 0%white→transparent: ⋀1≝   ⊻≝   0,  0,  0,0
Δ¡Δ🗘
↓ same behavior for identical data when it's alone eve
24𝑏 1px α=0   White dxColorMasked: ⋀1≝   ⊻Δ¡ 255,255,255,  0  ¦DX 255,255,255,255🆭=XOR   effect: invert
32𝑏 1px α=0   White dxColorMasked: ⋀1≝    Δ  255,255,255,  0α ¦DX 255,255,255,255🆭=XOR   effect: invert
**BUT** when α is present in other pixels, then same values have a different effect: no more inversion and DX reports different colors
32𝑏 1px α=0   White dxColorMasked: ⋀1≝   ⊻Δ¡ 255,255,255,  0  ¦DX 255,255,255,255🆭=XOR   effect: invert
32𝑏 2px α=0   White dxColor      : ⋀1≝    Δ  255,255,255,  0α ¦DX   0,  0,  0,  0α=trans effect: trans
32𝑏 2px transparent dxColorMasked: ⋀1≝   ⊻≝    0,  0,  0,  0  ¦DX   0,  0,  0,255🆭=XOR   effect: trans
32𝑏 2px transparent dxColor      : ⋀1≝    Δ  255,255,255,255α ¦DX   0,  0,  0,  0α=trans effect: trans
32𝑏 2px α=255 White dxColorMasked: ⋀0Δ   ⊻Δ¡ 255,255,255,  0  ¦DX 255,255,255,  0🆭=repl  effect: white
32𝑏 2px α=255 White dxColor      : ⋀0Δ    Δ  255,255,255,255α ¦DX 255,255,255,255α       effect: white
32𝑏 2px α=199 Grey  dxColor      : ⋀0Δ    Δ  255,255,255,199α ¦DX 199,199,199,199α       effect: grey 
  ?? why not 255,255,255,199? is it "premultiplied"?
# rule: if α>0 in hColor (only when ⋀1 or doesn't matter) then it's ColorMasked?
  - since ColorM can't have α>0, its XOR invert/replace effect is always active without relying on α
  ? can ColorMasked have 255 mask in non-dx? or is it remnants of my DX confusion
    - what if it's blank? then it doesn't matter, it's not masked since there is no invert and no 1₈,0
  - Update description that COlor is not MAsked, so it's not XOR, but just pixels colored

# ✓ ± fix lack of accesibility sizing in the getinfo cursor masks:
  - bounding box is wrong! due to `accessibility`
    - seems like only DX Duplication API an handle it
    - `GetCursorInfo` always returns `32⋅32` (dpi-scaled to the monitor)
    - is this needed? or does the API handle it itself? yes, needed for cursorinfo still!!
     - bitmask for some reason is all 0s, likely some wrong iteration?
      -  HKEY_CURRENT_USER\SOFTWARE\Microsoft\Accessibility\CursorSize
      or HKEY_CURRENT_USER\Control Panel\Cursors\CursorBaseSize
  - get size as now
  - multiply it by the calc factor ca (see Excel)
    - The second way was found here: we can get cursor size multiplier form registry value CursorSize under HKEY_CURRENT_USER\Software\Microsoft\Accessibility and then calculate the cursor size yourself. It can be done somehow like this: newHeight = cursorHeight + (multiplier - 1) * (cursorHeight / 2); where cursorHeight is value from GetSystemMetrics(SM_CYCURSOR) and multiplier is value from the registry. The cursor real width value will be the same as newHeight. All values are unsigned long if we use C++.`
± 1pixel off vs DX duplication
  - white Acc2 compare Arrow
    ←1–20→=20 ↑0–28↓=29 bound box PRE accessibility scaling (⋅1.5)
    ←2–30→=29 ↑0–42↓=43 bound box (¬0 px, 0-based coords)
    ←1–31→=31 ↑0–43↓=44 true bounding box (non0 pixels, 0-based coords )
  - white Acc2 compare iBeam
    ←16–32→=17 ↑10–37↓=28 bound box PRE accessibility scaling (⋅1.5)
    ←24–48→=25 ↑15–56↓=42 bound box (¬0 px, 0-based coords)
    ←23–48→=26 ↑15–56↓=42 true bounding box (non0 pixels, 0-based coords )

# ± Can't do:
  - convert to Option(cur_box)
    - not sure possible with AHK - it's a nullable pointer?ok
    - how to call dealloc from AHK to avoid memory leak? is it needed?
    - instead of returning default handle error?

# ✓ Done others:
  + test release workflow, build seems to work fine
  + add ahk examples, lib and actual test
  + add readme
  + compare shadow size for 200% 4 size va 100% 1 accessibility , is it truly flat 9 7?
  + ? add cli args to enable printing custom rows
  + export in c func for us ein AHK: 
    + one without a string one with a debug string: use cli for debuggin instead
  + also get screen position and calculate box screen coordinates instead of just the box, offset by hotspot to get top-left corner
  + add vec with capacit? or the macro covers it already (yes)?
  + test `24b ColorMasked blau I-beam.txt` compare to other files and test on my test 24 and 32b cursors
  + bug: measure box in measure.rs is wrong, likely incorrect logic vs drawing pixels
  check bug when pixel data that "shouldn't" exist for non-masked values twne `pxA=true`, so it's either transparent or inverted, but since 32b doesn't do inversion (non-masked color with α) it should have no values there except for 0,0,0,0?

# ✓ Done cursorinfo measure:
  + fix hotspot position on Acc scaling
  + ✗ can't try to do the more complicated route `https://learn.microsoft.com/en-us/windows/win32/api/wingdi/ns-wingdi-bitmapinfoheader` and see if it can detect 1px 24𝑏 vs 1px 32𝑏 for an inverted `255,255,255,0` pixel, which is recognized as the same ColoredMasked by DXGI, but in cursor authoring software is 
    + otherwise use the rule that if any α channel in a bitmap I get via old API is > 0, then it's Color, otherwise it's ColorMasked

# ✓ Done DXGI (or found impossible):
  + add is_hidden and also get `DXGI_OUTDUPL_POINTER_POSITION` to not require an extra call for the position
  + DXGI panics for black acc1 cursor which is black and white
  - ✗ remove screen capture, only capture the pointer
    - might be impossible since getting pointer shape "IDXGIOutputDuplication::GetFramePointerShape method (dxgi1_2.h) errors with DXGI_ERROR_INVALID_CALL if the application called GetFramePointerShape without owning the desktop image."
    - ✗not? maybe we can capture a smaller texture around the pointer only instead of the full screen?
      - via iteration ? how to detect which monitor has the pointer?
  - ✗ no screen captured on output duplication creation, need to call `AcquireNextFrame` (? capture the pointer on capturing the first screen without having to capture it again as currently the crate seems to be targeted for dynamic acquisition of frames)

- use another non desktop-duplication API, which is an overkill for this purpose since we don't need to capture the whole screen on the GPU
