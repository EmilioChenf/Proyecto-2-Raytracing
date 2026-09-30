use crate::{
    material::{rgb, Color},
    math::Vec3,
};

/// Cielo procedural con gradiente hemisférico y disco solar.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Skybox {
    pub zenith: Color,
    pub horizon: Color,
    pub ground: Color,
    pub sun_direction: Vec3,
    pub sun_color: Color,
    pub sun_strength: f32,
    pub cloud_strength: f32,
}

impl Skybox {
    #[must_use]
    pub fn main_world() -> Self {
        Self {
            zenith: rgb(38, 91, 176),
            horizon: rgb(255, 174, 137),
            ground: rgb(25, 27, 52),
            sun_direction: Vec3::new(-0.7, 0.65, 0.3).normalize(),
            sun_color: rgb(255, 226, 174),
            sun_strength: 2.2,
            cloud_strength: 0.42,
        }
    }

    #[must_use]
    pub fn polar() -> Self {
        Self {
            zenith: rgb(60, 127, 185),
            horizon: rgb(196, 231, 242),
            ground: rgb(61, 95, 121),
            sun_direction: Vec3::new(-0.5, 0.8, 0.2).normalize(),
            sun_color: rgb(218, 245, 255),
            sun_strength: 1.8,
            cloud_strength: 0.5,
        }
    }

    #[must_use]
    pub fn forest() -> Self {
        Self {
            zenith: rgb(66, 135, 205),
            horizon: rgb(255, 202, 137),
            ground: rgb(41, 62, 39),
            sun_direction: Vec3::new(-0.8, 0.55, 0.25).normalize(),
            sun_color: rgb(255, 190, 105),
            sun_strength: 2.6,
            cloud_strength: 0.24,
        }
    }

    #[must_use]
    pub fn bamboo() -> Self {
        Self {
            zenith: rgb(109, 171, 199),
            horizon: rgb(220, 235, 185),
            ground: rgb(37, 68, 42),
            sun_direction: Vec3::new(-0.4, 0.75, -0.3).normalize(),
            sun_color: rgb(242, 235, 171),
            sun_strength: 2.0,
            cloud_strength: 0.3,
        }
    }

    #[must_use]
    pub fn sample(&self, direction: Vec3) -> Color {
        let normalized = direction.normalize();
        let height = normalized.y.clamp(-1.0, 1.0);
        let base = if height >= 0.0 {
            let blend = height.sqrt();
            self.horizon * (1.0 - blend) + self.zenith * blend
        } else {
            let blend = (-height).sqrt();
            self.horizon * (1.0 - blend) + self.ground * blend
        };
        let cloud_noise = ((normalized.x * 8.0 + normalized.z * 3.0).sin()
            + (normalized.x * 17.0 - normalized.z * 11.0).sin() * 0.5)
            * 0.5
            + 0.5;
        let cloud = ((cloud_noise - 0.48) * 3.2).clamp(0.0, 1.0)
            * (1.0 - height.abs()).powi(2)
            * self.cloud_strength;
        let cloud_color = rgb(255, 244, 232);
        let base = base * (1.0 - cloud) + cloud_color * cloud;
        let sun = normalized.dot(&self.sun_direction).max(0.0).powf(384.0) * self.sun_strength;
        base + self.sun_color * sun
    }
}

impl Default for Skybox {
    fn default() -> Self {
        Self::main_world()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn samples_different_hemispheres() {
        let sky = Skybox::main_world();
        let upper = sky.sample(Vec3::new(0.0, 1.0, 0.0));
        let lower = sky.sample(Vec3::new(0.0, -1.0, 0.0));
        assert_ne!(upper, lower);
    }
}
