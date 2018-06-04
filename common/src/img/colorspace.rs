use magick_rust;
use std::str::FromStr;

#[derive(PartialEq,Eq,Debug,Fail)]
#[fail(display = "Unknown colorspace: {}", _0)]
pub struct UnknownColorspace(String);

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum Colorspace {
    Undefined,
    CMY,
    CMYK,
    GRAY,
    HCL,
    HCLp,
    HSB,
    HSI,
    HSL,
    HSV,
    HWB,
    Lab,
    LCH,
    LCHab,
    LCHuv,
    Log,
    LMS,
    Luv,
    OHTA,
    Rec601YCbCr,
    Rec709YCbCr,
    RGB,
    scRGB,
    sRGB,
    Transparent,
    xyY,
    XYZ,
    YCbCr,
    YCC,
    YDbDr,
    YIQ,
    YPbPr,
    YUV,
    LinearGRAY,
}

impl FromStr for Colorspace {
    type Err = UnknownColorspace;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let lowercase = input.to_owned().to_lowercase();
        Ok(match lowercase.as_str() {
            "undefined" => Colorspace::Undefined,
            "cmy" => Colorspace::CMY,
            "cmyk" => Colorspace::CMYK,
            "gray" => Colorspace::GRAY,
            "hcl" => Colorspace::HCL,
            "hclp" => Colorspace::HCLp,
            "hsb" => Colorspace::HSB,
            "hsi" => Colorspace::HSI,
            "hsl" => Colorspace::HSL,
            "hsv" => Colorspace::HSV,
            "hwb" => Colorspace::HWB,
            "lab" => Colorspace::Lab,
            "lch" => Colorspace::LCH,
            "lchab" => Colorspace::LCHab,
            "lchuv" => Colorspace::LCHuv,
            "log" => Colorspace::Log,
            "lms" => Colorspace::LMS,
            "luv" => Colorspace::Luv,
            "ohta" => Colorspace::OHTA,
            "rec601ycbcr" => Colorspace::Rec601YCbCr,
            "rec709ycbcr" => Colorspace::Rec709YCbCr,
            "rgb" => Colorspace::RGB,
            "scrgb" => Colorspace::scRGB,
            "srgb" => Colorspace::sRGB,
            "transparent" => Colorspace::Transparent,
            "xyy" => Colorspace::xyY,
            "xyz" => Colorspace::XYZ,
            "ycbcr" => Colorspace::YCbCr,
            "ycc" => Colorspace::YCC,
            "ydbdr" => Colorspace::YDbDr,
            "yiq" => Colorspace::YIQ,
            "ypbpr" => Colorspace::YPbPr,
            "yuv" => Colorspace::YUV,
            "lineargray" => Colorspace::LinearGRAY,
            _ => return Err(UnknownColorspace(input.to_owned()))
        })
    }
}

impl From<Colorspace> for magick_rust::ColorspaceType {
    fn from(from: Colorspace) -> magick_rust::ColorspaceType {
        match from {
            Colorspace::Undefined => magick_rust::ColorspaceType::UndefinedColorspace,
            Colorspace::CMY => magick_rust::ColorspaceType::CMYColorspace,
            Colorspace::CMYK => magick_rust::ColorspaceType::CMYKColorspace,
            Colorspace::GRAY => magick_rust::ColorspaceType::GRAYColorspace,
            Colorspace::HCL => magick_rust::ColorspaceType::HCLColorspace,
            Colorspace::HCLp => magick_rust::ColorspaceType::HCLpColorspace,
            Colorspace::HSB => magick_rust::ColorspaceType::HSBColorspace,
            Colorspace::HSI => magick_rust::ColorspaceType::HSIColorspace,
            Colorspace::HSL => magick_rust::ColorspaceType::HSLColorspace,
            Colorspace::HSV => magick_rust::ColorspaceType::HSVColorspace,
            Colorspace::HWB => magick_rust::ColorspaceType::HWBColorspace,
            Colorspace::Lab => magick_rust::ColorspaceType::LabColorspace,
            Colorspace::LCH => magick_rust::ColorspaceType::LCHColorspace,
            Colorspace::LCHab => magick_rust::ColorspaceType::LCHabColorspace,
            Colorspace::LCHuv => magick_rust::ColorspaceType::LCHuvColorspace,
            Colorspace::Log => magick_rust::ColorspaceType::LogColorspace,
            Colorspace::LMS => magick_rust::ColorspaceType::LMSColorspace,
            Colorspace::Luv => magick_rust::ColorspaceType::LuvColorspace,
            Colorspace::OHTA => magick_rust::ColorspaceType::OHTAColorspace,
            Colorspace::Rec601YCbCr => magick_rust::ColorspaceType::Rec601YCbCrColorspace,
            Colorspace::Rec709YCbCr => magick_rust::ColorspaceType::Rec709YCbCrColorspace,
            Colorspace::RGB => magick_rust::ColorspaceType::RGBColorspace,
            Colorspace::scRGB => magick_rust::ColorspaceType::scRGBColorspace,
            Colorspace::sRGB => magick_rust::ColorspaceType::sRGBColorspace,
            Colorspace::Transparent => magick_rust::ColorspaceType::TransparentColorspace,
            Colorspace::xyY => magick_rust::ColorspaceType::xyYColorspace,
            Colorspace::XYZ => magick_rust::ColorspaceType::XYZColorspace,
            Colorspace::YCbCr => magick_rust::ColorspaceType::YCbCrColorspace,
            Colorspace::YCC => magick_rust::ColorspaceType::YCCColorspace,
            Colorspace::YDbDr => magick_rust::ColorspaceType::YDbDrColorspace,
            Colorspace::YIQ => magick_rust::ColorspaceType::YIQColorspace,
            Colorspace::YPbPr => magick_rust::ColorspaceType::YPbPrColorspace,
            Colorspace::YUV => magick_rust::ColorspaceType::YUVColorspace,
            Colorspace::LinearGRAY => magick_rust::ColorspaceType::LinearGRAYColorspace,
        }
    }
}

impl From<magick_rust::ColorspaceType> for Colorspace {
    fn from(from: magick_rust::ColorspaceType) -> Self {
        match from {
            magick_rust::ColorspaceType::UndefinedColorspace => Colorspace::Undefined,
            magick_rust::ColorspaceType::CMYColorspace => Colorspace::CMY,
            magick_rust::ColorspaceType::CMYKColorspace => Colorspace::CMYK,
            magick_rust::ColorspaceType::GRAYColorspace => Colorspace::GRAY,
            magick_rust::ColorspaceType::HCLColorspace => Colorspace::HCL,
            magick_rust::ColorspaceType::HCLpColorspace => Colorspace::HCLp,
            magick_rust::ColorspaceType::HSBColorspace => Colorspace::HSB,
            magick_rust::ColorspaceType::HSIColorspace => Colorspace::HSI,
            magick_rust::ColorspaceType::HSLColorspace => Colorspace::HSL,
            magick_rust::ColorspaceType::HSVColorspace => Colorspace::HSV,
            magick_rust::ColorspaceType::HWBColorspace => Colorspace::HWB,
            magick_rust::ColorspaceType::LabColorspace => Colorspace::Lab,
            magick_rust::ColorspaceType::LCHColorspace => Colorspace::LCH,
            magick_rust::ColorspaceType::LCHabColorspace => Colorspace::LCHab,
            magick_rust::ColorspaceType::LCHuvColorspace => Colorspace::LCHuv,
            magick_rust::ColorspaceType::LogColorspace => Colorspace::Log,
            magick_rust::ColorspaceType::LMSColorspace => Colorspace::LMS,
            magick_rust::ColorspaceType::LuvColorspace => Colorspace::Luv,
            magick_rust::ColorspaceType::OHTAColorspace => Colorspace::OHTA,
            magick_rust::ColorspaceType::Rec601YCbCrColorspace => Colorspace::Rec601YCbCr,
            magick_rust::ColorspaceType::Rec709YCbCrColorspace => Colorspace::Rec709YCbCr,
            magick_rust::ColorspaceType::RGBColorspace => Colorspace::RGB,
            magick_rust::ColorspaceType::scRGBColorspace => Colorspace::scRGB,
            magick_rust::ColorspaceType::sRGBColorspace => Colorspace::sRGB,
            magick_rust::ColorspaceType::TransparentColorspace => Colorspace::Transparent,
            magick_rust::ColorspaceType::xyYColorspace => Colorspace::xyY,
            magick_rust::ColorspaceType::XYZColorspace => Colorspace::XYZ,
            magick_rust::ColorspaceType::YCbCrColorspace => Colorspace::YCbCr,
            magick_rust::ColorspaceType::YCCColorspace => Colorspace::YCC,
            magick_rust::ColorspaceType::YDbDrColorspace => Colorspace::YDbDr,
            magick_rust::ColorspaceType::YIQColorspace => Colorspace::YIQ,
            magick_rust::ColorspaceType::YPbPrColorspace => Colorspace::YPbPr,
            magick_rust::ColorspaceType::YUVColorspace => Colorspace::YUV,
            magick_rust::ColorspaceType::LinearGRAYColorspace => Colorspace::LinearGRAY,
        }
    }
}
