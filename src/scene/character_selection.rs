use crate::{
    characters::{create_panda, create_pardo, create_polar, CharacterKind},
    geometry::Aabb,
    lighting::Light,
    material::rgb,
    math::Vec3,
};

use super::{install_palette, Scene, Skybox, VoxelBuilder};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SelectionTarget {
    pub character: CharacterKind,
    pub bounds: Aabb,
}

#[derive(Debug, Clone)]
pub struct CharacterSelection {
    pub scene: Scene,
    pub targets: Vec<SelectionTarget>,
}

#[must_use]
pub fn create_character_selection() -> CharacterSelection {
    let mut scene = Scene::new(rgb(232, 221, 207), 0.28).with_skybox(Skybox::main_world());
    let palette = install_palette(&mut scene);
    scene.lights.push(Light::Directional {
        direction: Vec3::new(0.5, -1.0, -0.6),
        color: rgb(255, 238, 218),
        intensity: 1.45,
    });
    scene.lights.push(Light::Point {
        position: Vec3::new(0.0, 9.0, -6.0),
        color: rgb(255, 210, 168),
        intensity: 5.0,
    });

    let models = [
        create_polar(Vec3::new(-5.2, 0.25, 0.0), 0.78, &palette),
        create_pardo(Vec3::new(0.0, 0.25, 0.0), 0.78, &palette),
        create_panda(Vec3::new(5.2, 0.25, 0.0), 0.78, &palette),
    ];
    let targets = models
        .iter()
        .map(|model| SelectionTarget {
            character: model.kind,
            bounds: model.bounds,
        })
        .collect();

    let mut world = VoxelBuilder::new(scene);
    world.add_box(
        Vec3::new(0.0, -0.2, 0.0),
        Vec3::new(15.5, 0.5, 7.0),
        palette.stone,
    );
    world.add_box(
        Vec3::new(0.0, -0.52, 0.0),
        Vec3::new(16.2, 0.3, 7.7),
        palette.dark_wood,
    );
    for x in [-7.2, -6.4, -1.0, 1.0, 6.4, 7.2] {
        world.add_voxel(Vec3::new(x, 0.3, 2.7), palette.grass);
    }
    for model in models {
        for cube in model.cubes {
            world.scene.add_cube(cube);
        }
    }

    CharacterSelection {
        scene: world.finish(),
        targets,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn selection_contains_three_independent_models() {
        let selection = create_character_selection();
        assert_eq!(selection.targets.len(), 3);
        assert!(selection.scene.cubes.len() > 40);
        assert!(selection
            .targets
            .iter()
            .any(|target| target.character == CharacterKind::Panda));
    }
}
