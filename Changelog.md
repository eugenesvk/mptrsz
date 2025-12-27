# Changelog
All notable changes to this project will be documented in this file

[unreleased]: https://github.com/eugenesvk/mptrsz/compare/0.0.0...HEAD
## [Unreleased]
<!-- - ✨ __Added__ -->
  <!-- + new features -->
<!-- - Δ __Changed__ -->
  <!-- + changes in existing functionality -->
<!-- - 🐞 __Fixed__ -->
  <!-- + bug fixes -->
<!-- - 💩 __Deprecated__ -->
  <!-- + soon-to-be removed features -->
<!-- - 🗑️ __Removed__ -->
  <!-- + now removed features -->
<!-- - 🔒 __Security__ -->
  <!-- + vulnerabilities -->
- ✨ __Added__
  + `dll` with FFI functions `get_mcursor_sz_ci` `get_mcursor_sz_dx` to get the cursor size using either CursorInfo or DX Duplication (screenshot) API
  + Test executable with arguments for:
    + Print Binary mask (0¦1) or Color/color mask (255 0 0 255) values for given rows
    + Print masks for CursorInfo API
    + Print masks for DX Dupplication (screnshot) API
    + Report values in screen coordinates
    + Add an ≈approximation of shadow size (if it's enabled)

[0.0.0]: https://github.com/eugenesvk/mptrsz/releases/tag/0.0.0
## [0.0.0]
