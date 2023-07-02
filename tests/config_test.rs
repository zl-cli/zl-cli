use std::path::Path;

use zl_cli;

#[test]
fn test_config_read() {
    let test_config = zl_cli::config::ZConfig::from_file(Path::new("./config/test.toml")).unwrap();
    
}