#[macro_use] extern crate imaginator_common as imaginator;
extern crate crypto;
extern crate hyper;
extern crate hyper_tls;
extern crate zip;
extern crate linked_hash_map;
extern crate futures;
#[macro_use] extern crate failure;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate serde_json;
extern crate bincode;
extern crate byteorder;
extern crate serde_humanize_rs;

use futures::{Future as FutureTrait};
use imaginator::prelude::*;
use imaginator::img::{CompositeOperator, Colorspace, ColorProfile, CompressionType, Filter as FilterType, ResolutionUnit, ImageFormat, AlphaChannel, Gravity};
use imaginator::filter::{Args, Future, exec_from_partial_url, Context};
use imaginator::cfg::config;
use std::collections::HashMap;

pub mod cfg;
pub use cfg::Config;

pub mod lru_cache;
pub mod cache;

pub mod download;

fn init_caches() -> Result<(), Error> {
    let caches = &config::<Config>().unwrap().caches;
    for key in caches.keys() {
        let _mutex_gurard = cache::cache(key)?;
    }

    Ok(())
}

fn save_caches() -> Result<(), Error> {
    let caches = &config::<Config>().unwrap().caches;
    for key in caches.keys() {
        cache::cache(key)?.export()?;
    }

    Ok(())
}

pub fn plugin() -> PluginInformation {
    let mut map: FilterMap = HashMap::new();
    map.insert("download", &download::filter);
    map.insert("resize", &resize);
    map.insert("fit-in", &fit_in);
    map.insert("compose", &compose);
    map.insert("trim", &trim);
    map.insert("crop", &crop);
    map.insert("extend", &extend);
    map.insert("format", &format);
    map.insert("pattern", &pattern);
    map.insert("repeat", &repeat);
    map.insert("colorspace", &colorspace);
    map.insert("profile", &profile);
    map.insert("compression", &compression);
    map.insert("resample", &resample);
    map.insert("canvas", &canvas);
    map.insert("cm", &cm);
    map.insert("sepia", &sepia);
    map.insert("flip", &flip);
    map.insert("flop", &flop);
    map.insert("cache", &cache::filter);
    map.insert("dpi", &dpi);
    map.insert("alpha", &alpha);
    map.insert("gravity", &gravity);
    map.insert("bg", &background);
    PluginInformation {
        filters: map,
        init: Some(&init_caches),
        exit: Some(&save_caches)
    }
}

image_filter!(fit_in(img: Image, context: &Context, mut w: isize, mut h: isize) {
    let cfg = &config::<Config>().unwrap().image;
    if let Some(max_w) = cfg.max_width {
        if w > max_w {
            w = max_w;
        }
    }

    if let Some(max_h) = cfg.max_height {
        if h > max_h {
            h = max_h;
        }
    }
    let ratio: f64 = (img.width() as f64)/(img.height() as f64);
    if w == 0 {
        w = ((h as f64) * ratio) as isize;
    }
    if h == 0 {
        h = ((w as f64) / ratio) as isize;
    }
    img.fit_in(w as usize, h as usize);
});

image_filter!(resize(img: Image, context: &Context, mut w: isize, mut h: isize, filter: Option<FilterType>) {
    let filter = filter.unwrap_or_default();
    let cfg = &config::<Config>().unwrap().image;
    if let Some(max_w) = cfg.max_width {
        if w > max_w {
            w = max_w;
        }
    }

    if let Some(max_h) = cfg.max_height {
        if h > max_h {
            h = max_h;
        }
    }
    let ratio: f64 = (img.width() as f64)/(img.height() as f64);
    if w == 0 {
        w = ((h as f64) * ratio) as isize;
    }
    if h == 0 {
        h = ((w as f64) / ratio) as isize;
    }
    img.resize(w as usize, h as usize, &filter);
});

image_filter!(resample(img: Image, context: &Context, x_dpi: f32, y_dpi: f32, filter: Option<FilterType>) {
    let x_dpi = x_dpi as f64;
    let y_dpi = y_dpi as f64;
    let filter = filter.unwrap_or_default();
    let (orig_x_dpi, orig_y_dpi) = img.resolution()?;
    let cfg = &config::<Config>().unwrap().image;
    if let Some(max_w) = cfg.max_width {
        let w = img.width() as f64 * x_dpi / orig_x_dpi;
        if w as isize > max_w {
            bail!("Cannot resample image to horizontal resolution {}, because it would exceed the maximum width of {} pixels.", x_dpi, max_w);
        }
    }

    if let Some(max_h) = cfg.max_height {
        let h = img.height() as f64 * y_dpi / orig_y_dpi;
        if h as isize > max_h {
            bail!("Cannot resample image to horizontal resolution {}, because it would exceed the maximum width of {} pixels.", y_dpi, max_h);
        }
    }
    
    img.resample(x_dpi, y_dpi, &filter);
});

image_filter!(trim(img: Image) {
    img.trim(15.0)?;
});

image_filter!(crop(img: Image, x: isize, y: isize, w: isize, h: isize) {
    img.crop(x, y, w as usize, h as usize)?;
});

image_filter!(extend(img: Image, x: isize, y: isize, mut w: isize, mut h: isize) {
    w = w - x;
    h = h - y;
    img.extend(x, y, w as usize, h as usize)?;
});

image_filter!(format(img: Image, format: ImageFormat) {
    img.set_format(&format)?;
});

pub fn compose(context: &mut Context, args: &Args) -> Box<Future> {
    let dst = arg_type!(compose, args, 0, context, Image);
    let src = arg_type!(compose, args, 1, context, Image);
    let args = args.clone();
    Box::new(dst.join(src).and_then(move |(dst, src)| {
        let op  = arg_type!(compose, args, 2, dst, CompositeOperator);
        let x = arg_type!(compose, args, 3, dst, isize);
        let y = arg_type!(compose, args, 4, dst, isize);
        dst.compose(&op, &src, x as isize, y as isize).map(|_| dst.into())
    }))
}

image_filter!(colorspace(img: Image, colorspace: Colorspace) {
    img.set_colorspace(&colorspace)?;
});

image_filter!(alpha(img: Image, alpha_channel: AlphaChannel) {
    img.set_alpha_channel(&alpha_channel)?;
});

image_filter!(compression(img: Image, compression: CompressionType) {
    img.set_compression(&compression)?;
});

image_filter!(profile(img: Image, source: ColorProfile, dest: ColorProfile) {
    let image_colorspace = img.colorspace();
    let source_colorspace: Colorspace = (&source).into();
    if image_colorspace != source_colorspace {
        bail!("profile: The image cannot be converted, becuase the image is in {:?} colorspace instead of {:?}.", image_colorspace, source_colorspace)
    };
    img.transform_color_profile(&source, &dest)?;
});

image_filter!(flip(img: Image) {
    img.flip()?;
});

image_filter!(flop(img: Image) {
    img.flop()?;
});

image_filter!(cm(img: Image, x: f32, y: f32) {
    let x = x * 0.3937008;
    let y = y * 0.3937008;
    let x_dpi = img.width() as f32 / x;
    let y_dpi = img.height() as f32 / y;

    img.set_resolution_unit(&ResolutionUnit::PixelsPerInch)?;
    img.set_resolution(x_dpi as f64, y_dpi as f64)?;
});

image_filter!(dpi(img: Image, horizontal: f32, vertical: f32) {
    img.set_resolution_unit(&ResolutionUnit::PixelsPerInch)?;
    img.set_resolution(horizontal as f64, vertical as f64)?;
});

image_filter!(sepia(img: Image, threshold: f32) {
    img.sepia(threshold as f64)?;
});

image_filter!(gravity(img: Image, gravity: Gravity) {
    img.set_gravity(&gravity)?;
});

image_filter!(repeat(img: Image, count_x: isize, count_y: isize, offset_x: isize, offset_y: isize) {
    for x in 1..count_x {
        img.compose(&CompositeOperator::Over, &img, x * offset_x, 0)?;
    }
    for y in 1..count_y {
        img.compose(&CompositeOperator::Over, &img, 0, y * offset_y)?;
    }
});

pub fn pattern(context: &mut Context, args: &Args) -> Box<Future> {
    let img = arg_type!(pattern, args, 0, context, Image);
    let mut context = context.clone();
    let args = args.clone();

    Box::new(img.and_then(move |img| {
        let width = arg_type!(pattern, args, 1, img, isize) as f32;
        let height = arg_type!(pattern, args, 2, img, isize) as f32;
        Ok((img, width, height))
    }).and_then(move |(img, width, height)| {
        let img_width = img.width() as f32;
        let img_height = img.height() as f32;

        let qty_x = (img_width/width).ceil() as isize;
        let qty_y = (img_height/height).ceil() as isize;
        exec_from_partial_url(&mut context, img, &format!("fit-in({w},{h}):extend(0,0,{},{}):repeat({},{},{w},{h})", img_width, img_height, qty_x, qty_y, w=width, h=height))
    }))
}

image_filter!(canvas(img: Image, border_x: f32, border_y: f32) {
    // Remove the 1px border from the image in case it contains artifacts from some
    // earlier processing.
    let border_x = border_x as f64;
    let border_y = border_y as f64;
    img.extend(1, 1, img.width()-2, img.height()-2)?;
    let mut flipped = img.clone();
    flipped.flip()?;
    let mut flopped = img.clone();
    flopped.flop()?;
    let mut flipflopped = img.clone();
    flipflopped.flip()?;
    flipflopped.flop()?;
    let (x_dpi, y_dpi) = img.resolution()?;
    let (w, h) = (img.width(), img.height());
    let x_border = (x_dpi * border_x * 0.3937008) as isize; // convert the border to inches
    let y_border = (y_dpi * border_y * 0.3937008) as isize; // convert the border to inches
    img.extend(-x_border, -y_border, w + (x_border as usize)*2, h + (y_border as usize)*2)?;
    img.compose(&CompositeOperator::Over, &flopped, x_border - w as isize, y_border)?;
    img.compose(&CompositeOperator::Over, &flopped, x_border + w as isize, y_border)?;
    img.compose(&CompositeOperator::Over, &flipped, x_border, y_border - h as isize)?;
    img.compose(&CompositeOperator::Over, &flipped, x_border, y_border + h as isize)?;
    img.compose(&CompositeOperator::Over, &flipflopped, x_border - w as isize, y_border + h as isize)?;
    img.compose(&CompositeOperator::Over, &flipflopped, x_border + w as isize, y_border - h as isize)?;
    img.compose(&CompositeOperator::Over, &flipflopped, x_border - w as isize, y_border - h as isize)?;
    img.compose(&CompositeOperator::Over, &flipflopped, x_border + w as isize, y_border + h as isize)?;
});

image_filter!(background(img: Image, color: String) {
    img.set_background_color(&color)?;
});
