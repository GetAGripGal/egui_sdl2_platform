use egui::Key;
use sdl2::keyboard::Keycode::*;

/// A trait that adds a method to convert to an egui key
pub trait ToEguiKey {
    /// Convert the struct to an egui key
    fn to_egui_key(&self) -> Option<egui::Key>;
}

impl ToEguiKey for sdl2::keyboard::Keycode {
    fn to_egui_key(&self) -> Option<egui::Key> {
        Some(match *self {
            Left => Key::ArrowLeft,
            Up => Key::ArrowUp,
            Right => Key::ArrowRight,
            Down => Key::ArrowDown,
            Escape => Key::Escape,
            Tab => Key::Tab,
            Backspace => Key::Backspace,
            Space => Key::Space,
            Return => Key::Enter,
            Insert => Key::Insert,
            Home => Key::Home,
            Delete => Key::Delete,
            End => Key::End,
            PageDown => Key::PageDown,
            PageUp => Key::PageUp,
            Kp0 | Num0 => Key::Num0,
            Kp1 | Num1 => Key::Num1,
            Kp2 | Num2 => Key::Num2,
            Kp3 | Num3 => Key::Num3,
            Kp4 | Num4 => Key::Num4,
            Kp5 | Num5 => Key::Num5,
            Kp6 | Num6 => Key::Num6,
            Kp7 | Num7 => Key::Num7,
            Kp8 | Num8 => Key::Num8,
            Kp9 | Num9 => Key::Num9,
            A => Key::A,
            B => Key::B,
            C => Key::C,
            D => Key::D,
            E => Key::E,
            F => Key::F,
            G => Key::G,
            H => Key::H,
            I => Key::I,
            J => Key::J,
            K => Key::K,
            L => Key::L,
            M => Key::M,
            N => Key::N,
            O => Key::O,
            P => Key::P,
            Q => Key::Q,
            R => Key::R,
            S => Key::S,
            T => Key::T,
            U => Key::U,
            V => Key::V,
            W => Key::W,
            X => Key::X,
            Y => Key::Y,
            Z => Key::Z,
            _ => {
                return None;
            }
        })
    }
}
