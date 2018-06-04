use failure::Error;
use magick_rust::{MagickWand, PixelWand, magick_wand_genesis};
use std::sync::{Once, ONCE_INIT};

mod composite_op;
mod colorspace;
mod color_profile;
mod compression;
mod filter;
mod resolution_unit;
mod format;
mod alpha_channel;
mod gravity;
pub use self::composite_op::CompositeOperator;
pub use self::colorspace::Colorspace;
pub use self::color_profile::ColorProfile;
pub use self::compression::CompressionType;
pub use self::filter::Filter;
pub use self::resolution_unit::ResolutionUnit;
pub use self::format::ImageFormat;
pub use self::alpha_channel::AlphaChannel;
pub use self::gravity::Gravity;

static START: Once = ONCE_INIT;

#[derive(Clone, Debug)]
pub struct Image {
    wand: MagickWand
}

#[allow(dead_code)]
impl Image {
    pub fn new<'a, T: Into<Option<&'a Vec<u8>>>, R: Into<Option<f64>>>(source: T, resolution: R) -> Result<Self, Error> {
        init_magick();
        let instance = Image {
            wand: MagickWand::new()
        };
        if let Some(resolution) = resolution.into() {
            instance.wand.set_resolution(resolution, resolution).map_err(|msg|
                format_err!("{}", msg)
            )?;
        }
        if let Some(source) = source.into() {
            instance.wand.read_image_blob(source).map_err(|msg|
                format_err!("{}", msg)
            )?;
        }
        Ok(instance)
    }

    pub fn ping(&mut self, source: impl AsRef<[u8]>) -> Result<(), Error> {
        self.wand.ping_image_blob(source).map_err(|msg| format_err!("{}", msg))
    }

    pub fn read(&mut self, source: &Vec<u8>) -> Result<(), Error> {
        self.wand.read_image_blob(source).map_err(|msg| format_err!("{}", msg))
    }

    pub fn encode(&self, format: ImageFormat) -> Result<Vec<u8>, Error> {
        match self.wand.write_image_blob(format.magick_str()) {
            Ok(val) => Ok(val),
            Err(msg) => bail!("{}", msg)
        }
    }

    pub fn resize(&self, w: usize, h: usize, filter: &Filter) {
        self.wand.resize_image(w, h, filter.into())
    }

    pub fn fit_in(&self, w: usize, h: usize) {
        self.wand.fit(w, h)
    }

    pub fn compose(&self, operator: &CompositeOperator, other: &Image, x: isize, y: isize) -> Result<(), Error> {
        self.wand.compose_images(&other.wand, (*operator).into(), true, x, y)
            .map_err(|msg| format_err!("{}", msg))
    }

    pub fn crop(&self, x: isize, y: isize, width: usize, height: usize) -> Result<(), Error> {
        self.wand.crop_image(width, height, x, y).map_err(|msg| format_err!("{}", msg))
    }

    pub fn extend(&self, x: isize, y: isize, width: usize, height: usize) -> Result<(), Error> {
        self.wand.extend_image(width, height, x, y).map_err(|msg| format_err!("{}", msg))
    }

    pub fn trim(&self, fuzz: f64) -> Result<(), Error> {
        self.wand.trim_image(fuzz).map_err(|msg| format_err!("{}", msg))
    }

    pub fn width(&self) -> usize {
        self.wand.get_image_width()
    }

    pub fn height(&self) -> usize {
        self.wand.get_image_height()
    }

    pub fn set_quality(&mut self, quality: usize) -> Result<(), Error> {
        self.wand.set_image_compression_quality(quality).map_err(|msg| format_err!("{}", msg))
    }

    pub fn format(&self) -> Result<ImageFormat, Error> {
        Ok(self.wand.get_image_format().map_err(|msg| format_err!("{}", msg))?.parse()?)
    }

    pub fn set_format(&mut self, format: &ImageFormat) -> Result<(), Error> {
        let result = self.wand.set_image_format(format.magick_str()).map_err(|msg| format_err!("{}", msg));
        if format == &ImageFormat::TIFF {
            // If we don't set that, some popular photo editing programs
            // might have problems with opening the file.
            self.wand.set_option("tiff:rows-per-strip", "2").map_err(|msg| format_err!("{}", msg))?;
        } else if format == &ImageFormat::PDF {
            // For some reason, page geometry can assume incorrect values.
            // Resetting it seems to help.
            self.wand.reset_image_page("0x0").map_err(|msg| format_err!("{}", msg))?;
            // Also, for some reason ImageMagick needs the wand resolution set for PDFs.
            let (x_dpi, y_dpi) = self.resolution()?;
            self.wand.set_resolution(x_dpi, y_dpi).map_err(|msg| format_err!("{}", msg))?;
        }
        result
    }

    pub fn colorspace(&self) -> Colorspace {
        self.wand.get_image_colorspace().into()
    }

    pub fn set_colorspace(&mut self, colorspace: &Colorspace) -> Result<(), Error> {
        self.wand.transform_image_colorspace(colorspace.to_owned().into()).map_err(|msg| format_err!("{}", msg))
    }

    pub fn gravity(&self) -> Gravity {
        self.wand.get_gravity().into()
    }

    pub fn set_gravity(&mut self, gravity: &Gravity) -> Result<(), Error> {
        self.wand.set_gravity(gravity.to_owned().into()).map_err(|msg| format_err!("{}", msg))
    }

    pub fn set_alpha_channel(&mut self, alpha_channel: &AlphaChannel) -> Result<(), Error> {
        self.wand.set_image_alpha_channel(alpha_channel.to_owned().into()).map_err(|msg| format_err!("{}", msg))
    }

    pub fn compression(&self) -> CompressionType {
        self.wand.get_compression().into()
    }

    pub fn set_compression(&mut self, compression: &CompressionType) -> Result<(), Error> {
        self.wand.set_compression(compression.to_owned().into()).map_err(|msg| format_err!("{}", msg))
    }

    pub fn transform_color_profile(&mut self, src_profile: &ColorProfile, dest_profile: &ColorProfile) -> Result<(), Error> {
        self.wand.profile_image("icc", src_profile).map_err(|msg| format_err!("{}", msg))?;
        self.wand.profile_image("icc", dest_profile).map_err(|msg| format_err!("{}", msg))
    }

    pub fn set_color_profile(&mut self, profile: &ColorProfile) -> Result<(), Error> {
        self.wand.profile_image("icc", profile).map_err(|msg| format_err!("{}", msg))
    }

    pub fn resample(&mut self, x_dpi: f64, y_dpi: f64, filter: &Filter) {
        self.wand.resample_image(x_dpi, y_dpi, filter.into())
    }

    pub fn resolution(&self) -> Result<(f64, f64), Error> {
        self.wand.get_image_resolution().map_err(|msg| format_err!("{}", msg))
    }

    pub fn set_resolution(&mut self, x_dpi: f64, y_dpi: f64) -> Result<(), Error> {
        self.wand.profile_image("8bim", None).map_err(|msg| format_err!("{}", msg))?;
        self.wand.set_image_resolution(x_dpi, y_dpi).map_err(|msg| format_err!("{}", msg))
    }

    pub fn resolution_unit(&self) -> ResolutionUnit {
        self.wand.get_image_units().into()
    }

    pub fn set_resolution_unit(&mut self, unit: &ResolutionUnit) -> Result<(), Error> {
        self.wand.set_image_units(unit.to_owned().into()).map_err(|msg| format_err!("{}", msg))
    }

    pub fn set_background_color(&self, color: &str) -> Result<(), Error> {
        let mut pw = PixelWand::new();
        pw.set_color(color).map_err(|msg| format_err!("{}", msg))?;
        self.wand.set_image_background_color(&pw).map_err(|msg| format_err!("{}", msg))
    }

    pub fn flip(&mut self) -> Result<(), Error> {
        self.wand.flip_image().map_err(|msg| format_err!("{}", msg))
    }

    pub fn flop(&mut self) -> Result<(), Error> {
        self.wand.flop_image().map_err(|msg| format_err!("{}", msg))
    }

    pub fn sepia(&mut self, threshold: f64) -> Result<(), Error> {
        self.wand.sepia_tone_image(threshold).map_err(|msg| format_err!("{}", msg))
    }
}


fn init_magick() {
    START.call_once(|| {
        magick_wand_genesis();
    });
}

