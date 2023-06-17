use std::fs::read_to_string;
use std::io::Write;
use std::path::Path;
use toml::de::Error;

mod config;

pub type ZConfig = config::ZConfig;

impl ZConfig{
    fn from_file(file: &Path) -> Result<ZConfig, Error>{
        let content = read_to_string(file)?.as_str();

        toml::from_str(content)
    }

    fn write_to_file(config: &ZConfig, file: &Path) -> Result<(), String>{
        let content = match toml::to_string(config) {
            Ok(c) => c,
            Err(e) => {return Err(e.to_string());}
        };
        let mut config_file = match std::fs::File::open(file) {
            Ok(f) => f,
            Err(e) => {return Err(e.to_string());}
        };
        config_file.write_all(content.as_bytes());
        Ok(())
    }
}