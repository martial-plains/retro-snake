use bevy::color::{Color, Srgba};

pub const GREEN: Color = srgb_u8(173, 204, 96);
pub const DARK_GREEN: Color = srgb_u8(43, 51, 24);

pub const fn srgb_u8(red: u8, green: u8, blue: u8) -> Color {
    Color::Srgba(Srgba {
        red: red as f32 / 255.0,
        green: green as f32 / 255.0,
        blue: blue as f32 / 255.0,
        alpha: 1.0,
    })
}
