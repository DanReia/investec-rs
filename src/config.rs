use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub struct InvestecConfig {
    token: String,
}

/// Function to read the INVESTEC_PATH environment variable for the default home path and
/// return a PathBuf of the location of the .investecrc configuration file.
/// 
/// # Panics
///
/// Panics if INVESTEC_PATH is not set
pub fn read_path() -> PathBuf {

    let path = match std::env::var("INVESTEC_PATH") {
        Ok(x) => x,
        Err(_) => panic!("INVESTEC_PATH not set"),
    };
    PathBuf::from(path).join(".investecrc")
}

pub fn create_if_not_exist(home: PathBuf) {
    if home.exists() == false {
        let config = InvestecConfig {
            token: String::from(""),
        };
        let config_string = match serde_json::to_string_pretty(&config) {
            Ok(x) => x,
            Err(_) => panic!("Could not convert default config to pretty string"),
        };
        match std::fs::write(&home, config_string) {
            Ok(_) => println!("Created .investecrc at: {:#?}", home.into_os_string()),
            Err(_) => panic!(
                "Failed to create home directory at: {:#?}",
                home.into_os_string()
            ),
        };
    } else {
        println!("Already exists: {:#?}", home.into_os_string());
    }
}

pub fn read_config(home: PathBuf) -> InvestecConfig {
    let file_string = match std::fs::read_to_string(home) {
        Ok(x) => x,
        Err(_) => panic!(
            "Failed to read .investecrc, ensure it exists and that json is formatted correctly"
        ),
    };

    let config: InvestecConfig = match serde_json::from_str(&file_string) {
        Ok(x) => x,
        Err(_) => panic!("Failed to deserialize json from .investecrc, check json syntax"),
    };
    config
}
