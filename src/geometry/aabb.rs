use crate::math::{Ray, Vec3};

const PARALLEL_EPSILON: f32 = 1.0e-8;

/// Caja alineada con los ejes; también es el volumen base de cada voxel.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Aabb {
    pub min: Vec3,
    pub max: Vec3,
}

/// Resultado geométrico de atravesar una caja.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AabbHit {
    pub distance: f32,
    pub normal: Vec3,
}

impl Aabb {
    #[must_use]
    pub fn new(a: Vec3, b: Vec3) -> Self {
        Self {
            min: Vec3::new(a.x.min(b.x), a.y.min(b.y), a.z.min(b.z)),
            max: Vec3::new(a.x.max(b.x), a.y.max(b.y), a.z.max(b.z)),
        }
    }

    #[must_use]
    pub fn from_center_size(center: Vec3, size: Vec3) -> Self {
        let half_size = size.map(|component| component.abs()) * 0.5;
        Self::new(center - half_size, center + half_size)
    }

    #[must_use]
    pub fn center(&self) -> Vec3 {
        (self.min + self.max) * 0.5
    }

    #[must_use]
    pub fn extent(&self) -> Vec3 {
        self.max - self.min
    }

    #[must_use]
    pub fn union(&self, other: &Self) -> Self {
        Self {
            min: Vec3::new(
                self.min.x.min(other.min.x),
                self.min.y.min(other.min.y),
                self.min.z.min(other.min.z),
            ),
            max: Vec3::new(
                self.max.x.max(other.max.x),
                self.max.y.max(other.max.y),
                self.max.z.max(other.max.z),
            ),
        }
    }

    /// Distancia de entrada al intervalo de la caja. A diferencia de
    /// `intersect`, si el rayo ya está dentro devuelve `min_distance`; esto
    /// es lo correcto para recorrer volúmenes contenedores de un BVH.
    #[must_use]
    pub fn entry_distance(&self, ray: &Ray, min_distance: f32, max_distance: f32) -> Option<f32> {
        let mut entry = min_distance;
        let mut exit = max_distance;

        for axis in 0..3 {
            let origin = ray.origin[axis];
            let direction = ray.direction[axis];
            if direction.abs() < PARALLEL_EPSILON {
                if origin < self.min[axis] || origin > self.max[axis] {
                    return None;
                }
                continue;
            }

            let mut near = (self.min[axis] - origin) / direction;
            let mut far = (self.max[axis] - origin) / direction;
            if near > far {
                std::mem::swap(&mut near, &mut far);
            }
            entry = entry.max(near);
            exit = exit.min(far);
            if entry > exit {
                return None;
            }
        }

        Some(entry)
    }

    /// Algoritmo de slabs. Devuelve la entrada o, si el origen está dentro,
    /// la salida de la caja.
    #[must_use]
    pub fn intersect(&self, ray: &Ray, min_distance: f32, max_distance: f32) -> Option<AabbHit> {
        let mut entry = f32::NEG_INFINITY;
        let mut exit = f32::INFINITY;
        let mut entry_normal = Vec3::zeros();
        let mut exit_normal = Vec3::zeros();

        for axis in 0..3 {
            let origin = ray.origin[axis];
            let direction = ray.direction[axis];

            if direction.abs() < PARALLEL_EPSILON {
                if origin < self.min[axis] || origin > self.max[axis] {
                    return None;
                }
                continue;
            }

            let mut near = (self.min[axis] - origin) / direction;
            let mut far = (self.max[axis] - origin) / direction;
            let mut near_normal = axis_normal(axis, -1.0);
            let mut far_normal = axis_normal(axis, 1.0);

            if near > far {
                std::mem::swap(&mut near, &mut far);
                std::mem::swap(&mut near_normal, &mut far_normal);
            }

            if near > entry {
                entry = near;
                entry_normal = near_normal;
            }
            if far < exit {
                exit = far;
                exit_normal = far_normal;
            }
            if entry > exit {
                return None;
            }
        }

        let (distance, normal) = if entry >= min_distance {
            (entry, entry_normal)
        } else {
            (exit, exit_normal)
        };

        (distance >= min_distance && distance <= max_distance)
            .then_some(AabbHit { distance, normal })
    }
}

fn axis_normal(axis: usize, sign: f32) -> Vec3 {
    let mut normal = Vec3::zeros();
    normal[axis] = sign;
    normal
}

#[cfg(test)]
mod tests {
    use super::*;

    fn unit_box() -> Aabb {
        Aabb::new(Vec3::new(-1.0, -1.0, -1.0), Vec3::new(1.0, 1.0, 1.0))
    }

    #[test]
    fn hits_front_face() {
        let ray =
            Ray::new(Vec3::new(0.0, 0.0, 4.0), Vec3::new(0.0, 0.0, -1.0)).expect("valid test ray");
        let hit = unit_box()
            .intersect(&ray, 0.001, f32::INFINITY)
            .expect("ray should hit");

        assert!((hit.distance - 3.0).abs() < 1.0e-6);
        assert_eq!(hit.normal, Vec3::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn ray_inside_uses_exit_face() {
        let ray = Ray::new(Vec3::zeros(), Vec3::new(1.0, 0.0, 0.0)).expect("valid test ray");
        let hit = unit_box()
            .intersect(&ray, 0.001, f32::INFINITY)
            .expect("ray should leave box");

        assert!((hit.distance - 1.0).abs() < 1.0e-6);
        assert_eq!(hit.normal, Vec3::new(1.0, 0.0, 0.0));
    }

    #[test]
    fn parallel_ray_outside_misses() {
        let ray =
            Ray::new(Vec3::new(2.0, 0.0, 4.0), Vec3::new(0.0, 0.0, -1.0)).expect("valid test ray");
        assert!(unit_box().intersect(&ray, 0.001, f32::INFINITY).is_none());
    }

    #[test]
    fn bvh_entry_for_inside_ray_is_minimum_distance() {
        let ray = Ray::new(Vec3::zeros(), Vec3::new(1.0, 0.0, 0.0)).expect("valid test ray");
        assert_eq!(unit_box().entry_distance(&ray, 0.001, 0.5), Some(0.001));
    }
}
