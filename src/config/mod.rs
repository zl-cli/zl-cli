use std::fs::{File, read_to_string};
use std::io::{Read, Write};
use crate::config::config::ZGlobalConfig;
use crate::zerror::ZError;

mod config;

pub type ZConfig = config::ZConfig;

#[allow(dead_code)]
impl ZConfig{
    pub fn default() -> ZConfig{
        Self{
            global: ZGlobalConfig{
                domain: Some("singlelogin.re".to_string()),
                username: "".to_string(),
                token: None,
                lang: None,
            },
            proxy: None,
        }
    }

    pub fn from_file(file: &mut File) -> Result<ZConfig, ZError>{
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        let config = toml::from_str(content.as_str())?;

        Ok(config)
    }

    pub fn write_to_file(config: &ZConfig, file: &mut File) -> Result<(), ZError>{
        let content = toml::to_string(config)?;
        let config_file = file;

        config_file.write_all(content.as_bytes())?;

        Ok(())
    }
}