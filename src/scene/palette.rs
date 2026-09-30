use crate::material::{rgb, Material, Texture};

use super::Scene;

/// Índices de la paleta compartida por todos los mundos.
#[derive(Debug, Clone, Copy)]
pub struct Palette {
    pub grass: usize,
    pub dirt: usize,
    pub stone: usize,
    pub wood: usize,
    pub dark_wood: usize,
    pub leaves: usize,
    pub water: usize,
    pub ice: usize,
    pub snow: usize,
    pub metal: usize,
    pub white: usize,
    pub black: usize,
    pub brown: usize,
    pub orange: usize,
    pub bamboo: usize,
    pub blue: usize,
    pub lantern: usize,
    pub glass: usize,
}

pub fn install_palette(scene: &mut Scene) -> Palette {
    let grass = scene.add_material(Material::new(
        "Grass",
        rgb(60, 132, 42),
        Texture::Checker {
            secondary: rgb(105, 164, 48),
            scale: 5.0,
        },
    ));
    let dirt = scene.add_material(Material::new(
        "Dirt",
        rgb(112, 73, 43),
        Texture::Speckled {
            secondary: rgb(82, 52, 34),
            scale: 3.5,
            amount: 0.28,
        },
    ));
    let stone = scene.add_material(
        Material::new(
            "Stone",
            rgb(96, 104, 111),
            Texture::Speckled {
                secondary: rgb(63, 72, 79),
                scale: 4.0,
                amount: 0.24,
            },
        )
        .with_surface(0.18, 20.0, 0.04),
    );
    let wood = scene.add_material(
        Material::new(
            "Wood",
            rgb(157, 78, 31),
            Texture::Grain {
                secondary: rgb(79, 35, 20),
                scale: 7.0,
            },
        )
        .with_surface(0.22, 28.0, 0.02),
    );
    let dark_wood = scene.add_material(Material::new(
        "Dark wood",
        rgb(78, 42, 29),
        Texture::Grain {
            secondary: rgb(43, 24, 19),
            scale: 8.0,
        },
    ));
    let leaves = scene.add_material(Material::new(
        "Leaves",
        rgb(39, 101, 45),
        Texture::Speckled {
            secondary: rgb(83, 137, 49),
            scale: 5.0,
            amount: 0.32,
        },
    ));
    let water = scene.add_material(
        Material::new(
            "Water",
            rgb(24, 126, 181),
            Texture::Ripples {
                secondary: rgb(83, 205, 236),
                scale: 5.0,
            },
        )
        .with_surface(0.9, 120.0, 0.22)
        .with_transmission(0.72, 1.333),
    );
    let ice = scene.add_material(
        Material::new(
            "Ice",
            rgb(165, 224, 239),
            Texture::Checker {
                secondary: rgb(205, 245, 249),
                scale: 3.0,
            },
        )
        .with_surface(0.95, 150.0, 0.28)
        .with_transmission(0.78, 1.31),
    );
    let snow = scene.add_material(
        Material::new(
            "Snow",
            rgb(235, 244, 244),
            Texture::Speckled {
                secondary: rgb(196, 222, 231),
                scale: 5.0,
                amount: 0.14,
            },
        )
        .with_surface(0.35, 48.0, 0.1),
    );
    let metal = scene.add_material(
        Material::new(
            "Metal",
            rgb(118, 126, 137),
            Texture::Checker {
                secondary: rgb(157, 164, 172),
                scale: 8.0,
            },
        )
        .with_surface(1.0, 180.0, 0.68),
    );
    let white = scene.add_material(
        Material::new("White fur", rgb(231, 225, 210), Texture::Solid)
            .with_surface(0.22, 32.0, 0.02),
    );
    let black = scene.add_material(
        Material::new("Black fur", rgb(29, 27, 30), Texture::Solid).with_surface(0.2, 26.0, 0.01),
    );
    let brown = scene.add_material(Material::new(
        "Brown fur",
        rgb(161, 76, 31),
        Texture::Speckled {
            secondary: rgb(123, 53, 25),
            scale: 4.0,
            amount: 0.12,
        },
    ));
    let orange = scene.add_material(Material::new("Warm fur", rgb(203, 91, 33), Texture::Solid));
    let bamboo = scene.add_material(Material::new(
        "Bamboo",
        rgb(122, 157, 42),
        Texture::Grain {
            secondary: rgb(73, 112, 31),
            scale: 5.0,
        },
    ));
    let blue = scene.add_material(
        Material::new("Blue fabric", rgb(42, 101, 174), Texture::Solid)
            .with_surface(0.3, 38.0, 0.03),
    );
    let lantern = scene.add_material(
        Material::new("Lantern", rgb(255, 173, 52), Texture::Solid)
            .with_surface(0.4, 64.0, 0.08)
            .with_emission(rgb(255, 126, 31) * 2.5),
    );
    let glass = scene.add_material(
        Material::new("Glass", rgb(205, 235, 235), Texture::Solid)
            .with_surface(1.0, 180.0, 0.2)
            .with_transmission(0.82, 1.5),
    );

    Palette {
        grass,
        dirt,
        stone,
        wood,
        dark_wood,
        leaves,
        water,
        ice,
        snow,
        metal,
        white,
        black,
        brown,
        orange,
        bamboo,
        blue,
        lantern,
        glass,
    }
}
