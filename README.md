# ghostty-rs

Rust bindings for libghostty.

## Notes

libghostty is currently not stable, so breaking changes may occur frequently until the 1.0.0 release.

libghostty uses zig and therefore requires the zig compiler.

I could not get static compilation to work so the dynamic library has to be included in the final result.

## Completion

- [x] Startup and build info
- [ ] Config
  - [x] Creation
  - [x] Loading
  - [x] Fields also used by swift frontend
  - [ ] Other potential fields
  - [x] Diagnostics
  - [ ] Input triggers
- [ ] App
- [ ] Surface
- [ ] Inspector
