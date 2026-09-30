use crate::{
    characters::{create_panda, create_pardo, create_polar, CharacterKind},
    geometry::Aabb,
    lighting::Light,
    material::rgb,
    math::{Ray, Vec3},
};

use super::{install_palette, Scene, Skybox, VoxelBuilder};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SelectionTarget {
    pub character: CharacterKind,
    pub bounds: Aabb,
    pedestal_index: usize,
}

#[derive(Debug, Clone)]
pub struct CharacterSelection {
    pub scene: Scene,
    pub targets: Vec<SelectionTarget>,
    hovered: Option<CharacterKind>,
    base_material: usize,
    highlight_material: usize,
}

impl CharacterSelection {
    #[must_use]
    pub fn hovered(&self) -> Option<CharacterKind> {
        self.hovered
    }

    #[must_use]
    pub fn pick(&self, ray: &Ray) -> Option<CharacterKind> {
        self.targets
            .iter()
            .filter_map(|target| {
                target
                    .bounds
                    .intersect(ray, 0.001, f32::INFINITY)
                    .map(|hit| (hit.distance, target.character))
            })
            .min_by(|left, right| left.0.total_cmp(&right.0))
            .map(|(_, character)| character)
    }

    /// Actualiza el halo del pedestal y devuelve `true` si cambió el hover.
    pub fn update_hover(&mut self, ray: &Ray) -> bool {
        let next = self.pick(ray);
        if next == self.hovered {
            return false;
        }
        self.hovered = next;
        for target in &self.targets {
            if let Some(pedestal) = self.scene.cubes.get_mut(target.pedestal_index) {
                pedestal.material_index = if Some(target.character) == self.hovered {
                    self.highlight_material
                } else {
                    self.base_material
                };
            }
        }
        true
    }
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
    let mut targets = Vec::with_capacity(models.len());
    for model in &models {
        let pedestal_index = world.scene.cubes.len();
        world.add_box(
            Vec3::new(model.pivot.x, 0.11, model.pivot.z),
            Vec3::new(3.7, 0.16, 3.3),
            palette.stone,
        );
        targets.push(SelectionTarget {
            character: model.kind,
            bounds: model.bounds,
            pedestal_index,
        });
    }
    for model in models {
        for cube in model.cubes {
            world.scene.add_cube(cube);
        }
    }

    CharacterSelection {
        scene: world.finish(),
        targets,
        hovered: None,
        base_material: palette.stone,
        highlight_material: palette.lantern,
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

    #[test]
    fn raycast_selects_and_highlights_pardo() {
        let mut selection = create_character_selection();
        let pardo = selection
            .targets
            .iter()
            .find(|target| target.character == CharacterKind::Pardo)
            .copied()
            .expect("Pardo target exists");
        let origin = Vec3::new(0.0, 3.0, -14.0);
        let ray = Ray::new(origin, pardo.bounds.center() - origin).expect("valid selection ray");

        assert_eq!(selection.pick(&ray), Some(CharacterKind::Pardo));
        assert!(selection.update_hover(&ray));
        assert_eq!(selection.hovered(), Some(CharacterKind::Pardo));
        assert_eq!(
            selection.scene.cubes[pardo.pedestal_index].material_index,
            selection.highlight_material
        );
    }
}
