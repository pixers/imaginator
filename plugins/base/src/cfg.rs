use std::collections::HashMap;
use ::imaginator::img::ImageFormat;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ImageConfig {
    pub max_width: Option<isize>,
    pub max_height: Option<isize>,
    pub supported_formats: Option<Vec<ImageFormat>>,
}

impl Default for ImageConfig {
    fn default() -> Self {
        ImageConfig {
            max_width: None,
            max_height: None,
            supported_formats: None
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Cache {
    pub dir: String,
    pub size: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    #[serde(default)]
    pub image: ImageConfig,
    pub domains: HashMap<String,String>,
    pub caches: HashMap<String,Cache>
}
