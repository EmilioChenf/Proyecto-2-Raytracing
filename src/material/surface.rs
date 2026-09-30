use crate::math::Vec3;

use super::{Color, Texture};

/// Propiedades visuales y ópticas de una superficie.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Material {
    pub name: &'static str,
    pub albedo: Color,
    pub texture: Texture,
    pub specular: f32,
    pub shininess: f32,
    pub transparency: f32,
    pub reflectivity: f32,
    pub refractive_index: f32,
    pub emission: Color,
}

impl Material {
    #[must_use]
    pub fn new(name: &'static str, albedo: Color, texture: Texture) -> Self {
        Self {
            name,
            albedo,
            texture,
            specular: 0.15,
            shininess: 24.0,
            transparency: 0.0,
            reflectivity: 0.0,
            refractive_index: 1.0,
            emission: Color::zeros(),
        }
    }

    #[must_use]
    pub fn with_surface(mut self, specular: f32, shininess: f32, reflectivity: f32) -> Self {
        self.specular = specular.clamp(0.0, 1.0);
        self.shininess = shininess.max(1.0);
        self.reflectivity = reflectivity.clamp(0.0, 1.0);
        self
    }

    #[must_use]
    pub fn with_transmission(mut self, transparency: f32, refractive_index: f32) -> Self {
        self.transparency = transparency.clamp(0.0, 1.0);
        self.refractive_index = refractive_index.max(1.0);
        self
    }

    #[must_use]
    pub fn with_emission(mut self, emission: Color) -> Self {
        self.emission = emission.map(|channel| channel.max(0.0));
        self
    }

    #[must_use]
    pub fn sample(&self, point: Vec3, uv: (f32, f32)) -> Color {
        self.texture.sample(self.albedo, point, uv)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::material::rgb;

    #[test]
    fn optical_properties_are_kept_physical() {
        let material = Material::new("test", rgb(10, 20, 30), Texture::Solid)
            .with_surface(2.0, -5.0, -1.0)
            .with_transmission(3.0, 0.2);

        assert_eq!(material.specular, 1.0);
        assert_eq!(material.shininess, 1.0);
        assert_eq!(material.reflectivity, 0.0);
        assert_eq!(material.transparency, 1.0);
        assert_eq!(material.refractive_index, 1.0);
    }
}
