use crate::{
    camera::OrbitCamera,
    material::{pack_rgb, rgb, Color},
    math::{reflect, Ray},
    scene::{Scene, SceneHit},
};

/// Trazador CPU. La recursión óptica se incorporará en las siguientes fases.
#[derive(Debug, Clone, Copy)]
pub struct RayTracer {
    pub shadow_bias: f32,
    pub exposure: f32,
    pub max_depth: u8,
}

impl Default for RayTracer {
    fn default() -> Self {
        Self {
            shadow_bias: 1.0e-3,
            exposure: 1.15,
            max_depth: 3,
        }
    }
}

impl RayTracer {
    #[must_use]
    pub fn trace(&self, scene: &Scene, ray: &Ray) -> Color {
        self.trace_recursive(scene, ray, 0)
    }

    fn trace_recursive(&self, scene: &Scene, ray: &Ray, depth: u8) -> Color {
        let Some(hit) = scene.intersect(ray, self.shadow_bias, f32::INFINITY) else {
            return background(ray);
        };

        let local_color = self.shade(scene, ray, hit);
        let Some(material) = scene.materials.get(hit.surface.material_index) else {
            return local_color;
        };
        if depth >= self.max_depth || material.reflectivity <= 0.0 {
            return local_color;
        }

        let reflection_direction = reflect(ray.direction, hit.surface.normal).normalize();
        let bias_normal = if reflection_direction.dot(&hit.surface.normal) >= 0.0 {
            hit.surface.normal
        } else {
            -hit.surface.normal
        };
        let reflection_ray = Ray {
            origin: hit.surface.point + bias_normal * self.shadow_bias,
            direction: reflection_direction,
        };
        let reflected = self.trace_recursive(scene, &reflection_ray, depth + 1);

        local_color * (1.0 - material.reflectivity) + reflected * material.reflectivity
    }

    #[must_use]
    pub fn render(
        &self,
        scene: &Scene,
        camera: &OrbitCamera,
        width: usize,
        height: usize,
    ) -> Vec<u32> {
        let mut pixels = vec![0; width.saturating_mul(height)];
        for y in 0..height {
            for x in 0..width {
                let ray = camera.ray_for_pixel(x, y, width, height);
                pixels[y * width + x] = pack_rgb(self.trace(scene, &ray), self.exposure);
            }
        }
        pixels
    }

    fn shade(&self, scene: &Scene, ray: &Ray, hit: SceneHit) -> Color {
        let Some(material) = scene.materials.get(hit.surface.material_index) else {
            return rgb(255, 0, 255);
        };
        let albedo = material.sample(hit.surface.point, hit.surface.uv);
        let ambient = albedo.component_mul(&scene.ambient_color) * scene.ambient_intensity;
        let mut color = material.emission + ambient;
        let view_direction = -ray.direction;

        for light in &scene.lights {
            let sample = light.sample(hit.surface.point);
            let normal_dot_light = hit.surface.normal.dot(&sample.direction).max(0.0);
            if normal_dot_light <= 0.0 {
                continue;
            }

            let shadow_origin = hit.surface.point + hit.surface.normal * self.shadow_bias;
            let shadow_ray = Ray {
                origin: shadow_origin,
                direction: sample.direction,
            };
            if scene.occluded(&shadow_ray, sample.distance - self.shadow_bias) {
                continue;
            }

            let diffuse = albedo.component_mul(&sample.radiance) * normal_dot_light;
            let reflected_light = reflect(-sample.direction, hit.surface.normal);
            let specular_strength = view_direction
                .dot(&reflected_light)
                .max(0.0)
                .powf(material.shininess)
                * material.specular;
            let specular = sample.radiance * specular_strength;
            color += diffuse + specular;
        }

        color
    }
}

fn background(ray: &Ray) -> Color {
    let blend = (ray.direction.y * 0.5 + 0.5).clamp(0.0, 1.0);
    rgb(185, 211, 238) * blend + rgb(45, 71, 112) * (1.0 - blend)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        geometry::Cube,
        lighting::Light,
        material::{Material, Texture},
        math::Vec3,
    };

    fn lit_scene(with_blocker: bool) -> Scene {
        let mut scene = Scene::new(rgb(255, 255, 255), 0.05);
        let white = scene.add_material(Material::new("white", rgb(255, 255, 255), Texture::Solid));
        scene.add_cube(Cube::new(Vec3::zeros(), Vec3::new(2.0, 0.2, 2.0), white));
        if with_blocker {
            scene.add_cube(Cube::new(
                Vec3::new(0.0, 2.0, 0.0),
                Vec3::repeat(1.0),
                white,
            ));
        }
        scene.lights.push(Light::Point {
            position: Vec3::new(0.0, 4.0, 0.0),
            color: rgb(255, 255, 255),
            intensity: 2.0,
        });
        scene
    }

    #[test]
    fn blocker_reduces_direct_light() {
        let tracer = RayTracer::default();
        let ray =
            Ray::new(Vec3::new(0.0, 3.0, 3.0), Vec3::new(0.0, -1.0, -1.0)).expect("valid test ray");
        let lit = tracer.trace(&lit_scene(false), &ray);
        let shadowed = tracer.trace(&lit_scene(true), &ray);

        assert!(lit.norm() > shadowed.norm());
    }

    #[test]
    fn mirror_ray_reaches_emissive_cube() {
        let mut scene = Scene::new(rgb(0, 0, 0), 0.0);
        let mirror = scene.add_material(
            Material::new("mirror", rgb(0, 0, 0), Texture::Solid).with_surface(1.0, 128.0, 1.0),
        );
        let glow = scene.add_material(
            Material::new("glow", rgb(0, 0, 0), Texture::Solid)
                .with_emission(rgb(255, 20, 10) * 3.0),
        );
        scene.add_cube(Cube::new(Vec3::zeros(), Vec3::new(8.0, 0.1, 8.0), mirror));
        scene.add_cube(Cube::new(
            Vec3::new(0.0, 1.45, 2.35),
            Vec3::repeat(0.8),
            glow,
        ));

        let ray =
            Ray::new(Vec3::new(0.0, 1.0, 0.0), Vec3::new(0.0, -1.0, 1.0)).expect("valid test ray");
        let color = RayTracer::default().trace(&scene, &ray);

        assert!(color.x > color.z * 2.0);
    }
}
