use super::Vec3;

/// Semirrecta con dirección normalizada.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    /// Crea un rayo o devuelve `None` si la dirección es demasiado pequeña.
    pub fn new(origin: Vec3, direction: Vec3) -> Option<Self> {
        let length_squared = direction.norm_squared();
        if length_squared <= f32::EPSILON {
            return None;
        }

        Some(Self {
            origin,
            direction: direction / length_squared.sqrt(),
        })
    }

    /// Evalúa `origen + t * dirección`.
    #[must_use]
    pub fn at(&self, distance: f32) -> Vec3 {
        self.origin + self.direction * distance
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalizes_direction() {
        let ray = Ray::new(Vec3::zeros(), Vec3::new(0.0, 0.0, -4.0));
        let ray = ray.expect("the test direction is valid");

        assert!((ray.direction.norm() - 1.0).abs() < 1.0e-6);
        assert_eq!(ray.at(2.0), Vec3::new(0.0, 0.0, -2.0));
    }

    #[test]
    fn rejects_zero_direction() {
        assert!(Ray::new(Vec3::zeros(), Vec3::zeros()).is_none());
    }
}
