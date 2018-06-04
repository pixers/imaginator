use magick_rust;
use std::str::FromStr;

#[derive(PartialEq,Eq,Debug,Fail)]
#[fail(display = "Unknown resolution unit: {}", _0)]
pub struct UnknownResolutionUnit(String);

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ResolutionUnit {
    Undefined,
    PixelsPerInch,
    PixelsPerCentimeter,
}


impl FromStr for ResolutionUnit {
    type Err = UnknownResolutionUnit;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let lowercase = input.to_owned().to_lowercase();
        Ok(match lowercase.as_str() {
            "undefined" => ResolutionUnit::Undefined,
            "dpi" => ResolutionUnit::PixelsPerInch,
            "dpc" => ResolutionUnit::PixelsPerCentimeter,
            _ => return Err(UnknownResolutionUnit(input.to_owned()))
        })
    }
}

impl From<ResolutionUnit> for magick_rust::bindings::ResolutionType {
    fn from(from: ResolutionUnit) -> Self {
        match from {
            ResolutionUnit::Undefined => magick_rust::bindings::ResolutionType::UndefinedResolution,
            ResolutionUnit::PixelsPerInch => magick_rust::bindings::ResolutionType::PixelsPerInchResolution,
            ResolutionUnit::PixelsPerCentimeter => magick_rust::bindings::ResolutionType::PixelsPerCentimeterResolution,
        }
    }
}

impl From<magick_rust::bindings::ResolutionType> for ResolutionUnit {
    fn from(from: magick_rust::bindings::ResolutionType) -> Self {
        match from {
            magick_rust::bindings::ResolutionType::UndefinedResolution => ResolutionUnit::Undefined,
            magick_rust::bindings::ResolutionType::PixelsPerInchResolution => ResolutionUnit::PixelsPerInch,
            magick_rust::bindings::ResolutionType::PixelsPerCentimeterResolution => ResolutionUnit::PixelsPerCentimeter,
        }
    }
}
