use crate::{material::Color, math::Vec3};

/// Fuente puntual o direccional.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Light {
    Point {
        position: Vec3,
        color: Color,
        intensity: f32,
    },
    Directional {
        /// Dirección en la que viaja la luz.
        direction: Vec3,
        color: Color,
        intensity: f32,
    },
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LightSample {
    pub direction: Vec3,
    pub distance: f32,
    pub radiance: Color,
}

impl Light {
    #[must_use]
    pub fn sample(&self, point: Vec3) -> LightSample {
        match *self {
            Self::Point {
                position,
                color,
                intensity,
            } => {
                let offset = position - point;
                let distance = offset.norm().max(1.0e-4);
                let attenuation = 1.0 / (1.0 + 0.025 * distance * distance);
                LightSample {
                    direction: offset / distance,
                    distance,
                    radiance: color * (intensity.max(0.0) * attenuation),
                }
            }
            Self::Directional {
                direction,
                color,
                intensity,
            } => LightSample {
                direction: -direction.normalize(),
                distance: f32::INFINITY,
                radiance: color * intensity.max(0.0),
            },
        }
    }
}
