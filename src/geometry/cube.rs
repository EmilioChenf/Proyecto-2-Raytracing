use crate::math::{Ray, Vec3};

use super::Aabb;

/// Voxel rectangular con una referencia al material de la escena.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cube {
    pub bounds: Aabb,
    pub material_index: usize,
    center: Vec3,
    size: Vec3,
    rotation_y: f32,
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
        let size = size.map(|component| component.abs());
        Self {
            bounds: Aabb::from_center_size(center, size),
            material_index,
            center,
            size,
            rotation_y: 0.0,
        }
    }

    #[must_use]
    pub fn center(&self) -> Vec3 {
        self.center
    }

    #[must_use]
    pub fn size(&self) -> Vec3 {
        self.size
    }

    #[must_use]
    pub fn rotated_about_y(&self, pivot: Vec3, angle: f32) -> Self {
        let relative = self.center - pivot;
        let center = pivot + rotate_y(relative, angle);
        let rotation_y = self.rotation_y + angle;
        let cosine = rotation_y.cos().abs();
        let sine = rotation_y.sin().abs();
        let world_size = Vec3::new(
            self.size.x * cosine + self.size.z * sine,
            self.size.y,
            self.size.x * sine + self.size.z * cosine,
        );

        Self {
            bounds: Aabb::from_center_size(center, world_size),
            material_index: self.material_index,
            center,
            size: self.size,
            rotation_y,
        }
    }

    #[must_use]
    pub fn intersect(&self, ray: &Ray, min_distance: f32, max_distance: f32) -> Option<SurfaceHit> {
        // El AABB mundial descarta rayos antes de transformar al espacio local.
        self.bounds.intersect(ray, min_distance, max_distance)?;
        let local_ray = Ray {
            origin: rotate_y(ray.origin - self.center, -self.rotation_y),
            direction: rotate_y(ray.direction, -self.rotation_y),
        };
        let local_bounds = Aabb::from_center_size(Vec3::zeros(), self.size);
        let hit = local_bounds.intersect(&local_ray, min_distance, max_distance)?;
        let point = ray.at(hit.distance);
        let local_point = local_ray.at(hit.distance);
        let normal = rotate_y(hit.normal, self.rotation_y);

        Some(SurfaceHit {
            distance: hit.distance,
            point,
            normal,
            uv: face_uv(local_point, hit.normal),
            material_index: self.material_index,
        })
    }
}

fn rotate_y(vector: Vec3, angle: f32) -> Vec3 {
    let (sine, cosine) = angle.sin_cos();
    Vec3::new(
        vector.x * cosine + vector.z * sine,
        vector.y,
        -vector.x * sine + vector.z * cosine,
    )
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

    #[test]
    fn rotated_box_uses_oriented_intersection() {
        let cube = Cube::new(Vec3::zeros(), Vec3::new(4.0, 2.0, 1.0), 3)
            .rotated_about_y(Vec3::zeros(), std::f32::consts::FRAC_PI_2);
        let ray =
            Ray::new(Vec3::new(0.0, 0.0, 4.0), Vec3::new(0.0, 0.0, -1.0)).expect("valid test ray");
        let hit = cube
            .intersect(&ray, 0.001, f32::INFINITY)
            .expect("rotated box should be hit");

        assert!((hit.distance - 2.0).abs() < 1.0e-5);
        assert!((hit.normal - Vec3::new(0.0, 0.0, 1.0)).norm() < 1.0e-5);
    }
}
