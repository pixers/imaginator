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

use std::rc::Rc;

pub mod img;
pub mod url;
pub mod filter;
pub mod cfg;
pub mod prelude;
pub struct PluginInformation {
    pub filters: filter::FilterMap,
    pub init: Option<&'static Fn() -> Result<(), failure::Error>>,
    pub exit: Option<&'static Fn() -> Result<(), failure::Error>>,
    pub middleware: Option<&'static Fn(Rc<hyper::Request>) -> Box<futures::future::Future<Item=Option<hyper::Response>, Error=failure::Error>>>
}

impl PluginInformation {
    pub fn new(filters: filter::FilterMap) -> Self {
        PluginInformation {
            filters: filters,
            init: None,
            exit: None,
            middleware: None
        }
    }

    pub fn with_init(mut self, init: &'static Fn() -> Result<(), failure::Error>) -> Self {
        self.init = Some(init);
        self
    }

    pub fn with_exit(mut self, exit: &'static Fn() -> Result<(), failure::Error>) -> Self {
        self.exit = Some(exit);
        self
    }

    pub fn with_middleware(mut self, middleware: &'static Fn(Rc<hyper::Request>) -> Box<futures::future::Future<Item=Option<hyper::Response>, Error=failure::Error>>) -> Self {
        self.middleware = Some(middleware);
        self
    }
}
