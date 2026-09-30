use crate::{math::Vec3, scene::Palette};

use super::{CharacterKind, CharacterModel, ModelBuilder};

#[must_use]
pub fn create_panda(origin: Vec3, scale: f32, palette: &Palette) -> CharacterModel {
    let mut model = ModelBuilder::new(CharacterKind::Panda, origin, scale, palette);
    create_body(&mut model);
    create_head(&mut model);
    create_arms(&mut model);
    create_legs(&mut model);
    create_face(&mut model);
    model.finish()
}

fn create_body(model: &mut ModelBuilder<'_>) {
    model.add(
        Vec3::new(0.0, 2.55, 0.0),
        Vec3::new(2.75, 3.25, 1.85),
        model.palette.white,
    );
    model.add(
        Vec3::new(0.0, 3.65, 0.0),
        Vec3::new(2.9, 1.35, 1.95),
        model.palette.black,
    );
}

fn create_head(model: &mut ModelBuilder<'_>) {
    model.add(
        Vec3::new(0.0, 5.1, 0.0),
        Vec3::new(2.75, 2.25, 1.95),
        model.palette.white,
    );
    for x in [-1.02, 1.02] {
        model.add(
            Vec3::new(x, 6.15, 0.05),
            Vec3::new(0.75, 0.75, 0.7),
            model.palette.black,
        );
    }
}

fn create_arms(model: &mut ModelBuilder<'_>) {
    for x in [-1.62, 1.62] {
        model.add(
            Vec3::new(x, 4.25, 0.0),
            Vec3::new(0.9, 2.8, 1.0),
            model.palette.black,
        );
        model.add(
            Vec3::new(x, 5.48, -0.05),
            Vec3::new(1.0, 0.7, 1.05),
            model.palette.black,
        );
    }
}

fn create_legs(model: &mut ModelBuilder<'_>) {
    for x in [-0.78, 0.78] {
        model.add(
            Vec3::new(x, 0.72, 0.0),
            Vec3::new(1.2, 1.45, 1.45),
            model.palette.black,
        );
        model.add(
            Vec3::new(x, 0.18, -0.4),
            Vec3::new(1.32, 0.36, 1.8),
            model.palette.black,
        );
    }
}

fn create_face(model: &mut ModelBuilder<'_>) {
    for x in [-0.55, 0.55] {
        model.add(
            Vec3::new(x, 5.35, -1.05),
            Vec3::new(0.7, 0.78, 0.35),
            model.palette.black,
        );
        model.add(
            Vec3::new(x, 5.38, -1.27),
            Vec3::new(0.17, 0.2, 0.16),
            model.palette.white,
        );
    }
    model.add(
        Vec3::new(0.0, 4.82, -1.18),
        Vec3::new(1.2, 0.8, 0.48),
        model.palette.white,
    );
    model.add(
        Vec3::new(0.0, 4.96, -1.5),
        Vec3::new(0.42, 0.3, 0.25),
        model.palette.black,
    );
    model.add(
        Vec3::new(0.0, 4.55, -1.35),
        Vec3::new(0.48, 0.16, 0.18),
        model.palette.black,
    );
}
