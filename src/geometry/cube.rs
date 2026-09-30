use crate::math::{Ray, Vec3};

use super::Aabb;

/// Voxel rectangular con una referencia al material de la escena.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cube {
    pub bounds: Aabb,
    pub material_index: usize,
}

/// Información necesaria para sombrear una superficie.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SurfaceHit {
    pub distance: f32,
    pub point: Vec3,
    pub normal: Vec3,
    pub uv: (f32, f32),
    pub material_index: usize,
}

impl Cube {
    #[must_use]
    pub fn new(center: Vec3, size: Vec3, material_index: usize) -> Self {
        Self {
            bounds: Aabb::from_center_size(center, size),
            material_index,
        }
    }

    #[must_use]
    pub fn intersect(&self, ray: &Ray, min_distance: f32, max_distance: f32) -> Option<SurfaceHit> {
        let hit = self.bounds.intersect(ray, min_distance, max_distance)?;
        let point = ray.at(hit.distance);

        Some(SurfaceHit {
            distance: hit.distance,
            point,
            normal: hit.normal,
            uv: face_uv(point, hit.normal),
            material_index: self.material_index,
        })
    }
}

fn face_uv(point: Vec3, normal: Vec3) -> (f32, f32) {
    let (u, v) = if normal.x.abs() > 0.5 {
        (point.z, point.y)
    } else if normal.y.abs() > 0.5 {
        (point.x, point.z)
    } else {
        (point.x, point.y)
    };

    (u.rem_euclid(1.0), v.rem_euclid(1.0))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cube_preserves_material_and_point() {
        let cube = Cube::new(Vec3::zeros(), Vec3::new(2.0, 2.0, 2.0), 7);
        let ray =
            Ray::new(Vec3::new(0.25, 0.5, 3.0), Vec3::new(0.0, 0.0, -1.0)).expect("valid test ray");
        let hit = cube
            .intersect(&ray, 0.001, f32::INFINITY)
            .expect("ray should hit cube");

        assert_eq!(hit.material_index, 7);
        assert_eq!(hit.point, Vec3::new(0.25, 0.5, 1.0));
        assert_eq!(hit.uv, (0.25, 0.5));
    }
}
