extern crate imaginator_plugins;
extern crate mustache;

use std::env;
use std::fs::File;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let mut f = File::create(Path::new(&out_dir).join("cfg_plugins.rs")).unwrap();
    let template = mustache::compile_path("cfg_plugins.rs.template").unwrap();

    let map = mustache::MapBuilder::new().insert_vec("plugins", |mut builder| {
        for (name, _) in imaginator_plugins::plugins().iter() {
            builder = builder.push_str(name);
        }
        builder
    }).build();
    template.render_data(&mut f, &map).unwrap();
}
