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

        Self {
            scene,
            character: model.kind,
            character_range: start..end,
            character_pivot: model.pivot,
            base_character_cubes,
        }
    }
}
