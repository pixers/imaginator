#[macro_use] extern crate nom;
extern crate hyper;
#[macro_use] extern crate failure;
extern crate magick_rust;
extern crate base64;
extern crate crypto;
extern crate futures;
extern crate tokio_core;
extern crate serde;
#[macro_use] extern crate serde_derive;

pub mod img;
pub mod url;
pub mod filter;
pub mod cfg;
pub mod prelude;
pub struct PluginInformation {
    pub filters: filter::FilterMap,
    pub init: Option<&'static Fn() -> Result<(), failure::Error>>,
    pub exit: Option<&'static Fn() -> Result<(), failure::Error>>,
}
