//! Materiales y texturas procedurales.

mod color;
mod surface;
mod texture;

pub use color::{pack_rgb, rgb, Color};
pub use surface::Material;
pub use texture::Texture;
