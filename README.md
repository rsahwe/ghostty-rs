# ghostty-rs

Rust bindings for libghostty.

## Notes

libghostty is currently not stable, so breaking changes may occur frequently until the 1.0.0 release.

libghostty uses zig and therefore requires the zig compiler.

I could not get static compilation to work so the dynamic library has to be included in the final result.

There are three release modes, fast, small and safe. safe is enabled by default but can be overriden without disabling the default features.

## Completion

- [x] Startup and build info
- [ ] Config
- [ ] App
- [ ] Surface
- [ ] Inspector
