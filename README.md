[![Crates.io](https://img.shields.io/crates/v/egui_sdl2_platform.svg)](https://crates.io/crates/sdl2_egui_platform)
[![Documentation](https://docs.rs/egui_sdl2_platform/badge.svg)](https://docs.rs/sdl2_egui_platform)

# A Sdl2 + Egui Backend
An egui backend for sdl2 unbound to any renderer-backend.

You can include it like so:
```toml
[dependencies]
egui_sdl2_platform = "0.1.0"
sdl2 = "0.35"
```

## Examples
I have included an example of how to use this backend together with wgpu using [egui_wgpu_backend](https://github.com/hasenbanck/egui_wgpu_backend).
It can be found [here](https://github.com/ComLarsic/sdl2_egui_platform/tree/main/examples/sdl2_plus_wgpu).

## Alternatives
If you are using sdl2 with opengl I would recommend [egui_sdl2_gl](https://github.com/ArjunNair/egui_sdl2_gl/) which has a backend for opengl included.
