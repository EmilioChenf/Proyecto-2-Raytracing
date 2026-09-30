use crate::{math::Vec3, scene::Palette};

use super::{CharacterKind, CharacterModel, ModelBuilder};

#[must_use]
pub fn create_polar(origin: Vec3, scale: f32, palette: &Palette) -> CharacterModel {
    let mut model = ModelBuilder::new(CharacterKind::Polar, origin, scale, palette);
    create_body(&mut model);
    create_head(&mut model);
    create_arms(&mut model);
    create_legs(&mut model);
    create_face(&mut model);
    create_bandana(&mut model);
    model.finish()
}

fn create_body(model: &mut ModelBuilder<'_>) {
    model.add(
        Vec3::new(0.0, 2.4, 0.0),
        Vec3::new(2.5, 3.2, 1.65),
        model.palette.white,
    );
    model.add(
        Vec3::new(0.0, 3.35, -0.05),
        Vec3::new(2.75, 1.7, 1.8),
        model.palette.white,
    );
}

fn create_head(model: &mut ModelBuilder<'_>) {
    model.add(
        Vec3::new(0.0, 5.0, 0.0),
        Vec3::new(2.65, 2.25, 1.9),
        model.palette.white,
    );
    for x in [-0.92, 0.92] {
        model.add(
            Vec3::new(x, 6.05, 0.0),
            Vec3::new(0.65, 0.65, 0.65),
            model.palette.white,
        );
    }
}

fn create_arms(model: &mut ModelBuilder<'_>) {
    model.add(
        Vec3::new(-1.3, 3.1, -0.7),
        Vec3::new(0.85, 2.2, 0.85),
        model.palette.white,
    );
    model.add(
        Vec3::new(1.3, 3.1, -0.7),
        Vec3::new(0.85, 2.2, 0.85),
        model.palette.white,
    );
    model.add(
        Vec3::new(0.0, 2.7, -1.18),
        Vec3::new(2.45, 0.75, 0.75),
        model.palette.white,
    );
}

fn create_legs(model: &mut ModelBuilder<'_>) {
    for x in [-0.68, 0.68] {
        model.add(
            Vec3::new(x, 0.7, 0.0),
            Vec3::new(1.05, 1.4, 1.35),
            model.palette.white,
        );
        model.add(
            Vec3::new(x, 0.18, -0.35),
            Vec3::new(1.2, 0.36, 1.7),
            model.palette.white,
        );
    }
}

fn create_face(model: &mut ModelBuilder<'_>) {
    model.add(
        Vec3::new(0.0, 4.72, -1.08),
        Vec3::new(1.35, 0.85, 0.45),
        model.palette.white,
    );
    for x in [-0.48, 0.48] {
        model.add(
            Vec3::new(x, 5.28, -1.0),
            Vec3::new(0.18, 0.22, 0.18),
            model.palette.black,
        );
    }
    model.add(
        Vec3::new(0.0, 4.82, -1.35),
        Vec3::new(0.42, 0.3, 0.25),
        model.palette.black,
    );
}

fn create_bandana(model: &mut ModelBuilder<'_>) {
    model.add(
        Vec3::new(0.0, 5.72, -1.06),
        Vec3::new(2.7, 0.28, 0.25),
        model.palette.blue,
    );
    model.add(
        Vec3::new(1.18, 6.05, 0.15),
        Vec3::new(0.35, 1.0, 0.35),
        model.palette.blue,
    );
}
