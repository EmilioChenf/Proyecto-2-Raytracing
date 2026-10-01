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
            zenith: rgb(24, 72, 172),
            horizon: rgb(104, 174, 224),
            ground: rgb(18, 31, 76),
            sun_direction: Vec3::new(-0.7, 0.65, 0.3).normalize(),
            sun_color: rgb(255, 226, 174),
            sun_strength: 2.2,
            cloud_strength: 0.55,
        }
    }

    #[must_use]
    pub fn polar() -> Self {
        Self {
            zenith: rgb(31, 82, 166),
            horizon: rgb(126, 204, 234),
            ground: rgb(25, 48, 82),
            sun_direction: Vec3::new(-0.5, 0.8, 0.2).normalize(),
            sun_color: rgb(218, 245, 255),
            sun_strength: 1.8,
            cloud_strength: 0.5,
        }
    }

    #[must_use]
    pub fn forest() -> Self {
        Self {
            zenith: rgb(31, 79, 158),
            horizon: rgb(238, 132, 64),
            ground: rgb(24, 43, 24),
            sun_direction: Vec3::new(-0.8, 0.55, 0.25).normalize(),
            sun_color: rgb(255, 190, 105),
            sun_strength: 2.6,
            cloud_strength: 0.24,
        }
    }

    #[must_use]
    pub fn bamboo() -> Self {
        Self {
            zenith: rgb(43, 115, 179),
            horizon: rgb(161, 210, 114),
            ground: rgb(20, 51, 28),
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
            let blend = (height * 2.0).min(1.0).sqrt();
            self.horizon * (1.0 - blend) + self.zenith * blend
        } else {
            let blend = (-height).sqrt();
            self.horizon * (1.0 - blend) + self.ground * blend
        };
        let cloud = if height > 0.0 && self.cloud_strength > 0.0 {
            let middle_sky = (height * (1.0 - height)).max(0.0).sqrt() * 2.0;
            ((cloud_noise(normalized) - 0.28) * 1.8).clamp(0.0, 1.0)
                * middle_sky
                * self.cloud_strength
        } else {
            0.0
        };
        let cloud_color = rgb(242, 244, 238);
        let base = base * (1.0 - cloud) + cloud_color * cloud;
        let sun_alignment = normalized.dot(&self.sun_direction).max(0.0);
        let sun = sun_alignment.powf(384.0) * self.sun_strength;
        let sun_glow = sun_alignment.powf(10.0) * self.sun_strength * 0.16;
        base + self.sun_color * (sun + sun_glow)
    }
}

fn cloud_noise(direction: Vec3) -> f32 {
    let x = direction.x * 5.0 + direction.y * 4.5;
    let z = direction.z * 5.0 - direction.y * 3.5;
    let floor_x = x.floor();
    let floor_z = z.floor();
    let cell_x = floor_x as i32;
    let cell_z = floor_z as i32;
    let fraction_x = x - floor_x;
    let fraction_z = z - floor_z;
    let blend_x = fraction_x.powi(2) * (3.0 - 2.0 * fraction_x);
    let blend_z = fraction_z.powi(2) * (3.0 - 2.0 * fraction_z);

    let bottom = hash(cell_x, cell_z) * (1.0 - blend_x) + hash(cell_x + 1, cell_z) * blend_x;
    let top = hash(cell_x, cell_z + 1) * (1.0 - blend_x) + hash(cell_x + 1, cell_z + 1) * blend_x;
    bottom * (1.0 - blend_z) + top * blend_z
}

fn hash(x: i32, z: i32) -> f32 {
    let mut value = (x as u32)
        .wrapping_mul(374_761_393)
        .wrapping_add((z as u32).wrapping_mul(668_265_263));
    value = (value ^ (value >> 13)).wrapping_mul(1_274_126_177);
    ((value ^ (value >> 16)) & 0xffff) as f32 / 65_535.0
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

    #[test]
    fn sun_glow_depends_on_ray_direction() {
        let sky = Skybox::main_world();
        let toward_sun = sky.sample(sky.sun_direction);
        let away_from_sun = sky.sample(-sky.sun_direction);

        assert!(toward_sun.norm() > away_from_sun.norm());
    }
}
