# winpath.rs
Rust library for canonicalizing Windows file paths

This library provides functions for canonicalizing Windows file paths.
They are implemented in pure Rust, and do not call `GetFullPathNameW` or any similar API function.
Furthermore, these functions do not behave identically to the Windows API functions, and allow for paths to be accessed
that are not normally accessible.
