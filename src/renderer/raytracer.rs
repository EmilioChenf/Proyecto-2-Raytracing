use crate::{
    camera::OrbitCamera,
    material::{pack_rgb, rgb, Color},
    math::{reflect, refract, schlick, Ray},
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
            exposure: 1.05,
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
            return scene.skybox.sample(ray.direction);
        };

        let local_color = self.shade(scene, ray, hit);
        let Some(material) = scene.materials.get(hit.surface.material_index) else {
            return local_color;
        };
        if depth >= self.max_depth || (material.reflectivity <= 0.0 && material.transparency <= 0.0)
        {
            return local_color;
        }

        let entering = ray.direction.dot(&hit.surface.normal) < 0.0;
        let oriented_normal = if entering {
            hit.surface.normal
        } else {
            -hit.surface.normal
        };
        let (index_from, index_to) = if entering {
            (1.0, material.refractive_index)
        } else {
            (material.refractive_index, 1.0)
        };
        let cosine = (-ray.direction).dot(&oriented_normal).clamp(0.0, 1.0);

        let reflection_direction = reflect(ray.direction, oriented_normal).normalize();
        let reflection_ray = Ray {
            origin: hit.surface.point + oriented_normal * self.shadow_bias,
            direction: reflection_direction,
        };
        let reflected = self.trace_recursive(scene, &reflection_ray, depth + 1);

        let refracted_direction = refract(ray.direction, oriented_normal, index_from, index_to);
        let fresnel = if refracted_direction.is_some() {
            schlick(cosine, index_from, index_to)
        } else {
            1.0
        };
        let available = 1.0 - material.reflectivity;
        let reflection_weight = material.reflectivity + available * material.transparency * fresnel;
        let refraction_weight = available * material.transparency * (1.0 - fresnel);
        let local_weight = available * (1.0 - material.transparency);

        let refracted = refracted_direction.map_or_else(Color::zeros, |direction| {
            let refraction_ray = Ray {
                origin: hit.surface.point - oriented_normal * self.shadow_bias,
                direction,
            };
            let transmitted = self.trace_recursive(scene, &refraction_ray, depth + 1);
            let tint = Color::repeat(0.75) + material.albedo * 0.25;
            transmitted.component_mul(&tint)
        });

        local_color * local_weight + reflected * reflection_weight + refracted * refraction_weight
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
        if width == 0 || height == 0 {
            return pixels;
        }

        let worker_count = std::thread::available_parallelism()
            .map_or(1, std::num::NonZeroUsize::get)
            .min(height);
        if worker_count == 1 || pixels.len() < 16_384 {
            self.render_rows(&mut pixels, scene, camera, width, height, 0);
            return pixels;
        }

        let rows_per_worker = height.div_ceil(worker_count);
        let pixels_per_worker = rows_per_worker * width;
        std::thread::scope(|scope| {
            for (worker, pixel_chunk) in pixels.chunks_mut(pixels_per_worker).enumerate() {
                let first_row = worker * rows_per_worker;
                scope.spawn(move || {
                    self.render_rows(pixel_chunk, scene, camera, width, height, first_row);
                });
            }
        });
        pixels
    }

    fn render_rows(
        &self,
        pixels: &mut [u32],
        scene: &Scene,
        camera: &OrbitCamera,
        width: usize,
        height: usize,
        first_row: usize,
    ) {
        for (local_y, row) in pixels.chunks_mut(width).enumerate() {
            let y = first_row + local_y;
            for (x, pixel) in row.iter_mut().enumerate() {
                let ray = camera.ray_for_pixel(x, y, width, height);
                *pixel = pack_rgb(self.trace(scene, &ray), self.exposure);
            }
        }
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

    #[test]
    fn parallel_render_matches_serial_rows() {
        let tracer = RayTracer::default();
        let scene = lit_scene(false);
        let camera = OrbitCamera::new(Vec3::zeros(), 8.0, 0.4, 0.2);
        let mut serial = vec![0; 128 * 128];
        tracer.render_rows(&mut serial, &scene, &camera, 128, 128, 0);

        assert_eq!(tracer.render(&scene, &camera, 128, 128), serial);
    }
}
