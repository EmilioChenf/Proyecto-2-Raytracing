use crate::{lighting::Light, material::rgb, math::Vec3};

use super::{install_palette, Scene, Skybox, VoxelBuilder};

/// Diorama inicial: isla flotante, cabaña, laguna y cascada.
#[must_use]
pub fn create_main_world() -> Scene {
    let mut scene = Scene::new(rgb(190, 205, 230), 0.2).with_skybox(Skybox::main_world());
    let palette = install_palette(&mut scene);
    scene.lights.push(Light::Directional {
        direction: Vec3::new(0.7, -1.0, -0.45),
        color: rgb(255, 236, 210),
        intensity: 1.3,
    });
    scene.lights.push(Light::Point {
        position: Vec3::new(-6.0, 8.0, 6.0),
        color: rgb(255, 174, 92),
        intensity: 4.0,
    });

    let mut world = VoxelBuilder::new(scene);

    // Estratos escalonados que dan silueta de roca suspendida.
    world.add_box(
        Vec3::new(0.0, -0.45, 0.0),
        Vec3::new(18.0, 1.1, 14.0),
        palette.grass,
    );
    world.add_box(
        Vec3::new(0.0, -1.45, 0.2),
        Vec3::new(16.5, 1.0, 12.5),
        palette.dirt,
    );
    world.add_box(
        Vec3::new(0.0, -2.7, 0.5),
        Vec3::new(13.8, 1.5, 10.0),
        palette.stone,
    );
    world.add_box(
        Vec3::new(0.2, -4.2, 0.8),
        Vec3::new(10.2, 1.6, 7.2),
        palette.stone,
    );
    world.add_box(
        Vec3::new(0.7, -5.8, 1.0),
        Vec3::new(6.0, 1.7, 4.4),
        palette.stone,
    );
    world.add_box(
        Vec3::new(1.2, -7.5, 1.2),
        Vec3::new(2.8, 1.8, 2.4),
        palette.stone,
    );

    add_house(&mut world, &palette);

    // Laguna superior y caída de agua por el borde derecho.
    world.add_box(
        Vec3::new(4.8, 0.18, 1.2),
        Vec3::new(6.3, 0.34, 5.5),
        palette.water,
    );
    world.add_box(
        Vec3::new(8.4, -2.8, 1.3),
        Vec3::new(0.42, 6.1, 3.2),
        palette.water,
    );
    world.add_box(
        Vec3::new(8.5, -6.2, 1.3),
        Vec3::new(1.4, 0.3, 4.1),
        palette.water,
    );

    for (x, z, height) in [
        (-7.1, -4.7, 4.4),
        (-5.8, 4.3, 5.2),
        (-2.8, 5.0, 4.2),
        (1.2, 5.1, 4.8),
        (7.1, -4.0, 4.5),
        (6.8, 4.6, 5.0),
    ] {
        world.add_pine(Vec3::new(x, 0.0, z), height, &palette, false);
    }
    for (x, z, scale) in [
        (-8.1, 1.3, 1.4),
        (-6.8, -1.8, 1.0),
        (7.8, -0.8, 1.2),
        (6.4, 5.5, 0.9),
        (-1.1, -5.7, 0.8),
    ] {
        world.add_rock(Vec3::new(x, 0.0, z), scale, palette.stone);
    }
    world.add_lantern(Vec3::new(-6.8, 0.0, -3.0), &palette);

    // Vegetación baja deliberadamente irregular.
    for (x, z) in [
        (-4.8, -4.5),
        (-3.9, -4.9),
        (2.4, -5.4),
        (3.2, -5.0),
        (6.6, -5.1),
        (-7.6, 3.7),
        (2.8, 5.7),
        (7.5, 3.1),
    ] {
        world.add_box(
            Vec3::new(x, 0.35, z),
            Vec3::new(0.55, 0.7, 0.55),
            palette.leaves,
        );
        world.add_box(
            Vec3::new(x + 0.38, 0.2, z + 0.22),
            Vec3::new(0.38, 0.4, 0.38),
            palette.grass,
        );
    }

    world.finish()
}

fn add_house(world: &mut VoxelBuilder, palette: &super::Palette) {
    let base = Vec3::new(-2.6, 0.0, 0.0);
    world.add_box(
        base + Vec3::new(0.0, 0.2, 0.0),
        Vec3::new(7.2, 0.4, 5.4),
        palette.stone,
    );
    world.add_box(
        base + Vec3::new(0.0, 2.1, 0.0),
        Vec3::new(6.6, 3.8, 4.8),
        palette.wood,
    );
    // Ventanas, puerta y marcos salientes.
    world.add_box(
        base + Vec3::new(-1.5, 1.9, -2.46),
        Vec3::new(1.35, 1.45, 0.16),
        palette.glass,
    );
    world.add_box(
        base + Vec3::new(1.55, 1.5, -2.48),
        Vec3::new(1.25, 2.55, 0.18),
        palette.dark_wood,
    );
    for x in [-2.3, -0.7] {
        world.add_box(
            base + Vec3::new(x, 1.9, -2.58),
            Vec3::new(0.1, 1.65, 0.12),
            palette.dark_wood,
        );
    }
    world.add_box(
        base + Vec3::new(-1.5, 1.9, -2.58),
        Vec3::new(1.7, 0.1, 0.12),
        palette.dark_wood,
    );
    // Techo a dos aguas sugerido por capas decrecientes.
    for layer in 0..4 {
        let width = 8.0 - layer as f32 * 0.65;
        let depth = 6.2 - layer as f32 * 1.05;
        world.add_box(
            base + Vec3::new(0.0, 4.2 + layer as f32 * 0.48, 0.0),
            Vec3::new(width, 0.5, depth),
            palette.dark_wood,
        );
    }
    world.add_box(
        base + Vec3::new(2.0, 5.25, 0.7),
        Vec3::new(0.7, 2.5, 0.7),
        palette.stone,
    );
    // Porche y baranda frente a la puerta.
    world.add_box(
        base + Vec3::new(1.8, 0.35, -3.2),
        Vec3::new(3.0, 0.35, 1.4),
        palette.wood,
    );
    for x in [0.5, 3.0] {
        world.add_box(
            base + Vec3::new(x, 1.05, -3.75),
            Vec3::new(0.18, 1.4, 0.18),
            palette.dark_wood,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn main_world_is_a_complex_lit_diorama() {
        let scene = create_main_world();
        assert!(scene.cubes.len() >= 80);
        assert!(scene.materials.len() >= 15);
        assert!(scene.lights.len() >= 2);
        assert!(scene
            .materials
            .iter()
            .any(|material| material.transparency > 0.5));
    }
}
