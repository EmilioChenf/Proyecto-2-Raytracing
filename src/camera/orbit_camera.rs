use std::f32::consts::FRAC_PI_2;

use crate::math::{Ray, Vec3};

const PITCH_MARGIN: f32 = 0.05;

/// Cámara que mantiene su mirada en el centro del diorama.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct OrbitCamera {
    pub target: Vec3,
    pub distance: f32,
    pub yaw: f32,
    pub pitch: f32,
    pub vertical_fov: f32,
    pub min_distance: f32,
    pub max_distance: f32,
}

impl OrbitCamera {
    #[must_use]
    pub fn new(target: Vec3, distance: f32, yaw: f32, pitch: f32) -> Self {
        let mut camera = Self {
            target,
            distance,
            yaw,
            pitch,
            vertical_fov: 55.0_f32.to_radians(),
            min_distance: 3.0,
            max_distance: 80.0,
        };
        camera.clamp_parameters();
        camera
    }

    #[must_use]
    pub fn with_limits(mut self, min_distance: f32, max_distance: f32) -> Self {
        self.min_distance = min_distance.max(0.1);
        self.max_distance = max_distance.max(self.min_distance);
        self.clamp_parameters();
        self
    }

    #[must_use]
    pub fn position(&self) -> Vec3 {
        let horizontal = self.distance * self.pitch.cos();
        self.target
            + Vec3::new(
                horizontal * self.yaw.sin(),
                self.distance * self.pitch.sin(),
                horizontal * self.yaw.cos(),
            )
    }

    pub fn orbit(&mut self, delta_yaw: f32, delta_pitch: f32) {
        self.yaw = (self.yaw + delta_yaw).rem_euclid(std::f32::consts::TAU);
        self.pitch += delta_pitch;
        self.clamp_parameters();
    }

    /// Un delta positivo acerca la cámara.
    pub fn zoom(&mut self, delta: f32) {
        self.distance -= delta;
        self.clamp_parameters();
    }

    #[must_use]
    pub fn ray_for_pixel(&self, x: usize, y: usize, width: usize, height: usize) -> Ray {
        self.ray_from_screen(x as f32 + 0.5, y as f32 + 0.5, width, height)
    }

    /// Convierte coordenadas de ventana a un rayo en el mundo. Se utiliza
    /// tanto para renderizar como para seleccionar personajes.
    #[must_use]
    pub fn ray_from_screen(&self, x: f32, y: f32, width: usize, height: usize) -> Ray {
        let safe_width = width.max(1) as f32;
        let safe_height = height.max(1) as f32;
        let aspect = safe_width / safe_height;
        let perspective = (self.vertical_fov * 0.5).tan();

        let screen_x = (2.0 * x / safe_width - 1.0) * aspect * perspective;
        let screen_y = (1.0 - 2.0 * y / safe_height) * perspective;

        let origin = self.position();
        let forward = (self.target - origin).normalize();
        let world_up = Vec3::new(0.0, 1.0, 0.0);
        let right = forward.cross(&world_up).normalize();
        let up = right.cross(&forward).normalize();
        let direction = (forward + right * screen_x + up * screen_y).normalize();

        Ray { origin, direction }
    }

    fn clamp_parameters(&mut self) {
        self.distance = self.distance.clamp(self.min_distance, self.max_distance);
        self.pitch = self
            .pitch
            .clamp(-FRAC_PI_2 + PITCH_MARGIN, FRAC_PI_2 - PITCH_MARGIN);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn center_ray_points_at_target() {
        let camera = OrbitCamera::new(Vec3::zeros(), 10.0, 0.0, 0.0);
        let ray = camera.ray_from_screen(400.0, 300.0, 800, 600);
        let expected = (camera.target - camera.position()).normalize();

        assert!((ray.direction - expected).norm() < 1.0e-6);
    }

    #[test]
    fn zoom_and_pitch_respect_limits() {
        let mut camera = OrbitCamera::new(Vec3::zeros(), 10.0, 0.0, 0.0).with_limits(4.0, 20.0);

        camera.zoom(100.0);
        camera.orbit(0.0, 10.0);

        assert_eq!(camera.distance, 4.0);
        assert!(camera.pitch < FRAC_PI_2);
    }

    #[test]
    fn orbit_preserves_distance() {
        let mut camera = OrbitCamera::new(Vec3::new(2.0, 1.0, -4.0), 12.0, 0.2, 0.3);
        camera.orbit(1.1, -0.4);

        assert!(((camera.position() - camera.target).norm() - 12.0).abs() < 1.0e-5);
    }
}
