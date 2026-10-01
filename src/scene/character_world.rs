use std::ops::Range;

use crate::{
    characters::{CharacterKind, CharacterModel},
    geometry::Cube,
    math::Vec3,
};

use super::Scene;

/// Escena individual y metadatos que permiten animar solo al personaje.
#[derive(Debug, Clone)]
pub struct CharacterWorld {
    pub scene: Scene,
    pub character: CharacterKind,
    pub character_range: Range<usize>,
    pub character_pivot: Vec3,
    pub base_character_cubes: Vec<Cube>,
}

impl CharacterWorld {
    #[must_use]
    pub fn attach(mut scene: Scene, model: CharacterModel) -> Self {
        let start = scene.cubes.len();
        let base_character_cubes = model.cubes;
        scene.cubes.extend(base_character_cubes.iter().copied());
        let end = scene.cubes.len();
        scene.rebuild_acceleration();

        Self {
            scene,
            character: model.kind,
            character_range: start..end,
            character_pivot: model.pivot,
            base_character_cubes,
        }
    }

    pub fn set_character_rotation(&mut self, angle: f32) {
        for (target, original) in self
            .scene
            .cubes
            .get_mut(self.character_range.clone())
            .into_iter()
            .flatten()
            .zip(&self.base_character_cubes)
        {
            *target = original.rotated_about_y(self.character_pivot, angle);
        }
        self.scene.rebuild_acceleration();
    }
}

#[cfg(test)]
mod tests {
    use crate::scene::create_panda_world;

    #[test]
    fn rotation_changes_only_character_geometry() {
        let mut world = create_panda_world();
        let environment = world.scene.cubes[..world.character_range.start].to_vec();
        let character = world.scene.cubes[world.character_range.clone()].to_vec();

        world.set_character_rotation(std::f32::consts::FRAC_PI_2);

        assert_eq!(
            world.scene.cubes[..world.character_range.start],
            environment
        );
        assert_ne!(world.scene.cubes[world.character_range.clone()], character);
        assert!(world.scene.acceleration_node_count() > 1);
    }
}
