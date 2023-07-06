use std::fs::File;
use std::path::{PathBuf};
use std::process::exit;
use std::str::FromStr;
use z_api::ZApi;
use crate::api::ZApiImpl;
use crate::config::ZConfig;

mod config;
mod zerror;
mod api;

fn main() {
   let (config, _config_file) = match init_config() {
       Some(value) => value,
       None => { exit(-1);}
   };

    let api = init_api(config);

}


fn init_config() -> Option<(ZConfig, File)> {
    let config_file_name = "zl-cli.toml";
    let user_dirs = directories::UserDirs::new();
    let config_path = match user_dirs {
        Some(user_dirs) => user_dirs.home_dir().to_path_buf(),
        None => PathBuf::from_str(".").unwrap()
    };
    let config_file_path = config_path.join(config_file_name);
    dbg!(&config_file_path);

    let mut config_file: File;

    if let Ok(file) = File::open(config_file_path.as_path()) {
        config_file = file;
    } else if let Ok(file) = File::create(config_file_path.as_path()){
        config_file = file;
    }else {
        eprintln!("Cannot create config file in {:?}", config_file_path.as_path());
        return None;
    }

    let config = match ZConfig::from_file(&mut config_file){
        Ok(config) => config,
        Err(e) => {
            eprintln!("Error '{}' occurs while loading config file from {:?}", e.to_string(), config_file_path.as_path());
            ZConfig::default()
        }
    };

   Some( (config, config_file) )
}


fn init_api( config: ZConfig) -> impl ZApi{
    ZApiImpl{ config }
}