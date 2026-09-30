use crate::{math::Vec3, scene::Palette};

use super::{CharacterKind, CharacterModel, ModelBuilder};

#[must_use]
pub fn create_pardo(origin: Vec3, scale: f32, palette: &Palette) -> CharacterModel {
    let mut model = ModelBuilder::new(CharacterKind::Pardo, origin, scale, palette);
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
        Vec3::new(2.9, 3.45, 1.9),
        model.palette.brown,
    );
    model.add(
        Vec3::new(0.0, 3.55, -0.05),
        Vec3::new(3.15, 1.75, 2.0),
        model.palette.orange,
    );
}

fn create_head(model: &mut ModelBuilder<'_>) {
    model.add(
        Vec3::new(0.0, 5.05, 0.0),
        Vec3::new(2.8, 2.2, 2.0),
        model.palette.brown,
    );
    for x in [-0.98, 0.98] {
        model.add(
            Vec3::new(x, 6.08, 0.0),
            Vec3::new(0.7, 0.65, 0.68),
            model.palette.brown,
        );
        model.add(
            Vec3::new(x, 6.08, -0.38),
            Vec3::new(0.34, 0.3, 0.12),
            model.palette.orange,
        );
    }
}

fn create_arms(model: &mut ModelBuilder<'_>) {
    model.add(
        Vec3::new(-2.0, 4.0, -0.05),
        Vec3::new(2.0, 0.92, 1.0),
        model.palette.orange,
    );
    model.add(
        Vec3::new(2.0, 4.35, -0.05),
        Vec3::new(2.0, 0.92, 1.0),
        model.palette.orange,
    );
    model.add(
        Vec3::new(-2.9, 4.0, -0.05),
        Vec3::new(0.65, 1.05, 1.05),
        model.palette.brown,
    );
    model.add(
        Vec3::new(2.9, 4.35, -0.05),
        Vec3::new(0.65, 1.05, 1.05),
        model.palette.brown,
    );
}

fn create_legs(model: &mut ModelBuilder<'_>) {
    for x in [-0.82, 0.82] {
        model.add(
            Vec3::new(x, 0.72, 0.0),
            Vec3::new(1.25, 1.45, 1.5),
            model.palette.brown,
        );
        model.add(
            Vec3::new(x, 0.18, -0.4),
            Vec3::new(1.4, 0.36, 1.9),
            model.palette.brown,
        );
    }
}

fn create_face(model: &mut ModelBuilder<'_>) {
    model.add(
        Vec3::new(0.0, 3.0, -1.02),
        Vec3::new(1.65, 1.85, 0.18),
        model.palette.orange,
    );
    model.add(
        Vec3::new(0.0, 4.75, -1.18),
        Vec3::new(1.4, 0.95, 0.55),
        model.palette.orange,
    );
    model.add(
        Vec3::new(0.0, 4.98, -1.5),
        Vec3::new(0.48, 0.33, 0.28),
        model.palette.black,
    );
    for x in [-0.53, 0.53] {
        model.add(
            Vec3::new(x, 5.42, -1.08),
            Vec3::new(0.2, 0.24, 0.2),
            model.palette.black,
        );
    }
    model.add(
        Vec3::new(0.0, 4.48, -1.35),
        Vec3::new(0.5, 0.18, 0.2),
        model.palette.black,
    );
    model.add(
        Vec3::new(0.0, 4.27, -1.33),
        Vec3::new(0.18, 0.22, 0.14),
        model.palette.black,
    );
}
