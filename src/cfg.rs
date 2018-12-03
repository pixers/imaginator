use std::fs::File;
use std::collections::HashMap;
use serde_yaml;
use std::any::Any;
use crate::imaginator::cfg::CONFIG as PLUGIN_CONFIG;

include!(concat!(env!("OUT_DIR"), "/cfg_plugins.rs"));

#[derive(Serialize,Deserialize)]
pub struct Config {
    pub secret: Option<String>,
    pub aliases: HashMap<String, String>,
    #[serde(flatten)]
    pub filters: Filters,
    pub allow_builtin_filters: bool,
    pub log_filters_header: Option<String>
}

lazy_static! {
    pub static ref CONFIG: Box<Config> = {
        let config: Box<Config> = Box::new(serde_yaml::from_reader(File::open("/etc/imaginator.yml").unwrap()).unwrap());
        let mut plugin_config = HashMap::new();
        config.filters.init_plugin_config(&mut plugin_config);
        unsafe {
            PLUGIN_CONFIG = Some(plugin_config);
        }
        config
    };
}
