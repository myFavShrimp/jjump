use std::{
    collections::HashMap,
    fs::{File, OpenOptions},
    io::{BufReader, BufWriter, Write},
    path::PathBuf,
};

use serde::{Deserialize, Serialize};
use thiserror::Error;
use xdg::BaseDirectories;

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Config {
    pub portals: HashMap<PathBuf, Vec<String>>,
}

const CRATE_NAME: &str = env!("CARGO_PKG_NAME");

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("config dir error")]
    XdgBaseDirectoriesError(#[from] xdg::BaseDirectoriesError),
    #[error("storage error")]
    IoError(#[from] std::io::Error),
    #[error("config format error")]
    SerdeYamlError(#[from] serde_yaml::Error),
}

fn get_config_file() -> Result<PathBuf, ConfigError> {
    let config_file = format!("{}.yaml", CRATE_NAME);
    let xdg_dirs = BaseDirectories::with_prefix(CRATE_NAME)?;
    let path = match xdg_dirs.find_config_file(&config_file) {
        Some(path) => path,
        None => xdg_dirs.place_config_file(&config_file)?,
    };
    Ok(path)
}

fn file_reader(file_path: &PathBuf) -> Result<BufReader<File>, std::io::Error> {
    File::open(file_path).map(|file| BufReader::new(file))
}

fn file_writer(file_path: &PathBuf) -> Result<BufWriter<File>, std::io::Error> {
    OpenOptions::new()
        .write(true)
        .open(file_path)
        .map(|file| BufWriter::new(file))
}

pub fn write(config: Config) -> Result<(), ConfigError> {
    let path = get_config_file()?;
    let writer = file_writer(&path)?;
    Ok(serde_yaml::to_writer(writer, &config)?)
}

fn create_default_config(file_path: &PathBuf) -> Result<(), ConfigError> {
    let mut file = File::create(file_path)?;
    file.write_all(serde_yaml::to_string(&Config::default())?.as_bytes())?;
    Ok(())
}

pub fn create_and_read() -> Result<Config, ConfigError> {
    let path = get_config_file()?;
    if !path.exists() {
        create_default_config(&path)?;
    }

    let reader = file_reader(&path)?;
    Ok(serde_yaml::from_reader(reader)?)
}
