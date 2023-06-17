use std::fs::read_to_string;
use std::io::Write;
use std::path::Path;
use crate::zerror::ZError;

mod config;

pub type ZConfig = config::ZConfig;

#[allow(dead_code)]
impl ZConfig{
    fn from_file(file: &Path) -> Result<ZConfig, ZError>{
        let content = read_to_string(file)?;
        let config = toml::from_str(content.as_str())?;

        Ok(config)
    }

    fn write_to_file(config: &ZConfig, file: &Path) -> Result<(), ZError>{
        let content = toml::to_string(config)?;
        let mut config_file = std::fs::File::open(file)?;

        config_file.write_all(content.as_bytes())?;

        Ok(())
    }
}