//! An graphics-backend independant egui backend for sdl2
pub mod conversions;
pub mod platform;

pub use crate::conversions::*;
pub use crate::platform::*;

/// SDL2 is re-exported to enable easier version sync for users
pub use sdl2;
