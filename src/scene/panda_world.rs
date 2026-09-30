use crate::{characters::create_panda, lighting::Light, material::rgb, math::Vec3};

use super::{install_palette, CharacterWorld, Scene, Skybox, VoxelBuilder};

#[must_use]
pub fn create_panda_world() -> CharacterWorld {
    let mut scene = Scene::new(rgb(190, 222, 174), 0.18).with_skybox(Skybox::bamboo());
    let palette = install_palette(&mut scene);
    scene.lights.push(Light::Directional {
        direction: Vec3::new(0.4, -1.0, 0.25),
        color: rgb(236, 244, 197),
        intensity: 1.5,
    });
    scene.lights.push(Light::Point {
        position: Vec3::new(-5.0, 4.0, -3.0),
        color: rgb(255, 181, 72),
        intensity: 4.5,
    });

    let mut world = VoxelBuilder::new(scene);
    world.add_box(
        Vec3::new(0.0, -0.5, 0.0),
        Vec3::new(18.0, 1.0, 14.0),
        palette.grass,
    );
    world.add_box(
        Vec3::new(0.0, -1.25, 0.0),
        Vec3::new(17.0, 0.6, 13.0),
        palette.dirt,
    );
    world.add_box(
        Vec3::new(0.0, -1.72, 0.0),
        Vec3::new(15.4, 0.35, 11.5),
        palette.stone,
    );

    // Arroyo que cruza el claro y puente de madera.
    world.add_box(
        Vec3::new(3.8, 0.08, 0.0),
        Vec3::new(4.2, 0.2, 12.0),
        palette.water,
    );
    for z in -3..=3 {
        world.add_box(
            Vec3::new(3.8, 0.55, z as f32 * 0.7),
            Vec3::new(5.0, 0.3, 0.55),
            palette.wood,
        );
    }
    for x in [1.5, 6.1] {
        world.add_box(
            Vec3::new(x, 1.05, 0.0),
            Vec3::new(0.18, 1.2, 5.0),
            palette.dark_wood,
        );
        for z in [-2.25, 2.25] {
            world.add_box(
                Vec3::new(x, 0.95, z),
                Vec3::new(0.34, 1.9, 0.34),
                palette.dark_wood,
            );
        }
    }

    for (x, z, height) in [
        (-7.2, -4.4, 5.4),
        (-6.5, 3.8, 4.6),
        (-4.4, 5.0, 5.1),
        (-3.2, -4.8, 4.3),
        (7.2, 4.5, 5.0),
        (7.1, -4.2, 4.7),
        (5.8, 5.1, 4.2),
        (0.3, 5.3, 5.5),
    ] {
        world.add_bamboo(Vec3::new(x, 0.0, z), height, &palette);
    }
    for (x, z, height) in [(-5.5, 1.0, 3.7), (5.7, -1.1, 3.5)] {
        world.add_bamboo(Vec3::new(x, 0.0, z), height, &palette);
    }
    for (x, z, height) in [(-7.4, 0.2, 4.6), (7.2, 0.7, 4.8)] {
        world.add_tree(Vec3::new(x, 0.0, z), height, &palette);
    }
    for (x, z, scale) in [(-5.2, -3.7, 0.9), (-5.8, 2.0, 1.2), (6.3, -2.5, 0.75)] {
        world.add_rock(Vec3::new(x, 0.0, z), scale, palette.stone);
    }
    world.add_rock(Vec3::new(1.35, 0.0, -2.6), 0.75, palette.stone);
    world.add_rock(Vec3::new(6.25, 0.0, 2.8), 0.68, palette.stone);
    world.add_lantern(Vec3::new(-4.8, 0.0, -1.8), &palette);
    world.add_lantern(Vec3::new(6.8, 0.0, 2.5), &palette);

    // Camino de losas frente a Panda.
    for step in -5_i32..=5 {
        let x = step as f32 * 0.78 - 2.8;
        let z = -3.7 + (step.rem_euclid(2) as f32) * 0.18;
        world.add_box(
            Vec3::new(x, 0.08, z),
            Vec3::new(0.62, 0.16, 0.72),
            palette.stone,
        );
    }
    world.add_box(
        Vec3::new(-1.2, 0.07, -0.4),
        Vec3::new(3.8, 0.14, 2.9),
        palette.stone,
    );
    for (x, z) in [(-6.2, -2.4), (-3.9, 3.8), (0.7, 3.9), (7.2, -2.5)] {
        world.add_box(
            Vec3::new(x, 0.3, z),
            Vec3::new(0.5, 0.6, 0.5),
            palette.leaves,
        );
    }

    let model = create_panda(Vec3::new(-1.2, 0.1, -0.4), 0.9, &palette);
    CharacterWorld::attach(world.finish(), model)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::characters::CharacterKind;

    #[test]
    fn panda_world_has_bamboo_bridge_and_water() {
        let world = create_panda_world();
        assert_eq!(world.character, CharacterKind::Panda);
        assert!(world.scene.cubes.len() > 90);
        assert!(world
            .scene
            .materials
            .iter()
            .any(|material| material.name == "Bamboo"));
        assert!(world
            .scene
            .materials
            .iter()
            .any(|material| material.name == "Water" && material.transparency > 0.5));
    }
}
