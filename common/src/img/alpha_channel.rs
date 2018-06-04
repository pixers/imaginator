use magick_rust;
use std::str::FromStr;

#[derive(PartialEq,Eq,Debug,Fail)]
#[fail(display = "Unknown alpha channel option: {}", _0)]
pub struct UnknownAlphaChannelOption(String);

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum AlphaChannel {
    Undefined,
    Activate,
    Associate,
    Background,
    Copy,
    Deactivate,
    Discrete,
    Disassociate,
    Extract,
    Off,
    On,
    Opaque,
    Remove,
    Set,
    Shape,
    Transparent,
}

impl FromStr for AlphaChannel {
    type Err = UnknownAlphaChannelOption;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let lowercase = input.to_owned().to_lowercase();
        Ok(match lowercase.as_str() {
            "undefined" => AlphaChannel::Undefined,
            "activate" => AlphaChannel::Activate,
            "associate" => AlphaChannel::Associate,
            "background" => AlphaChannel::Background,
            "copy" => AlphaChannel::Copy,
            "deactivate" => AlphaChannel::Deactivate,
            "discrete" => AlphaChannel::Discrete,
            "disassociate" => AlphaChannel::Disassociate,
            "extract" => AlphaChannel::Extract,
            "off" => AlphaChannel::Off,
            "on" => AlphaChannel::On,
            "opaque" => AlphaChannel::Opaque,
            "remove" => AlphaChannel::Remove,
            "set" => AlphaChannel::Set,
            "shape" => AlphaChannel::Shape,
            "transparent" => AlphaChannel::Transparent,
            _ => return Err(UnknownAlphaChannelOption(input.to_owned()))
        })
    }
}

impl From<AlphaChannel> for magick_rust::bindings::AlphaChannelOption {
    fn from(from: AlphaChannel) -> magick_rust::bindings::AlphaChannelOption {
        match from {
            AlphaChannel::Undefined => magick_rust::bindings::AlphaChannelOption::UndefinedAlphaChannel,
            AlphaChannel::Activate => magick_rust::bindings::AlphaChannelOption::ActivateAlphaChannel,
            AlphaChannel::Associate => magick_rust::bindings::AlphaChannelOption::AssociateAlphaChannel,
            AlphaChannel::Background => magick_rust::bindings::AlphaChannelOption::BackgroundAlphaChannel,
            AlphaChannel::Copy => magick_rust::bindings::AlphaChannelOption::CopyAlphaChannel,
            AlphaChannel::Deactivate => magick_rust::bindings::AlphaChannelOption::DeactivateAlphaChannel,
            AlphaChannel::Discrete => magick_rust::bindings::AlphaChannelOption::DiscreteAlphaChannel,
            AlphaChannel::Disassociate => magick_rust::bindings::AlphaChannelOption::DisassociateAlphaChannel,
            AlphaChannel::Extract => magick_rust::bindings::AlphaChannelOption::ExtractAlphaChannel,
            AlphaChannel::Off => magick_rust::bindings::AlphaChannelOption::OffAlphaChannel,
            AlphaChannel::On => magick_rust::bindings::AlphaChannelOption::OnAlphaChannel,
            AlphaChannel::Opaque => magick_rust::bindings::AlphaChannelOption::OpaqueAlphaChannel,
            AlphaChannel::Remove => magick_rust::bindings::AlphaChannelOption::RemoveAlphaChannel,
            AlphaChannel::Set => magick_rust::bindings::AlphaChannelOption::SetAlphaChannel,
            AlphaChannel::Shape => magick_rust::bindings::AlphaChannelOption::ShapeAlphaChannel,
            AlphaChannel::Transparent => magick_rust::bindings::AlphaChannelOption::TransparentAlphaChannel,
        }
    }
}

impl From<magick_rust::bindings::AlphaChannelOption> for AlphaChannel {
    fn from(from: magick_rust::bindings::AlphaChannelOption) -> Self {
        match from {
            magick_rust::bindings::AlphaChannelOption::UndefinedAlphaChannel => AlphaChannel::Undefined,
            magick_rust::bindings::AlphaChannelOption::ActivateAlphaChannel => AlphaChannel::Activate,
            magick_rust::bindings::AlphaChannelOption::AssociateAlphaChannel => AlphaChannel::Associate,
            magick_rust::bindings::AlphaChannelOption::BackgroundAlphaChannel => AlphaChannel::Background,
            magick_rust::bindings::AlphaChannelOption::CopyAlphaChannel => AlphaChannel::Copy,
            magick_rust::bindings::AlphaChannelOption::DeactivateAlphaChannel => AlphaChannel::Deactivate,
            magick_rust::bindings::AlphaChannelOption::DiscreteAlphaChannel => AlphaChannel::Discrete,
            magick_rust::bindings::AlphaChannelOption::DisassociateAlphaChannel => AlphaChannel::Disassociate,
            magick_rust::bindings::AlphaChannelOption::ExtractAlphaChannel => AlphaChannel::Extract,
            magick_rust::bindings::AlphaChannelOption::OffAlphaChannel => AlphaChannel::Off,
            magick_rust::bindings::AlphaChannelOption::OnAlphaChannel => AlphaChannel::On,
            magick_rust::bindings::AlphaChannelOption::OpaqueAlphaChannel => AlphaChannel::Opaque,
            magick_rust::bindings::AlphaChannelOption::RemoveAlphaChannel => AlphaChannel::Remove,
            magick_rust::bindings::AlphaChannelOption::SetAlphaChannel => AlphaChannel::Set,
            magick_rust::bindings::AlphaChannelOption::ShapeAlphaChannel => AlphaChannel::Shape,
            magick_rust::bindings::AlphaChannelOption::TransparentAlphaChannel => AlphaChannel::Transparent,
        }
    }
}
