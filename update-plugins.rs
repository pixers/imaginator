#!/usr/bin/env run-cargo-script
//cargo-deps: toml, failure
extern crate toml;
extern crate failure;

use std::fs::{File, read_dir};
use std::io::{Read, Write};
use std::path::Path;
use failure::Error;

fn get_package_name<P: AsRef<Path>>(path: P) -> Result<String, Error> {
    let mut data: String = String::new();
    File::open(path)?.read_to_string(&mut data)?;
    let doc: toml::Value = data.parse()?;
    Ok(doc.get("package").unwrap().get("name").unwrap().as_str().unwrap().to_owned())
}

fn write_plugin_modules() -> Result<(), Error> {
    let mut lib = File::create("plugins/lib.rs")?;
    let mut cargo = File::create("plugins/Cargo.toml")?;

    let main_pkg_name = get_package_name("Cargo.toml")?;
    writeln!(cargo, "[package]")?;
    writeln!(cargo, "name = \"{}-plugins\"", main_pkg_name)?;
    writeln!(cargo, "version = \"1.0.0\"")?;
    writeln!(cargo, "[lib]")?;
    writeln!(cargo, "path = \"lib.rs\"")?;
    writeln!(cargo, "\n[dependencies]")?;
    writeln!(cargo, "imaginator-common = {{ path = \"../common\" }}")?;
    writeln!(lib, "extern crate imaginator_common;")?;

    let mut package_names = vec![];
	for file in read_dir("plugins")? {
        let path = file?.path();
        if !path.is_dir() {
            continue
        }
        
        let pkg_dir = path.file_name().unwrap().to_string_lossy();
        if pkg_dir == "target" {
            continue
        }
        let pkg_name = get_package_name(format!("plugins/{}/Cargo.toml", pkg_dir))?;
        let rust_pkg_name = pkg_name.replace("-", "_");
        package_names.push(rust_pkg_name.clone());
        writeln!(cargo, "{} = {{ path = \"{}\" }}", pkg_name, pkg_dir)?;
        writeln!(lib, "pub extern crate {0};", rust_pkg_name)?;
	}

    writeln!(lib, "\nuse std::collections::HashMap;")?;
    writeln!(lib, "\npub fn plugins() -> HashMap<String, imaginator_common::PluginInformation> {{")?;
    writeln!(lib, "    let mut map = HashMap::new();")?;
    for pkg in package_names {
        writeln!(lib, "    map.insert(\"{0}\".to_owned(), {0}::plugin());", pkg)?;
    }
    writeln!(lib, "    map")?;
    writeln!(lib, "}}")?;
    Ok(())
}

fn main() {
    write_plugin_modules().unwrap();
}
