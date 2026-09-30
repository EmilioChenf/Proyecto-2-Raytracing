use crate::math::Vec3;

use super::Color;

/// Patrones procedurales: no requieren archivos externos y conservan el
/// aspecto cúbico de los materiales en todas las caras.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Texture {
    Solid,
    Checker {
        secondary: Color,
        scale: f32,
    },
    Speckled {
        secondary: Color,
        scale: f32,
        amount: f32,
    },
    Grain {
        secondary: Color,
        scale: f32,
    },
    Ripples {
        secondary: Color,
        scale: f32,
    },
}

impl Texture {
    #[must_use]
    pub fn sample(&self, albedo: Color, point: Vec3, uv: (f32, f32)) -> Color {
        match *self {
            Self::Solid => albedo,
            Self::Checker { secondary, scale } => {
                let u = (uv.0 * scale).floor() as i32;
                let v = (uv.1 * scale).floor() as i32;
                if (u + v).rem_euclid(2) == 0 {
                    albedo
                } else {
                    secondary
                }
            }
            Self::Speckled {
                secondary,
                scale,
                amount,
            } => {
                if cell_hash(point * scale) < amount.clamp(0.0, 1.0) {
                    secondary
                } else {
                    albedo
                }
            }
            Self::Grain { secondary, scale } => {
                let grain = (point.y * 0.75 + point.x * scale + (point.z * scale * 0.37).sin())
                    .sin()
                    .abs();
                albedo * grain + secondary * (1.0 - grain)
            }
            Self::Ripples { secondary, scale } => {
                let wave = ((point.x + point.z) * scale).sin() * 0.5 + 0.5;
                albedo * (0.65 + wave * 0.35) + secondary * (0.15 * (1.0 - wave))
            }
        }
    }
}

fn cell_hash(point: Vec3) -> f32 {
    let x = point.x.floor() as i32 as u32;
    let y = point.y.floor() as i32 as u32;
    let z = point.z.floor() as i32 as u32;
    let mut hash =
        x.wrapping_mul(0x8da6_b343) ^ y.wrapping_mul(0xd816_3841) ^ z.wrapping_mul(0xcb1a_b31f);
    hash ^= hash >> 13;
    hash = hash.wrapping_mul(0x85eb_ca6b);
    (hash & 0x00ff_ffff) as f32 / 0x00ff_ffff as f32
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::material::rgb;

    #[test]
    fn checker_alternates_cells() {
        let primary = rgb(255, 255, 255);
        let secondary = rgb(0, 0, 0);
        let texture = Texture::Checker {
            secondary,
            scale: 2.0,
        };

        assert_eq!(texture.sample(primary, Vec3::zeros(), (0.1, 0.1)), primary);
        assert_eq!(
            texture.sample(primary, Vec3::zeros(), (0.6, 0.1)),
            secondary
        );
    }
}
