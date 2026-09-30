//! Constructores de personajes voxel.

mod panda;
mod pardo;
mod polar;

use crate::{
    geometry::{Aabb, Cube},
    math::Vec3,
};

use crate::scene::Palette;

pub use panda::create_panda;
pub use pardo::create_pardo;
pub use polar::create_polar;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CharacterKind {
    Polar,
    Pardo,
    Panda,
}

#[derive(Debug, Clone)]
pub struct CharacterModel {
    pub kind: CharacterKind,
    pub pivot: Vec3,
    pub bounds: Aabb,
    pub cubes: Vec<Cube>,
}

impl CharacterModel {
    fn from_cubes(kind: CharacterKind, pivot: Vec3, cubes: Vec<Cube>) -> Self {
        let bounds = cubes
            .iter()
            .map(|cube| cube.bounds)
            .reduce(|left, right| left.union(&right))
            .unwrap_or_else(|| Aabb::new(pivot, pivot));
        Self {
            kind,
            pivot,
            bounds,
            cubes,
        }
    }
}

pub(super) struct ModelBuilder<'a> {
    kind: CharacterKind,
    origin: Vec3,
    scale: f32,
    palette: &'a Palette,
    cubes: Vec<Cube>,
}

impl<'a> ModelBuilder<'a> {
    pub(super) fn new(kind: CharacterKind, origin: Vec3, scale: f32, palette: &'a Palette) -> Self {
        Self {
            kind,
            origin,
            scale,
            palette,
            cubes: Vec::new(),
        }
    }

    pub(super) fn add(&mut self, center: Vec3, size: Vec3, material: usize) {
        self.cubes.push(Cube::new(
            self.origin + center * self.scale,
            size * self.scale,
            material,
        ));
    }

    pub(super) fn finish(self) -> CharacterModel {
        CharacterModel::from_cubes(self.kind, self.origin, self.cubes)
    }
}
