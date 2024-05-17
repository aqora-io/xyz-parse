mod atom;
mod molecule;
mod xyz;

#[cfg(feature = "python")]
mod python;

pub use atom::*;
pub use molecule::*;
pub use xyz::*;

pub fn parse_xyz(s: &str) -> Result<Xyz, XyzParseError> {
    Xyz::parse(s)
}
