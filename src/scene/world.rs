use crate::{
    geometry::{Cube, SurfaceHit},
    lighting::Light,
    material::{Color, Material},
    math::Ray,
};

use super::{bvh::Bvh, Skybox};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SceneHit {
    pub surface: SurfaceHit,
    pub cube_index: usize,
}

/// Datos renderizables de una escena. Los constructores voxel se agregarán
/// por separado para que el trazador no dependa de un mundo concreto.
#[derive(Debug, Clone)]
pub struct Scene {
    pub cubes: Vec<Cube>,
    pub materials: Vec<Material>,
    pub lights: Vec<Light>,
    pub ambient_color: Color,
    pub ambient_intensity: f32,
    pub skybox: Skybox,
    bvh: Option<Bvh>,
}

impl Scene {
    #[must_use]
    pub fn new(ambient_color: Color, ambient_intensity: f32) -> Self {
        Self {
            cubes: Vec::new(),
            materials: Vec::new(),
            lights: Vec::new(),
            ambient_color,
            ambient_intensity: ambient_intensity.max(0.0),
            skybox: Skybox::default(),
            bvh: None,
        }
    }

    #[must_use]
    pub fn with_skybox(mut self, skybox: Skybox) -> Self {
        self.skybox = skybox;
        self
    }

    pub fn add_material(&mut self, material: Material) -> usize {
        let index = self.materials.len();
        self.materials.push(material);
        index
    }

    pub fn add_cube(&mut self, cube: Cube) {
        self.cubes.push(cube);
        self.bvh = None;
    }

    pub fn rebuild_acceleration(&mut self) {
        self.bvh = Bvh::build(&self.cubes);
    }

    #[must_use]
    pub fn acceleration_node_count(&self) -> usize {
        self.bvh.as_ref().map_or(0, Bvh::node_count)
    }

    #[must_use]
    pub fn intersect(&self, ray: &Ray, min_distance: f32, max_distance: f32) -> Option<SceneHit> {
        if let Some(bvh) = &self.bvh {
            return bvh
                .intersect(&self.cubes, ray, min_distance, max_distance)
                .map(|(cube_index, surface)| SceneHit {
                    surface,
                    cube_index,
                });
        }

        let mut closest_distance = max_distance;
        let mut closest = None;

        for (cube_index, cube) in self.cubes.iter().enumerate() {
            if let Some(surface) = cube.intersect(ray, min_distance, closest_distance) {
                closest_distance = surface.distance;
                closest = Some(SceneHit {
                    surface,
                    cube_index,
                });
            }
        }

        closest
    }

    #[must_use]
    pub fn occluded(&self, ray: &Ray, max_distance: f32) -> bool {
        self.intersect(ray, 0.0, max_distance).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{material::rgb, math::Vec3};

    #[test]
    fn intersection_returns_closest_cube() {
        let mut scene = Scene::new(rgb(255, 255, 255), 0.1);
        scene.add_cube(Cube::new(Vec3::new(0.0, 0.0, -8.0), Vec3::repeat(1.0), 0));
        scene.add_cube(Cube::new(Vec3::new(0.0, 0.0, -3.0), Vec3::repeat(1.0), 0));
        let ray = Ray::new(Vec3::zeros(), Vec3::new(0.0, 0.0, -1.0)).expect("valid test ray");

        assert_eq!(
            scene
                .intersect(&ray, 0.001, f32::INFINITY)
                .expect("ray should hit")
                .cube_index,
            1
        );
    }

    #[test]
    fn bvh_matches_linear_intersection() {
        let mut scene = Scene::new(rgb(255, 255, 255), 0.1);
        for z in 0..12 {
            scene.add_cube(Cube::new(
                Vec3::new(0.0, 0.0, -(z as f32 * 2.0 + 2.0)),
                Vec3::repeat(1.0),
                0,
            ));
        }
        let ray = Ray::new(Vec3::zeros(), Vec3::new(0.0, 0.0, -1.0)).expect("valid test ray");
        let linear = scene
            .intersect(&ray, 0.001, f32::INFINITY)
            .expect("linear hit");
        scene.rebuild_acceleration();
        let accelerated = scene
            .intersect(&ray, 0.001, f32::INFINITY)
            .expect("BVH hit");

        assert!(scene.acceleration_node_count() > 1);
        assert_eq!(linear.cube_index, accelerated.cube_index);
        assert!((linear.surface.distance - accelerated.surface.distance).abs() < 1.0e-6);
    }
}
