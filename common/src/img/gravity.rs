use magick_rust;
use std::str::FromStr;

#[derive(PartialEq,Eq,Debug,Fail)]
#[fail(display = "Unknown gravity type: {}", _0)]
pub struct UnknownGravity(String);

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Gravity {
    Undefined,
    NorthWest,
    North,
    NorthEast,
    West,
    Center,
    East,
    SouthWest,
    South,
    SouthEast,
}

impl FromStr for Gravity {
    type Err = UnknownGravity;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let lowercase = input.to_owned().to_lowercase();
        Ok(match lowercase.as_str() {
            "undefined" => Gravity::Undefined,
            "north_west" => Gravity::NorthWest,
            "north" => Gravity::North,
            "north_east" => Gravity::NorthEast,
            "west" => Gravity::West,
            "center" => Gravity::Center,
            "east" => Gravity::East,
            "south_west" => Gravity::SouthWest,
            "south" => Gravity::South,
            "south_east" => Gravity::SouthEast,
            _ => return Err(UnknownGravity(input.to_owned()))
        })
    }
}

impl From<Gravity> for magick_rust::bindings::GravityType {
    fn from(from: Gravity) -> magick_rust::bindings::GravityType {
        match from {
            Gravity::Undefined => magick_rust::bindings::GravityType::UndefinedGravity,
            Gravity::NorthWest => magick_rust::bindings::GravityType::NorthWestGravity,
            Gravity::North => magick_rust::bindings::GravityType::NorthGravity,
            Gravity::NorthEast => magick_rust::bindings::GravityType::NorthEastGravity,
            Gravity::West => magick_rust::bindings::GravityType::WestGravity,
            Gravity::Center => magick_rust::bindings::GravityType::CenterGravity,
            Gravity::East => magick_rust::bindings::GravityType::EastGravity,
            Gravity::SouthWest => magick_rust::bindings::GravityType::SouthWestGravity,
            Gravity::South => magick_rust::bindings::GravityType::SouthGravity,
            Gravity::SouthEast => magick_rust::bindings::GravityType::SouthEastGravity,
        }
    }
}

impl From<magick_rust::bindings::GravityType> for Gravity {
    fn from(from: magick_rust::bindings::GravityType) -> Self {
        match from {
            magick_rust::bindings::GravityType::UndefinedGravity => Gravity::Undefined,
            magick_rust::bindings::GravityType::NorthWestGravity => Gravity::NorthWest,
            magick_rust::bindings::GravityType::NorthGravity => Gravity::North,
            magick_rust::bindings::GravityType::NorthEastGravity => Gravity::NorthEast,
            magick_rust::bindings::GravityType::WestGravity => Gravity::West,
            magick_rust::bindings::GravityType::CenterGravity => Gravity::Center,
            magick_rust::bindings::GravityType::EastGravity => Gravity::East,
            magick_rust::bindings::GravityType::SouthWestGravity => Gravity::SouthWest,
            magick_rust::bindings::GravityType::SouthGravity => Gravity::South,
            magick_rust::bindings::GravityType::SouthEastGravity => Gravity::SouthEast,
        }
    }
}
