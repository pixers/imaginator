use magick_rust;
use std::str::FromStr;

#[derive(PartialEq,Eq,Debug,Fail)]
#[fail(display = "Unknown compression type: {}", _0)]
pub struct UnknownCompression(String);

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum CompressionType {
    Undefined,
    B44A,
    B44,
    BZip,
    DXT1,
    DXT3,
    DXT5,
    Fax,
    Group4,
    JBIG1,
    JBIG2,
    JPEG2000,
    JPEG,
    LosslessJPEG,
    LZMA,
    LZW,
    No,
    Piz,
    Pxr24,
    RLE,
    Zip,
    ZipS,
}


impl FromStr for CompressionType {
    type Err = UnknownCompression;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let lowercase = input.to_owned().to_lowercase();
        Ok(match lowercase.as_str() {
            "undefined" => CompressionType::Undefined,
            "b44a" => CompressionType::B44A,
            "b44" => CompressionType::B44,
            "bzip" => CompressionType::BZip,
            "dxt1" => CompressionType::DXT1,
            "dxt3" => CompressionType::DXT3,
            "dxt5" => CompressionType::DXT5,
            "fax" => CompressionType::Fax,
            "group4" => CompressionType::Group4,
            "jbig1" => CompressionType::JBIG1,
            "jbig2" => CompressionType::JBIG2,
            "jpeg2000" => CompressionType::JPEG2000,
            "jpeg" => CompressionType::JPEG,
            "losslessjpeg" => CompressionType::LosslessJPEG,
            "lzma" => CompressionType::LZMA,
            "lzw" => CompressionType::LZW,
            "no" => CompressionType::No,
            "piz" => CompressionType::Piz,
            "pxr24" => CompressionType::Pxr24,
            "rle" => CompressionType::RLE,
            "zip" => CompressionType::Zip,
            "zips" => CompressionType::ZipS,
            _ => return Err(UnknownCompression(input.to_owned()))
        })
    }
}

impl From<CompressionType> for magick_rust::bindings::CompressionType {
    fn from(from: CompressionType) -> magick_rust::bindings::CompressionType {
        match from {
            CompressionType::Undefined => magick_rust::bindings::CompressionType::UndefinedCompression,
            CompressionType::B44A => magick_rust::bindings::CompressionType::B44ACompression,
            CompressionType::B44 => magick_rust::bindings::CompressionType::B44Compression,
            CompressionType::BZip => magick_rust::bindings::CompressionType::BZipCompression,
            CompressionType::DXT1 => magick_rust::bindings::CompressionType::DXT1Compression,
            CompressionType::DXT3 => magick_rust::bindings::CompressionType::DXT3Compression,
            CompressionType::DXT5 => magick_rust::bindings::CompressionType::DXT5Compression,
            CompressionType::Fax => magick_rust::bindings::CompressionType::FaxCompression,
            CompressionType::Group4 => magick_rust::bindings::CompressionType::Group4Compression,
            CompressionType::JBIG1 => magick_rust::bindings::CompressionType::JBIG1Compression,
            CompressionType::JBIG2 => magick_rust::bindings::CompressionType::JBIG2Compression,
            CompressionType::JPEG2000 => magick_rust::bindings::CompressionType::JPEG2000Compression,
            CompressionType::JPEG => magick_rust::bindings::CompressionType::JPEGCompression,
            CompressionType::LosslessJPEG => magick_rust::bindings::CompressionType::LosslessJPEGCompression,
            CompressionType::LZMA => magick_rust::bindings::CompressionType::LZMACompression,
            CompressionType::LZW => magick_rust::bindings::CompressionType::LZWCompression,
            CompressionType::No => magick_rust::bindings::CompressionType::NoCompression,
            CompressionType::Piz => magick_rust::bindings::CompressionType::PizCompression,
            CompressionType::Pxr24 => magick_rust::bindings::CompressionType::Pxr24Compression,
            CompressionType::RLE => magick_rust::bindings::CompressionType::RLECompression,
            CompressionType::Zip => magick_rust::bindings::CompressionType::ZipCompression,
            CompressionType::ZipS => magick_rust::bindings::CompressionType::ZipSCompression,
        }
    }
}

impl From<magick_rust::bindings::CompressionType> for CompressionType {
    fn from(from: magick_rust::bindings::CompressionType) -> Self {
        match from {
            magick_rust::bindings::CompressionType::UndefinedCompression => CompressionType::Undefined,
            magick_rust::bindings::CompressionType::B44ACompression => CompressionType::B44A,
            magick_rust::bindings::CompressionType::B44Compression => CompressionType::B44,
            magick_rust::bindings::CompressionType::BZipCompression => CompressionType::BZip,
            magick_rust::bindings::CompressionType::DXT1Compression => CompressionType::DXT1,
            magick_rust::bindings::CompressionType::DXT3Compression => CompressionType::DXT3,
            magick_rust::bindings::CompressionType::DXT5Compression => CompressionType::DXT5,
            magick_rust::bindings::CompressionType::FaxCompression => CompressionType::Fax,
            magick_rust::bindings::CompressionType::Group4Compression => CompressionType::Group4,
            magick_rust::bindings::CompressionType::JBIG1Compression => CompressionType::JBIG1,
            magick_rust::bindings::CompressionType::JBIG2Compression => CompressionType::JBIG2,
            magick_rust::bindings::CompressionType::JPEG2000Compression => CompressionType::JPEG2000,
            magick_rust::bindings::CompressionType::JPEGCompression => CompressionType::JPEG,
            magick_rust::bindings::CompressionType::LosslessJPEGCompression => CompressionType::LosslessJPEG,
            magick_rust::bindings::CompressionType::LZMACompression => CompressionType::LZMA,
            magick_rust::bindings::CompressionType::LZWCompression => CompressionType::LZW,
            magick_rust::bindings::CompressionType::NoCompression => CompressionType::No,
            magick_rust::bindings::CompressionType::PizCompression => CompressionType::Piz,
            magick_rust::bindings::CompressionType::Pxr24Compression => CompressionType::Pxr24,
            magick_rust::bindings::CompressionType::RLECompression => CompressionType::RLE,
            magick_rust::bindings::CompressionType::ZipCompression => CompressionType::Zip,
            magick_rust::bindings::CompressionType::ZipSCompression => CompressionType::ZipS,
        }
    }
}
