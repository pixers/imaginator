use hyper::header::ContentType;
use std::str::FromStr;

#[derive(PartialEq,Eq,Debug,Fail)]
#[fail(display = "Unknown format: {}", _0)]
pub struct UnknownImageFormat(String);

#[derive(PartialEq, Eq, Clone, Debug, Serialize, Deserialize)]
pub enum ImageFormat {
    Undefined,
    PNG,
    JPEG,
    TIFF,
    PDF,
    PS,
    SVG,
}

impl ImageFormat {
    pub fn magick_str(&self) -> &str {
        match *self {
            ImageFormat::PNG => "PNG",
            ImageFormat::JPEG => "JPEG",
            ImageFormat::TIFF => "TIF",
            ImageFormat::PDF => "PDF",
            ImageFormat::PS => "PS",
            ImageFormat::SVG => "SVG",
            ImageFormat::Undefined => ""
        }
    }
}

impl FromStr for ImageFormat {
    type Err = UnknownImageFormat;
    fn from_str(src: &str) -> Result<Self, Self::Err> {
        Ok(match src {
            "PNG" | "png" => ImageFormat::PNG,
            "JPEG" | "JPG" | "jpeg" | "jpg" => ImageFormat::JPEG,
            "TIFF" | "TIF" | "tiff" | "tif" => ImageFormat::TIFF,
            "PDF" | "pdf" => ImageFormat::PDF,
            "PS" | "ps" => ImageFormat::PS,
            "SVG" | "svg" => ImageFormat::SVG,
            _ => return Err(UnknownImageFormat(src.to_owned()))
        })
    }
}

impl<'a> From<&'a ImageFormat> for ContentType {
    fn from(src: &'a ImageFormat) -> Self {
        match *src {
            ImageFormat::PNG => ContentType::png(),
            ImageFormat::JPEG => ContentType::jpeg(),
            ImageFormat::TIFF => ContentType("image/tiff".parse().unwrap()),
            ImageFormat::PDF => ContentType("application/pdf".parse().unwrap()),
            ImageFormat::PS => ContentType("application/postscript".parse().unwrap()),
            ImageFormat::SVG => ContentType("image/svg+xml".parse().unwrap()),
            ImageFormat::Undefined => ContentType::plaintext()
        }
    }
}

impl From<ImageFormat> for ContentType {
    fn from(src: ImageFormat) -> Self {
        From::from(&src)
    }
}
