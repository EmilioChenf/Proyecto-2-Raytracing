use crate::{characters::create_pardo, lighting::Light, material::rgb, math::Vec3};

use super::{install_palette, CharacterWorld, Palette, Scene, Skybox, VoxelBuilder};

#[must_use]
pub fn create_pardo_world() -> CharacterWorld {
    let mut scene = Scene::new(rgb(224, 183, 133), 0.22).with_skybox(Skybox::forest());
    let palette = install_palette(&mut scene);
    scene.lights.push(Light::Directional {
        direction: Vec3::new(0.8, -1.0, -0.3),
        color: rgb(255, 214, 158),
        intensity: 1.45,
    });
    scene.lights.push(Light::Point {
        position: Vec3::new(-3.5, 4.5, -2.0),
        color: rgb(255, 139, 55),
        intensity: 6.0,
    });
    scene.lights.push(Light::Point {
        position: Vec3::new(5.5, 3.8, 1.5),
        color: rgb(255, 176, 80),
        intensity: 4.0,
    });

    let mut world = VoxelBuilder::new(scene);
    world.add_box(
        Vec3::new(0.0, -0.45, 0.0),
        Vec3::new(18.0, 0.9, 14.0),
        palette.grass,
    );
    world.add_box(
        Vec3::new(0.0, -1.2, 0.0),
        Vec3::new(17.0, 0.65, 13.0),
        palette.dirt,
    );
    world.add_box(
        Vec3::new(0.0, -1.75, 0.0),
        Vec3::new(15.5, 0.45, 11.5),
        palette.stone,
    );

    add_cabin(&mut world, &palette);

    for (x, z, height) in [
        (-7.2, -4.4, 5.3),
        (-7.0, 3.8, 4.7),
        (-3.4, 5.2, 5.5),
        (3.8, 5.4, 4.6),
        (7.0, 4.0, 5.2),
        (7.2, -4.7, 4.8),
    ] {
        world.add_tree(Vec3::new(x, 0.0, z), height, &palette);
    }
    for (x, z, scale) in [
        (-5.3, -4.8, 1.1),
        (-6.2, 0.2, 0.85),
        (2.4, -5.3, 0.75),
        (6.2, 2.0, 1.0),
    ] {
        world.add_rock(Vec3::new(x, 0.0, z), scale, palette.stone);
    }
    world.add_lantern(Vec3::new(-3.8, 0.0, -3.6), &palette);
    world.add_lantern(Vec3::new(4.8, 0.0, -2.4), &palette);

    // Cerca y camino hacia la cabaña.
    for x in -4..=4 {
        world.add_box(
            Vec3::new(x as f32 * 0.95, 0.04, -4.2),
            Vec3::new(0.78, 0.08, 0.75),
            palette.stone,
        );
    }
    for x in [-7.8, -5.8, 4.8, 6.8] {
        world.add_box(
            Vec3::new(x, 0.75, 3.0),
            Vec3::new(0.25, 1.5, 0.25),
            palette.wood,
        );
    }
    world.add_box(
        Vec3::new(-6.8, 0.8, 3.0),
        Vec3::new(2.2, 0.22, 0.22),
        palette.wood,
    );
    world.add_box(
        Vec3::new(5.8, 0.8, 3.0),
        Vec3::new(2.2, 0.22, 0.22),
        palette.wood,
    );

    let model = create_pardo(Vec3::new(0.2, 0.1, -1.2), 0.9, &palette);
    CharacterWorld::attach(world.finish(), model)
}

fn add_cabin(world: &mut VoxelBuilder, palette: &Palette) {
    let center = Vec3::new(0.0, 0.0, 4.6);
    world.add_box(
        center + Vec3::new(0.0, 1.9, 0.0),
        Vec3::new(7.2, 3.8, 3.8),
        palette.wood,
    );
    world.add_box(
        center + Vec3::new(0.0, 0.18, -2.2),
        Vec3::new(8.0, 0.36, 1.2),
        palette.dark_wood,
    );
    world.add_box(
        center + Vec3::new(0.0, 1.55, -1.96),
        Vec3::new(1.35, 2.8, 0.18),
        palette.dark_wood,
    );
    for x in [-2.25, 2.25] {
        world.add_box(
            center + Vec3::new(x, 2.0, -2.0),
            Vec3::new(1.4, 1.4, 0.2),
            palette.glass,
        );
    }
    for layer in 0..4 {
        world.add_box(
            center + Vec3::new(0.0, 4.0 + layer as f32 * 0.45, 0.0),
            Vec3::new(8.2 - layer as f32 * 0.7, 0.5, 5.0 - layer as f32 * 0.75),
            palette.dark_wood,
        );
    }
    world.add_box(
        center + Vec3::new(2.4, 5.0, 0.5),
        Vec3::new(0.7, 2.6, 0.7),
        palette.stone,
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::characters::CharacterKind;

    #[test]
    fn pardo_world_has_cabin_and_warm_lights() {
        let world = create_pardo_world();
        assert_eq!(world.character, CharacterKind::Pardo);
        assert!(world.scene.cubes.len() > 70);
        assert!(world.scene.lights.len() >= 3);
        assert!(world
            .scene
            .materials
            .iter()
            .any(|material| material.name == "Wood"));
        assert!(world
            .scene
            .materials
            .iter()
            .any(|material| material.name == "Metal"));
    }
}
