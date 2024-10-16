//! # Art
//!
//! A library for modeling artistic concepts.
//!

pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;

pub mod kinds {
    /// The primary colors according to the RYB color model.
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// The secondary colors according to the RYB color model.
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use crate::art::kinds::{PrimaryColor, SecondaryColor};
    use crate::art::kinds::SecondaryColor::Orange;

    /// Combines two primary colors in equal amounts to create
    /// a secondary color.
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        Orange
    }
}