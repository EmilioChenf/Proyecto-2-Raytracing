//! Tipos matemáticos compartidos.

mod optics;
mod ray;

pub use nalgebra_glm::Vec3;
pub use optics::{reflect, refract, schlick};
pub use ray::Ray;
