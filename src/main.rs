extern crate hyper;
extern crate futures;
extern crate pretty_env_logger;
extern crate num_cpus;
extern crate net2;
extern crate tokio_core;
extern crate magick_rust;
extern crate hyper_tls;
extern crate serde;
extern crate serde_yaml;
extern crate lru_disk_cache;
extern crate crypto;
extern crate base64;
extern crate flate2;
extern crate regex;
extern crate chrono;
extern crate zip;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate failure;
#[macro_use] extern crate nom;
#[macro_use] extern crate serde_derive;

extern crate imaginator_common as imaginator;
extern crate imaginator_plugins;

use std::alloc::System;

// When used in this program, jemalloc leaks virtual memory.
// Unfortunately, I don't know why. Regardless, using malloc fixes the problem.
#[global_allocator]
static ALLOCATOR: System = System;

mod cfg;
mod http;
mod url;
mod app;

fn main() {
    pretty_env_logger::init();
    http::server();
}
