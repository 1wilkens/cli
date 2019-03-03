use clap::ArgMatches;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

use std::{
    fs::{DirBuilder, File},
    io::{Read, Write},
    path::Path,
};

#[derive(Default, Serialize, Deserialize)]
pub(crate) struct Config {
    pub(crate) verbose: bool,
    pub(crate) debug: bool,
    pub(crate) base_url: String,
    pub(crate) email: String,
    pub(crate) password: String,
}

impl Config {
    pub fn from_args(args: &ArgMatches) -> Self {
        let mut cfg = Self::default();
        let default_path =
            ProjectDirs::from("", "", "kapitalist_cli").expect("Failed to find $HOME");
        let default_path = default_path.config_dir().join("config.toml");
        let path = if let Some(p) = args.value_of("config") {
            Path::new(p)
        } else {
            &default_path
        };

        let mut cfg_str = String::new();
        if !Path::exists(path) {
            let parent_dir = &path.parent().unwrap();
            // Create config directory
            if !Path::exists(parent_dir) {
                DirBuilder::new()
                    .recursive(true)
                    .create(parent_dir)
                    .expect("Failed to create config directory");
            }
            // Write default config file
            File::create(path)
                .expect("Failed to create default config file")
                .write_all(toml::to_string_pretty(&cfg).unwrap().as_bytes())
                .expect("Failed to write default config file");
        }
        File::open(path)
            .expect("Failed to open config file")
            .read_to_string(&mut cfg_str)
            .expect("Failed to read config file");
        cfg = toml::from_str(&cfg_str).expect("Invalid configuration file specified");

        cfg.verbose = cfg.verbose || args.is_present("verbose");
        cfg.debug = cfg.debug || args.is_present("debug");

        cfg
    }
}
