use std::{collections::HashMap, path::PathBuf};
use thiserror::Error;

#[derive(Clone, Debug, PartialEq)]
pub struct Config {
    internal: HashMap<String, String>,
    pub path: PathBuf,
}

impl Config {
    /// Create a new system config.
    pub fn new<T: AsRef<str>>(name: T) -> Result<Self, Error> {
        let mut path = match dirs::config_dir() {
            Some(data) => data,
            None => return Err(Error::Other("Couldn't get config path".to_string())),
        };

        path.push(format!("{}.yaml", name.as_ref()));

        let contents = match fstream::read_text(path.clone()) {
            Some(data) => data,
            None => {
                let temp: HashMap<String, String> = HashMap::new();

                let deserialized = serde_yaml::to_string(&temp).unwrap();
        
                match fstream::write_text(path.clone(), deserialized.clone(), true) {
                    Some(_) => (),
                    None => return Err(Error::Io(format!("Couldn't write text to {}", path.to_str().unwrap()))),
                };

                deserialized
            }
        };

        let internal: HashMap<String, String> = match serde_yaml::from_str(contents.as_str()) {
            Ok(data) => data,
            Err(error) => return Err(Error::Parse(error.to_string())),
        };

        let myself = Self{
            internal,
            path,
        };

        return Ok(myself);
    }

    /// Update the config from disk.
    pub fn read(&mut self) -> Result<(), Error> {       
        let contents = match fstream::read_text(self.path.clone()) {
            Some(data) => data,
            None => return Err(Error::Io("Couldn't read text".to_string())),
        };

        let internal: HashMap<String, String> = match serde_yaml::from_str(contents.as_str()) {
            Ok(data) => data,
            Err(error) => return Err(Error::Parse(error.to_string())),
        };

        self.internal = internal;

        Ok(())
    }

    /// Update the disk from the config.
    pub fn write(&self) -> Result<(), Error> {
        let deserialized = serde_yaml::to_string(&self.clone().internal).unwrap();

        match fstream::write_text(self.clone().path, deserialized, true) {
            Some(_) => (),
            None => return Err(Error::Io(format!("Couldn't write text to {}", self.path.to_str().unwrap()))),
        };

        Ok(())
    }

    /// Insert a key-value pair into the config.
    pub fn insert<T: AsRef<str>>(&mut self, key: T, value: T) {
        self.internal.insert(key.as_ref().to_string(), value.as_ref().to_string());
    }

    /// Get a value for a key.
    pub fn get<T: AsRef<str>>(&self, query: T) -> Option<String> {
        let res = match self.internal.get(&query.as_ref().to_string()) {
            Some(data) => data,
            None => return None,
        };

        return Some(res.to_string());
    }

    
    /// Clear all data in the config.
    pub fn clear(&mut self) {
        self.internal.clear();
    }

    /// Insert a key-value pair into the config and write to disk.
    pub fn write_insert<T: AsRef<str>>(&mut self, key: T, value: T) -> Result<(), Error> {
        self.insert(key, value);

        return self.write();
    }

    /// Read the system config and query the config.
    pub fn read_get<T: AsRef<str>>(&mut self, query: T) -> Result<Option<String>, Error> {
        match self.read() {
            Ok(_) => (),
            Err(error) => return Err(Error::Path(error.to_string())),
        }

        return Ok(self.get(query));
    }

    /// Clear all data in the config and write to disk.
    pub fn write_clear(&mut self) -> Result<(), Error> {
        self.clear();

        return self.write();
    }
}

#[derive(Error, Debug, Clone, PartialEq, Hash)]
pub enum Error {
    #[error("Other Error: {0}")]
    Other(String),
    #[error("Error getting path: {0}")]
    Path(String),
    #[error("Error getting file data: {0}")]
    Io(String),
    #[error("Error parsing file: {0}")]
    Parse(String),
}