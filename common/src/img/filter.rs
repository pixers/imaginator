use magick_rust;
use std::str::FromStr;

#[derive(PartialEq,Eq,Debug,Fail)]
#[fail(display = "Unknown resize filter: {}", _0)]
pub struct UnknownFilter(String);

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Filter {
    Undefined,
    Point,
    Box,
    Triangle,
    Hermite,
    Hann,
    Hamming,
    Blackman,
    Gaussian,
    Quadratic,
    Cubic,
    Catrom,
    Mitchell,
    Jinc,
    Sinc,
    SincFast,
    Kaiser,
    Welch,
    Parzen,
    Bohman,
    Bartlett,
    Lagrange,
    Lanczos,
    LanczosSharp,
    Lanczos2,
    Lanczos2Sharp,
    Robidoux,
    RobidouxSharp,
    Cosine,
    Spline,
    LanczosRadius,
    CubicSpline,
    Sentinel,
}


impl FromStr for Filter {
    type Err = UnknownFilter;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let lowercase = input.to_owned().to_lowercase();
        Ok(match lowercase.as_str() {
            "undefined" => Filter::Undefined,
            "point" => Filter::Point,
            "box" => Filter::Box,
            "triangle" => Filter::Triangle,
            "hermite" => Filter::Hermite,
            "hann" => Filter::Hann,
            "hamming" => Filter::Hamming,
            "blackman" => Filter::Blackman,
            "gaussian" => Filter::Gaussian,
            "quadratic" => Filter::Quadratic,
            "cubic" => Filter::Cubic,
            "catrom" => Filter::Catrom,
            "mitchell" => Filter::Mitchell,
            "jinc" => Filter::Jinc,
            "sinc" => Filter::Sinc,
            "sinc_fast" => Filter::SincFast,
            "kaiser" => Filter::Kaiser,
            "welch" => Filter::Welch,
            "parzen" => Filter::Parzen,
            "bohman" => Filter::Bohman,
            "bartlett" => Filter::Bartlett,
            "lagrange" => Filter::Lagrange,
            "lanczos" => Filter::Lanczos,
            "lanczos_sharp" => Filter::LanczosSharp,
            "lanczos2" => Filter::Lanczos2,
            "lanczos2_sharp" => Filter::Lanczos2Sharp,
            "robidoux" => Filter::Robidoux,
            "robidoux_sharp" => Filter::RobidouxSharp,
            "cosine" => Filter::Cosine,
            "spline" => Filter::Spline,
            "lanczos_radius" => Filter::LanczosRadius,
            "cubic_spline" => Filter::CubicSpline,
            "sentinel" => Filter::Sentinel,
            _ => return Err(UnknownFilter(input.to_owned()))
        })
    }
}

impl<'a> From<&'a Filter> for magick_rust::bindings::FilterType {
    fn from(from: &'a Filter) -> magick_rust::bindings::FilterType {
        match *from {
            Filter::Undefined => magick_rust::bindings::FilterType::UndefinedFilter,
            Filter::Point => magick_rust::bindings::FilterType::PointFilter,
            Filter::Box => magick_rust::bindings::FilterType::BoxFilter,
            Filter::Triangle => magick_rust::bindings::FilterType::TriangleFilter,
            Filter::Hermite => magick_rust::bindings::FilterType::HermiteFilter,
            Filter::Hann => magick_rust::bindings::FilterType::HannFilter,
            Filter::Hamming => magick_rust::bindings::FilterType::HammingFilter,
            Filter::Blackman => magick_rust::bindings::FilterType::BlackmanFilter,
            Filter::Gaussian => magick_rust::bindings::FilterType::GaussianFilter,
            Filter::Quadratic => magick_rust::bindings::FilterType::QuadraticFilter,
            Filter::Cubic => magick_rust::bindings::FilterType::CubicFilter,
            Filter::Catrom => magick_rust::bindings::FilterType::CatromFilter,
            Filter::Mitchell => magick_rust::bindings::FilterType::MitchellFilter,
            Filter::Jinc => magick_rust::bindings::FilterType::JincFilter,
            Filter::Sinc => magick_rust::bindings::FilterType::SincFilter,
            Filter::SincFast => magick_rust::bindings::FilterType::SincFastFilter,
            Filter::Kaiser => magick_rust::bindings::FilterType::KaiserFilter,
            Filter::Welch => magick_rust::bindings::FilterType::WelchFilter,
            Filter::Parzen => magick_rust::bindings::FilterType::ParzenFilter,
            Filter::Bohman => magick_rust::bindings::FilterType::BohmanFilter,
            Filter::Bartlett => magick_rust::bindings::FilterType::BartlettFilter,
            Filter::Lagrange => magick_rust::bindings::FilterType::LagrangeFilter,
            Filter::Lanczos => magick_rust::bindings::FilterType::LanczosFilter,
            Filter::LanczosSharp => magick_rust::bindings::FilterType::LanczosSharpFilter,
            Filter::Lanczos2 => magick_rust::bindings::FilterType::Lanczos2Filter,
            Filter::Lanczos2Sharp => magick_rust::bindings::FilterType::Lanczos2SharpFilter,
            Filter::Robidoux => magick_rust::bindings::FilterType::RobidouxFilter,
            Filter::RobidouxSharp => magick_rust::bindings::FilterType::RobidouxSharpFilter,
            Filter::Cosine => magick_rust::bindings::FilterType::CosineFilter,
            Filter::Spline => magick_rust::bindings::FilterType::SplineFilter,
            Filter::LanczosRadius => magick_rust::bindings::FilterType::LanczosRadiusFilter,
            Filter::CubicSpline => magick_rust::bindings::FilterType::CubicSplineFilter,
            Filter::Sentinel => magick_rust::bindings::FilterType::SentinelFilter,
        }
    }
}

impl From<magick_rust::bindings::FilterType> for Filter {
    fn from(from: magick_rust::bindings::FilterType) -> Self {
        match from {
            magick_rust::bindings::FilterType::UndefinedFilter => Filter::Undefined,
            magick_rust::bindings::FilterType::PointFilter => Filter::Point,
            magick_rust::bindings::FilterType::BoxFilter => Filter::Box,
            magick_rust::bindings::FilterType::TriangleFilter => Filter::Triangle,
            magick_rust::bindings::FilterType::HermiteFilter => Filter::Hermite,
            magick_rust::bindings::FilterType::HannFilter => Filter::Hann,
            magick_rust::bindings::FilterType::HammingFilter => Filter::Hamming,
            magick_rust::bindings::FilterType::BlackmanFilter => Filter::Blackman,
            magick_rust::bindings::FilterType::GaussianFilter => Filter::Gaussian,
            magick_rust::bindings::FilterType::QuadraticFilter => Filter::Quadratic,
            magick_rust::bindings::FilterType::CubicFilter => Filter::Cubic,
            magick_rust::bindings::FilterType::CatromFilter => Filter::Catrom,
            magick_rust::bindings::FilterType::MitchellFilter => Filter::Mitchell,
            magick_rust::bindings::FilterType::JincFilter => Filter::Jinc,
            magick_rust::bindings::FilterType::SincFilter => Filter::Sinc,
            magick_rust::bindings::FilterType::SincFastFilter => Filter::SincFast,
            magick_rust::bindings::FilterType::KaiserFilter => Filter::Kaiser,
            magick_rust::bindings::FilterType::WelchFilter => Filter::Welch,
            magick_rust::bindings::FilterType::ParzenFilter => Filter::Parzen,
            magick_rust::bindings::FilterType::BohmanFilter => Filter::Bohman,
            magick_rust::bindings::FilterType::BartlettFilter => Filter::Bartlett,
            magick_rust::bindings::FilterType::LagrangeFilter => Filter::Lagrange,
            magick_rust::bindings::FilterType::LanczosFilter => Filter::Lanczos,
            magick_rust::bindings::FilterType::LanczosSharpFilter => Filter::LanczosSharp,
            magick_rust::bindings::FilterType::Lanczos2Filter => Filter::Lanczos2,
            magick_rust::bindings::FilterType::Lanczos2SharpFilter => Filter::Lanczos2Sharp,
            magick_rust::bindings::FilterType::RobidouxFilter => Filter::Robidoux,
            magick_rust::bindings::FilterType::RobidouxSharpFilter => Filter::RobidouxSharp,
            magick_rust::bindings::FilterType::CosineFilter => Filter::Cosine,
            magick_rust::bindings::FilterType::SplineFilter => Filter::Spline,
            magick_rust::bindings::FilterType::LanczosRadiusFilter => Filter::LanczosRadius,
            magick_rust::bindings::FilterType::CubicSplineFilter => Filter::CubicSpline,
            magick_rust::bindings::FilterType::SentinelFilter => Filter::Sentinel,
        }
    }
}

impl Default for Filter {
    fn default() -> Self {
        Filter::Lanczos
    }
}
