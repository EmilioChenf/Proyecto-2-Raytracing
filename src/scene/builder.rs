use crate::{geometry::Cube, math::Vec3};

use super::{Palette, Scene};

/// API compacta para componer dioramas sin listas planas de cubos.
pub struct VoxelBuilder {
    pub scene: Scene,
}

impl VoxelBuilder {
    #[must_use]
    pub fn new(scene: Scene) -> Self {
        Self { scene }
    }

    pub fn add_box(&mut self, center: Vec3, size: Vec3, material: usize) {
        self.scene.add_cube(Cube::new(center, size, material));
    }

    pub fn add_voxel(&mut self, position: Vec3, material: usize) {
        self.add_box(position, Vec3::repeat(1.0), material);
    }

    pub fn add_tree(&mut self, position: Vec3, height: f32, palette: &Palette) {
        self.add_box(
            position + Vec3::new(0.0, height * 0.5, 0.0),
            Vec3::new(0.8, height, 0.8),
            palette.wood,
        );
        for (offset, size) in [(0.55, 3.4), (1.65, 2.8), (2.65, 2.0)] {
            self.add_box(
                position + Vec3::new(0.0, height * 0.45 + offset, 0.0),
                Vec3::new(size, 1.4, size),
                palette.leaves,
            );
        }
    }

    pub fn add_pine(&mut self, position: Vec3, height: f32, palette: &Palette, snowy: bool) {
        self.add_box(
            position + Vec3::new(0.0, height * 0.35, 0.0),
            Vec3::new(0.55, height * 0.7, 0.55),
            palette.dark_wood,
        );
        for layer in 0..4 {
            let width = 3.4 - layer as f32 * 0.65;
            let y = height * 0.28 + layer as f32 * 1.05;
            self.add_box(
                position + Vec3::new(0.0, y, 0.0),
                Vec3::new(width, 0.75, width),
                palette.leaves,
            );
            if snowy {
                self.add_box(
                    position + Vec3::new(0.0, y + 0.43, 0.0),
                    Vec3::new(width * 0.82, 0.16, width * 0.82),
                    palette.snow,
                );
            }
        }
    }

    pub fn add_rock(&mut self, position: Vec3, scale: f32, material: usize) {
        self.add_box(
            position + Vec3::new(0.0, scale * 0.3, 0.0),
            Vec3::new(scale, scale * 0.65, scale * 0.85),
            material,
        );
        self.add_box(
            position + Vec3::new(scale * 0.25, scale * 0.72, -scale * 0.1),
            Vec3::new(scale * 0.55, scale * 0.35, scale * 0.5),
            material,
        );
    }

    pub fn add_lantern(&mut self, position: Vec3, palette: &Palette) {
        self.add_box(
            position + Vec3::new(0.0, 1.5, 0.0),
            Vec3::new(0.16, 3.0, 0.16),
            palette.metal,
        );
        self.add_box(
            position + Vec3::new(0.38, 2.72, 0.0),
            Vec3::new(0.75, 0.14, 0.14),
            palette.metal,
        );
        self.add_box(
            position + Vec3::new(0.72, 2.42, 0.0),
            Vec3::new(0.42, 0.58, 0.42),
            palette.lantern,
        );
    }

    #[must_use]
    pub fn finish(self) -> Scene {
        self.scene
    }
}
