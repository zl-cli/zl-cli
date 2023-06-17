use std::fs::read_to_string;
use std::path::Path;
use toml::de::Error;

mod config;

pub type ZConfig = config::ZConfig;

impl ZConfig{
    fn from_file(file: &Path) -> Result<ZConfig, Error>{
        let content = read_to_string(file)?.as_str();

        toml::from_str(content)
    }
}