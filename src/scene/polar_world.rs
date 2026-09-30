use crate::{characters::create_polar, lighting::Light, material::rgb, math::Vec3};

use super::{install_palette, CharacterWorld, Scene, Skybox, VoxelBuilder};

#[must_use]
pub fn create_polar_world() -> CharacterWorld {
    let mut scene = Scene::new(rgb(170, 215, 235), 0.17).with_skybox(Skybox::polar());
    let palette = install_palette(&mut scene);
    scene.lights.push(Light::Directional {
        direction: Vec3::new(0.55, -1.0, -0.35),
        color: rgb(214, 242, 255),
        intensity: 1.6,
    });
    scene.lights.push(Light::Point {
        position: Vec3::new(-5.0, 7.5, -4.0),
        color: rgb(153, 217, 255),
        intensity: 5.2,
    });
    scene.lights.push(Light::Point {
        position: Vec3::new(-1.5, 5.5, -6.0),
        color: rgb(218, 241, 255),
        intensity: 3.2,
    });

    let mut world = VoxelBuilder::new(scene);
    world.add_box(
        Vec3::new(0.0, -0.5, 0.0),
        Vec3::new(17.0, 1.0, 13.0),
        palette.ice,
    );
    world.add_box(
        Vec3::new(0.0, -1.25, 0.0),
        Vec3::new(15.8, 0.6, 11.8),
        palette.stone,
    );
    world.add_box(
        Vec3::new(-3.8, 0.08, 0.8),
        Vec3::new(8.2, 0.18, 8.0),
        palette.snow,
    );
    // Lago abierto y placas de hielo transparentes.
    world.add_box(
        Vec3::new(4.5, 0.05, 0.6),
        Vec3::new(6.4, 0.22, 6.5),
        palette.water,
    );
    for (x, z, size) in [
        (3.2, -1.0, 1.4),
        (5.3, 1.8, 1.1),
        (4.0, 2.4, 0.8),
        (6.2, -1.8, 0.9),
    ] {
        world.add_box(
            Vec3::new(x, 0.24, z),
            Vec3::new(size, 0.28, size * 0.8),
            palette.ice,
        );
    }

    // Pequeña cueva glaciar detrás del personaje.
    world.add_box(
        Vec3::new(0.0, 2.0, 5.25),
        Vec3::new(7.0, 4.0, 1.0),
        palette.ice,
    );
    world.add_box(
        Vec3::new(0.0, 2.0, 5.82),
        Vec3::new(5.4, 3.2, 0.18),
        palette.stone,
    );
    world.add_box(
        Vec3::new(-3.0, 1.4, 3.7),
        Vec3::new(1.0, 2.8, 3.2),
        palette.ice,
    );
    world.add_box(
        Vec3::new(3.0, 1.4, 3.7),
        Vec3::new(1.0, 2.8, 3.2),
        palette.ice,
    );
    world.add_box(
        Vec3::new(0.0, 3.55, 4.0),
        Vec3::new(5.2, 0.9, 3.0),
        palette.snow,
    );

    for (x, z, height) in [
        (-7.0, -4.2, 4.4),
        (-6.4, 4.4, 5.0),
        (6.7, 4.3, 4.7),
        (7.2, -4.0, 4.1),
        (-2.8, 4.7, 4.0),
    ] {
        world.add_pine(Vec3::new(x, 0.0, z), height, &palette, true);
    }
    for (x, z, scale) in [
        (-6.5, -0.5, 1.0),
        (-4.8, -4.5, 1.25),
        (6.7, 1.2, 0.9),
        (2.0, -4.8, 0.8),
    ] {
        world.add_rock(Vec3::new(x, 0.0, z), scale, palette.snow);
    }
    // Cristales verticales para mostrar reflexión y transmisión.
    for (x, z, height) in [
        (-5.4, 2.0, 1.5),
        (-4.9, 2.5, 0.9),
        (6.7, 3.0, 1.3),
        (7.1, 3.5, 0.75),
        (2.0, 5.0, 1.2),
    ] {
        world.add_box(
            Vec3::new(x, height * 0.5, z),
            Vec3::new(0.48, height, 0.48),
            palette.ice,
        );
    }

    // Ventanas de hielo delante de núcleos oscuros: hacen visible el cambio
    // de dirección producido por IOR 1.31 en lugar de perderse en la nieve.
    for x in [-4.6, 4.6] {
        world.add_box(
            Vec3::new(x, 0.95, -1.8),
            Vec3::new(0.58, 1.55, 0.58),
            palette.stone,
        );
        world.add_box(
            Vec3::new(x, 1.15, -2.65),
            Vec3::new(1.35, 2.3, 0.92),
            palette.ice,
        );
    }
    world.add_box(
        Vec3::new(-0.8, 0.08, -1.2),
        Vec3::new(4.0, 0.16, 3.1),
        palette.blue,
    );

    let model = create_polar(Vec3::new(-0.8, 0.2, -1.2), 0.92, &palette);
    CharacterWorld::attach(world.finish(), model)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::characters::CharacterKind;

    #[test]
    fn polar_world_emphasizes_refractive_materials() {
        let world = create_polar_world();
        assert_eq!(world.character, CharacterKind::Polar);
        assert!(world.scene.cubes.len() > 70);
        assert!(world
            .scene
            .materials
            .iter()
            .any(|material| (material.refractive_index - 1.31).abs() < 0.001));
        assert!(world
            .scene
            .materials
            .iter()
            .any(|material| (material.refractive_index - 1.333).abs() < 0.001));
    }
}
