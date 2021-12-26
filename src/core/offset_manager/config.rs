// Basically taken from hazedumper

use serde::{Serialize, Deserialize};

use std::fs::File;

#[derive(Debug, Serialize, Deserialize)]
pub enum ConfigError {
    LoadingFromFile,
}

// This struct represents a signature.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Signature {
    // Signature name.
    pub name: String,

    // Signature pattern.
    pub pattern: String,

    // Module name.
    pub module: String,

    // Signature offsets for dereferencing.
    #[serde(default)]
    pub offsets: Vec<isize>,

    // Extra to be added to the result.
    #[serde(default)]
    pub extra: isize,

    // If true, subtract module base from result.
    #[serde(default)]
    pub relative: bool,

    // If true, read a u32 at the position and add it to the result.
    #[serde(default)]
    pub rip_relative: bool,

    // Offset to the rip relative.
    #[serde(default)]
    pub rip_offset: isize,
}

impl Default for Signature {
    fn default() -> Self {
        Signature {
            name: "".to_string(),
            pattern: "".to_string(),
            module: "".to_string(),
            offsets: vec![],
            extra: 0,
            relative: false,
            rip_relative: false,
            rip_offset: 0,
        }
    }
}

// This struct represents a netvar.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Netvar {
    // Netvar name.
    pub name: String,

    // Table name.
    pub table: String,

    // Prop name.
    pub prop: String,

    // Offset to be added to the result.
    #[serde(default)]
    pub offset: isize,
}

// This struct represents the config.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    // Executable target name.
    pub executable: String,

    // Output file names
    #[serde(default)]
    pub filename: String,

    // `Vec` containing the `Signature`s.
    #[serde(default)]
    pub signatures: Vec<Signature>,

    // `Vec` containing the `Netvar`s.
    #[serde(default)]
    pub netvars: Vec<Netvar>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            executable: "csgo.exe".to_string(),
            filename: "csgo".to_string(),
            signatures: vec![],
            netvars: vec![],
        }
    }
}

impl Config {
    pub fn load(path: &str) -> std::result::Result<Self, ConfigError> {
        let mut file_input = File::open(path).map_err(|_| ConfigError::LoadingFromFile)?;
        serde_json::from_reader(&mut file_input).map_err(|_| ConfigError::LoadingFromFile)
    }
}
