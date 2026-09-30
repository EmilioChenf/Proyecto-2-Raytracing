use crate::math::Vec3;

/// Color lineal RGB, con componentes normalmente dentro de `0.0..=1.0`.
pub type Color = Vec3;

#[must_use]
pub fn rgb(red: u8, green: u8, blue: u8) -> Color {
    Color::new(
        f32::from(red) / 255.0,
        f32::from(green) / 255.0,
        f32::from(blue) / 255.0,
    )
}

/// Aplica exposición, tonemapping de Reinhard y gamma para el framebuffer.
#[must_use]
pub fn pack_rgb(color: Color, exposure: f32) -> u32 {
    let mapped = color.map(|channel| {
        let exposed = (channel.max(0.0) * exposure).min(1.0e6);
        let reinhard = exposed / (1.0 + exposed);
        reinhard.powf(1.0 / 2.2)
    });

    let red = (mapped.x.clamp(0.0, 1.0) * 255.0).round() as u32;
    let green = (mapped.y.clamp(0.0, 1.0) * 255.0).round() as u32;
    let blue = (mapped.z.clamp(0.0, 1.0) * 255.0).round() as u32;

    (red << 16) | (green << 8) | blue
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn packs_channels_in_rgb_order() {
        let red = pack_rgb(rgb(255, 0, 0), 1.0);
        let green = pack_rgb(rgb(0, 255, 0), 1.0);
        let blue = pack_rgb(rgb(0, 0, 255), 1.0);

        assert_ne!(red & 0xFF_0000, 0);
        assert_eq!(red & 0x00_FFFF, 0);
        assert_ne!(green & 0x00_FF00, 0);
        assert_eq!(green & 0xFF_00FF, 0);
        assert_ne!(blue & 0x00_00FF, 0);
        assert_eq!(blue & 0xFF_FF00, 0);
    }
}
