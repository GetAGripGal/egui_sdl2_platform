[![Crates.io](https://img.shields.io/crates/v/egui_sdl2_platform.svg)](https://crates.io/crates/egui_sdl2_platform)
[![Documentation](https://docs.rs/egui_sdl2_platform/badge.svg)](https://docs.rs/sdl2_egui_platform)

# A Sdl2 + Egui Backend
An egui backend for sdl2 unbound to any renderer-backend.

You can include it like so:
```toml
[dependencies]
egui_sdl2_platform = "0.2.0"
```

[SDL2](https://github.com/Rust-SDL2/rust-sdl2) is re-exported with various feature flags that can be enabled with:

- sdl2_unsafe_textures
- sdl2_gfx
- sdl2_mixer
- sdl2_image
- sdl2_ttf
- sdl2_use-bindgen
- sdl2_use-pkgconfig
- sdl2_use-vcpkg
- sdl2_use-mac_framework
- sdl2_bundled
- sdl2_static-link

## Examples
I have included an example of how to use this backend together with wgpu using [egui_wgpu_backend](https://github.com/hasenbanck/egui_wgpu_backend).
It can be found [here](https://github.com/ComLarsic/sdl2_egui_platform/tree/main/examples/sdl2_plus_wgpu).

There is also an additional example using [egui_glow](https://github.com/emilk/egui/tree/master/crates/egui_glow) to enable openGL.

## Alternatives
If you are using sdl2 with opengl it is worth looking at [egui_sdl2_gl](https://github.com/ArjunNair/egui_sdl2_gl/) also.
