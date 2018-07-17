use std::fmt;
use img::{self, Image};
use url;
use std::fmt::Display;
use std::str::FromStr;
use hyper;
use futures::{Future as FutureTrait, IntoFuture, future};
use tokio_core::reactor::Remote;
use failure::{Fail, Error};
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug,Clone,Eq,PartialEq)]
pub enum SizeUnit {
    None,
    Px,
    Width,
    Height,
    HorizontalCentimeters,
    VerticalCentimeters,
    HorizontalInches,
    VerticalInches
}

#[derive(PartialEq,Eq,Debug,Fail)]
#[fail(display = "Unknown size unit: {}", _0)]
pub struct UnknownSizeUnit(String);

impl FromStr for SizeUnit {
    type Err = UnknownSizeUnit;
    fn from_str(input: &str) -> Result<Self,Self::Err> {
        Ok(match input {
            "" => SizeUnit::None,
            "px" => SizeUnit::Px,
            "w" => SizeUnit::Width,
            "h" => SizeUnit::Height,
            "hcm" => SizeUnit::HorizontalCentimeters,
            "vcm" => SizeUnit::VerticalCentimeters,
            "hin" => SizeUnit::HorizontalInches,
            "vin" => SizeUnit::VerticalInches,
            _ => return Err(UnknownSizeUnit(input.to_owned()))
        })
    }
}

#[derive(Debug,Clone)]
pub enum FilterArg {
    Int(isize, SizeUnit),
	Float(f32, SizeUnit),
    String(String),
    Img(Filter),
    ResolvedImg(img::Image),
}

#[derive(Debug,Clone)]
pub struct Filter {
    pub name: String,
    pub args: Vec<FilterArg>
}

impl Display for FilterArg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FilterArg::Int(ref v, ref u) => write!(f, "{}{:?}", v, u),
            FilterArg::Float(ref v, ref u) => write!(f, "{}{:?}", v, u),
            FilterArg::String(ref v) => write!(f, "{}", v),
            FilterArg::Img(ref v) => write!(f, "{}", v),
            FilterArg::ResolvedImg(_) => unimplemented!()
        }
    }
}

impl Display for Filter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}(", self.name)?;
        write!(f, "{}", self.args.iter().map(|arg| format!("{}", arg)).collect::<Vec<_>>().join(","))?;
        write!(f, ")")
    }
}

#[derive(Debug,Fail)]
#[fail(display="ErrorResponse")] // This shouldn't actually be used.
pub struct ErrorResponse(pub Box<FilterResult + Sync + Send>);

pub trait FilterResult: fmt::Debug {
    fn content_type(&self) -> Result<hyper::header::ContentType, Error>;
    fn dpi(&self) -> Result<(f64, f64), Error> { bail!("This filter does not return an image!"); }
    fn status_code(&self) -> hyper::StatusCode { hyper::StatusCode::Ok }
    fn content(&self) -> Rc<Vec<u8>>;
    fn image(self: Box<Self>) -> Result<img::Image, Error> { bail!("This filter does not return an image!") }
}

impl FilterResult for img::Image {
    fn content_type(&self) -> Result<hyper::header::ContentType, Error> {
        Ok(hyper::header::ContentType::from(&self.format()?))
    }

    fn dpi(&self) -> Result<(f64, f64), Error> {
        self.resolution()
    }

    fn content(&self) -> Rc<Vec<u8>> {
        Rc::new(self.encode(self.format().unwrap()).unwrap())
    }

    fn image(self: Box<Self>) -> Result<img::Image, Error> {
        Ok(*self)
    }
}

impl FilterResult for ErrorResponse {
    fn content_type(&self) -> Result<hyper::header::ContentType, Error> { self.0.content_type() }
    fn status_code(&self) -> hyper::StatusCode { self.0.status_code() }
    fn content(&self) -> Rc<Vec<u8>> { self.0.content() }
    fn image(self: Box<Self>) -> Result<img::Image, Error> { self.0.image() }
}

impl<T: FilterResult + 'static> From<Box<T>> for Box<FilterResult> {
    fn from(obj: Box<T>) -> Self {
        obj
    }
}

pub fn as_future_image(f: Box<FutureImage>) -> Box<FutureImage> {
    f
}

pub type FutureImage = FutureTrait<Item = Box<img::Image>, Error = Error>;
pub type Future = FutureTrait<Item = Box<FilterResult>, Error = Error>;
pub type Args = Vec<FilterArg>;
pub type FilterMap = HashMap<&'static str, &'static (Fn(&mut Context, &Args) -> Box<Future> + Sync)>;

#[derive(Clone)]
pub struct Context {
    pub filters: &'static FilterMap,
    pub remote: Remote,
    pub log_filters_header: &'static Option<String>,
    pub response_headers: Rc<HashMap<String, String>>
}

pub fn parse_size<T: Into<f32>>(val: T, unit: &SizeUnit, img: &Image) -> Result<f32, Error> {
        let val = val.into();
        Ok(match *unit {
            SizeUnit::None | SizeUnit::Px => val,
            SizeUnit::Width => (val * img.width() as f32),
            SizeUnit::Height => (val * img.height() as f32),
            SizeUnit::HorizontalCentimeters => (val * 0.3937008 * img.resolution()?.0 as f32),
            SizeUnit::VerticalCentimeters => (val * 0.3937008 * img.resolution()?.1 as f32),
            SizeUnit::HorizontalInches => (val * img.resolution()?.0 as f32),
            SizeUnit::VerticalInches => (val * img.resolution()?.1 as f32),
        })
}

pub trait ArgType: Sized {
    /// This trait exists to be used by the arg_type! macro.
    ///
    /// It exists, because this macro has to return a future, and therefore,
    /// depends on the futures crate. It could just `use futures;`, but then
    /// this code would be just pasted at call size, requiring the caller to
    /// `extern crate futures;`. This is undesirable.
    /// If we extract this dependency to something defined here, like a function,
    /// this problem is solved.
    fn arg_type(name: &str, args: &Args, i: usize) -> Result<Self, Box<Future>>;
}

pub trait ArgTypeImg: Sized + FromStr {
    /// This trait is similar to AugType, but requires an additional parameter and returns
    /// an immediate result instead of a future.
    /// And it has a default implementation for enum support.
    fn arg_type_img(name: &str, typename: &str, args: &Args, i: usize, _img: &Image) -> Result<Self, Error> where <Self as FromStr>::Err: Fail + Send + Sync + 'static {
        match args.get(i) {
            Some(&FilterArg::String(ref val)) => val.parse::<Self>().map_err(Error::from),
            _ => return Err(format_err!("Argument {} to `{}` must be an {}", i + 1, name, typename))
        }
    }
}

pub trait ArgTypeContext: Sized {
    /// This trait is similar to AugType, but requires a Context.
    fn arg_type_context(name: &str, args: &Args, i: usize, context: &mut Context) -> Self;
}

impl ArgType for String {
    fn arg_type(name: &str, args: &Args, i: usize) -> Result<Self, Box<Future>> {
        match args.get(i) {
            Some(&FilterArg::String(ref s)) => Ok(s.clone()),
            _ => Err(Box::new(future::err::<_, Error>(format_err!("Argument {} to `{}` must be a string", i + 1, name))))
        }
    }
}

impl ArgType for isize {
    fn arg_type(name: &str, args: &Args, i: usize) -> Result<Self, Box<Future>> {
        match args.get(i) {
            Some(&FilterArg::Int(ref val, SizeUnit::None)) => Ok(val.clone()),
            _ => Err(Box::new(future::err::<_, Error>(format_err!("Argument {} to `{}` must be a literal integer", i + 1, name))))
        }
    }
}

impl ArgTypeImg for String {
    fn arg_type_img(name: &str, _: &str, args: &Args, i: usize, _img: &Image) -> Result<Self, Error> {
        match args.get(i) {
            Some(&FilterArg::String(ref s)) => Ok(s.clone()),
            _ => Err(format_err!("Argument {} to `{}` must be a string", i + 1, name))
        }
    }
}

impl ArgTypeImg for isize {
    fn arg_type_img(name: &str, _: &str, args: &Args, i: usize, img: &Image) -> Result<Self, Error> {
        match args.get(i) {
            Some(&FilterArg::Int(ref val, ref unit)) => match *unit {
                SizeUnit::None | SizeUnit::Px => Ok(val.clone()),
                _ => Ok(parse_size(val.clone() as f32, unit, &img)? as isize)
            },
            Some(&FilterArg::Float(ref val, ref unit)) => match *unit {
                SizeUnit::None | SizeUnit::Px => Err(format_err!("Unit px does not support fractional values.")),
                _ => Ok(parse_size(val.clone(), unit, &img)? as isize)
            },
            _ => return Err(format_err!("Argument {} to `{}` must be an integer", i + 1, name))
        }
    }
}

impl ArgTypeImg for f32 {
    fn arg_type_img(name: &str, _: &str, args: &Args, i: usize, img: &Image) -> Result<Self, Error> {
        match args.get(i) {
            Some(&FilterArg::Int(ref val, ref unit)) => match *unit {
                SizeUnit::None | SizeUnit::Px => Ok(val.clone() as f32),
                _ => parse_size(val.clone() as f32, unit, &img)
            },
            Some(&FilterArg::Float(ref val, ref unit)) => match *unit {
                SizeUnit::None | SizeUnit::Px => Ok(val.clone()),
                _ => parse_size(val.clone(), unit, &img)
            },
            _ => Err(format_err!("Argument {} to `{}` must be a float", i + 1, name))
        }
    }
}

impl ArgType for Filter {
    fn arg_type(name: &str, args: &Args, i: usize) -> Result<Self, Box<Future>> {
        match args.get(i) {
            Some(&FilterArg::Img(ref val)) => Ok(val.clone()),
            _ => Err(Box::new(future::err::<_, Error>(format_err!("Argument {} to `{}` must be an image", i + 1, name))))
        }
    }
}

impl ArgTypeContext for Box<FutureImage> {
    fn arg_type_context(name: &str, args: &Args, i: usize, context: &mut Context) -> Self {
        match args.get(i) {
            Some(&FilterArg::Img(ref val)) =>
                as_future_image(Box::new(exec_filter(context, val).and_then(|img| Ok(Box::new(img.image()?))))),
            Some(&FilterArg::ResolvedImg(ref val)) =>
                as_future_image(Box::new(future::ok(Box::new(val.clone())))),
            _ => return Box::new(future::err::<_, Error>(format_err!("Argument {} to `{}` must be an image", i + 1, name)))
        }
    }
}

impl ArgTypeImg for img::CompositeOperator {}
impl ArgTypeImg for img::Colorspace {}
impl ArgTypeImg for img::ColorProfile {}
impl ArgTypeImg for img::CompressionType {}
impl ArgTypeImg for img::Filter {}
impl ArgTypeImg for img::ResolutionUnit {}
impl ArgTypeImg for img::ImageFormat {}
impl ArgTypeImg for img::AlphaChannel {}
impl ArgTypeImg for img::Gravity {}

#[macro_export]
macro_rules! arg_type {
    ($name:ident, $args:expr, $i:expr, String) => {{
        use $crate::filter;
        match <String as filter::ArgType>::arg_type(stringify!($name), &$args, $i) {
            Ok(v) => v,
            Err(e) => return e
        }
    }};
    ($name:ident, $args:expr, $i:expr, isize) => {{
        use $crate::filter;
        match <isize as filter::ArgType>::arg_type(stringify!($name), &$args, $i) {
            Ok(v) => v,
            Err(e) => return e
        }
    }};
    ($name:ident, $args:expr, $i:expr, $img:expr, isize) => {{
        use $crate::filter;
        match <isize as filter::ArgTypeImg>::arg_type_img(stringify!($name), "", &$args, $i, &$img) {
            Ok(v) => v,
            Err(e) => return Err(e)
        }
    }};
    ($name:ident, $args:expr, $i:expr, $img:expr, f32) => {{
        use $crate::filter;
        match <f32 as filter::ArgTypeImg>::arg_type_img(stringify!($name), "", &$args, $i, &$img) {
            Ok(v) => v,
            Err(e) => return Err(e)
        }
    }};
    ($name:ident, $args:expr, $i:expr, Filter) => {{
        use $crate::filter;
        match <Filter as filter::ArgType>::arg_type(stringify!($name), &$args, $i) {
            Ok(v) => v,
            Err(e) => return e
        }
    }};
    ($name:ident, $args:expr, $i:expr, $context:expr, Image) => {{
        use $crate::filter;
        <Box<filter::FutureImage> as filter::ArgTypeContext>::arg_type_context(
            stringify!($name), &$args, $i, $context
        )
    }};
    ($name:ident, $args:expr, $i:expr, $img:expr, $type:ty) => {{
        use $crate::filter;
        match <$type as filter::ArgTypeImg>::arg_type_img(stringify!($name), stringify!($type), &$args, $i, &$img) {
            Ok(v) => v,
            Err(e) => return Err(e)
        }
    }};
}

#[macro_export]
macro_rules! image_filter_args {
    // mut arg: Type
    ($name: ident, $args:ident, $img:ident, $i:expr, mut $arg_name:ident : $arg_type:tt) => {
        let mut $arg_name = arg_type!($name, $args, $i, $img, $arg_type);
    };
    // arg: Type
    ($name: ident, $args:ident, $img:ident, $i:expr, $arg_name:ident : $arg_type:tt) => {
        let $arg_name = arg_type!($name, $args, $i, $img, $arg_type);
    };
    // mut arg: Option<Type>
    ($name: ident, $args:ident, $img:ident, $i:expr, mut $arg_name:ident : Option<$arg_type:tt>) => {
        let mut $arg_name = if $args.len() > $i {
            Some(arg_type!($name, $args, $i, $img, $arg_type))
        } else { None };
    };
    // arg: Option<Type>
    ($name: ident, $args:ident, $img:ident, $i:expr, $arg_name:ident : Option<$arg_type:tt>) => {
        let $arg_name = if $args.len() > $i {
            Some(arg_type!($name, $args, $i, $img, $arg_type))
        } else { None };
    };
    // mut arg, args
    ($name: ident, $args:ident, $img:ident, $i:expr, mut $arg_name:ident : $arg_type:tt, $( $rest:tt )+) => {
        image_filter_args!($name, $args, $img, $i, mut $arg_name: $arg_type);
        image_filter_args!($name, $args, $img, $i + 1, $($rest)+);
    };
    // arg, args
    ($name: ident, $args:ident, $img:ident, $i:expr, $arg_name:ident : $arg_type:tt, $( $rest:tt )+) => {
        image_filter_args!($name, $args, $img, $i, $arg_name: $arg_type);
        image_filter_args!($name, $args, $img, $i + 1, $($rest)+);
    };
    // empty list of args
    ($name: ident, $args:ident, $img:ident, $i:expr, ) => {};
}

#[macro_export]
macro_rules! image_filter {
    ($name:ident ($img:ident : Image ) $body:block) => {
        image_filter!($name($img: Image, ) $body);
    };

    ($name:ident ($img:ident : Image, $ctx:ident : &Context, $( $args:tt )* ) $body:block) => {
        pub fn $name($ctx: &mut $crate::filter::Context, args: &$crate::filter::Args) -> Box<$crate::filter::Future> {
            let img = arg_type!($name, args, 0, $ctx, Image);

            #[allow(unused_variables)]
            let args = args.clone();
            #[allow(unused_mut)] // Because Image doesn't need to be mutable right now.
            Box::new(img.and_then(move |mut $img| {
                image_filter_args!($name, args, $img, 1, $($args)*);
                $body
                Ok($img.into())
            }))
        }
    };

    ($name:ident ($img:ident : Image, $( $args:tt )* ) $body:block) => {
        image_filter!($name($img: Image, ctx: &Context, $($args)*) $body);
    }
}

#[allow(dead_code)]
fn inject_img(img: Box<img::Image>, filter: &mut Filter) {
    if filter.args.len() > 0 {
        if let FilterArg::Img(ref mut inner) = filter.args[0] {
            if inner.name != "__img__" {
                inject_img(img, inner);
                return
            }
        } else {
            return
        }
    }
    filter.args[0] = FilterArg::ResolvedImg(*img);
}

pub fn exec_from_partial_url(context: &mut Context, img: Box<img::Image>, url: &str) -> Box<Future> {
    let mut context = context.clone();
    let mut subst_url = "__img__():".to_owned();
    subst_url.push_str(url);
    Box::new(url::parse(&subst_url).into_future().and_then(move |mut filter| {
        inject_img(img, &mut filter);
        exec_filter(&mut context, &filter)
    }))
}

fn log_filter(context: &mut Context, filter: &Filter) -> Result<(), Error> {
    let header_name = if let Some(ref name) = context.log_filters_header {
        name
    } else {
        return Ok(())
    };

    let headers = match Rc::get_mut(&mut context.response_headers) {
        Some(headers) => headers,
        None => bail!("Can't log usage of filter {}", filter.name)
    };
    let mut route = String::new();
    if let Some(val) = headers.get(header_name) {
        route.push_str(val);
    }
    if route.len() > 0 {
        route.push_str(",")
    }
    route.push_str(filter.name.as_str());
    headers.insert(header_name.to_owned(), route);
    Ok(())
}

pub fn exec_filter(context: &mut Context, filter: &Filter) -> Box<Future> {
    if let Err(e) = log_filter(context, filter) {
        return Box::new(future::err(e));
    }
    match context.filters.get(filter.name.as_str()) {
        Some(f) => (f)(context, &filter.args),
        None => Box::new(future::err(format_err!("no such filter: {}", filter.name)))
    }
}
