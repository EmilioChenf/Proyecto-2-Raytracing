//! Tipos matemáticos compartidos.

mod ray;

pub use nalgebra_glm::Vec3;
pub use ray::Ray;

#[must_use]
pub fn reflect(incident: Vec3, normal: Vec3) -> Vec3 {
    incident - normal * (2.0 * incident.dot(&normal))
}
