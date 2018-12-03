use std::str::FromStr;
use crate::img::colorspace::Colorspace;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum ColorProfile {
    sRGB,
    Fogra39
}

#[derive(PartialEq,Eq,Debug,Fail)]
#[fail(display = "Unknown color profile: {}", _0)]
pub struct UnknownColorProfile(String);

impl<'a> From<&'a ColorProfile> for &'static [u8] {
    fn from(profile: &'a ColorProfile) -> Self {
        match *profile {
            ColorProfile::sRGB => include_bytes!("profiles/sRGB.icc"),
            ColorProfile::Fogra39 => include_bytes!("profiles/CMYK.icc"),
        }
    }
}


impl<'a> From<&'a ColorProfile> for Option<&'static [u8]> {
    fn from(profile: &'a ColorProfile) -> Self {
        Some(profile.into())
    }
}

impl<'a> From<&'a ColorProfile> for Colorspace {
    fn from(profile: &'a ColorProfile) -> Self {
        match profile {
            ColorProfile::sRGB => Colorspace::sRGB,
            ColorProfile::Fogra39 => Colorspace::CMYK
        }
    }
}


impl FromStr for ColorProfile {
    type Err = UnknownColorProfile;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let lowercase = input.to_owned().to_lowercase();
        Ok(match lowercase.as_str() {
            "srgb" => ColorProfile::sRGB,
            "fogra39" => ColorProfile::Fogra39,
            _ => return Err(UnknownColorProfile(input.to_owned())),
        })
    }
}
