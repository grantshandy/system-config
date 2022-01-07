//! # system-config
//! A Rust library for storing application properties on disk in any context and between restarts.
//!
//! # Example
//! Add key-value pairs to the config and write it to disk...
//! ```rust
//! let mut config = Config::new("system-config-example").unwrap();
//!
//! config.insert("key1", "value1");
//! config.insert("key2", "value2");
//!
//! config.write().unwrap();
//! ```
//!
//! Then retrieve the information at any other time, even after the application is restarted or in different contexts.
//! ```rust
//! let config = Config::new("system-config-example").unwrap();
//!
//! let key1 = config.get("key1").unwrap();
//! let key2 = config.get("key2").unwrap();
//!
//! println!("key1: {}", key1);
//! println!("key2: {}", key2);
//! ```

use std::{collections::HashMap, fs, path::PathBuf};
use anyhow::{Result, anyhow};

#[derive(Clone, Debug, PartialEq)]
pub struct Config {
    internal: HashMap<String, String>,
    pub path: PathBuf,
}

impl Config {
    /// Create a new system config.
    pub fn new<T: AsRef<str>>(name: T) -> Result<Self> {
        let mut path = match dirs::config_dir() {
            Some(data) => data,
            None => return Err(anyhow!("Couldn't get config path")),
        };

        path.push(format!("{}.yaml", name.as_ref()));

        let contents = match fs::read_to_string(&path) {
            Ok(data) => data,
            Err(_error) => {
                let temp: HashMap<String, String> = HashMap::new();

                let deserialized = serde_yaml::to_string(&temp)?;

                fs::write(&path, deserialized.as_bytes())?;

                deserialized
            }
        };

        let internal: HashMap<String, String> = serde_yaml::from_str(contents.as_str())?;

        let myself = Self{
            internal,
            path,
        };

        Ok(myself)
    }

    /// Clear a config by name and sync it with the disk.
    pub fn write_clear_by_name<T: AsRef<str>>(name: T) -> Result<()> {
        let mut config = Self::new(name)?;
        
        config.write_clear()
    }

    /// Update the config from disk.
    pub fn read(&mut self) -> Result<()> {       
        let contents = fs::read_to_string(&self.path)?;

        let internal: HashMap<String, String> = serde_yaml::from_str(contents.as_str())?;

        self.internal = internal;

        Ok(())
    }

    /// Update the disk from the config.
    pub fn write(&self) -> Result<()> {
        let deserialized = serde_yaml::to_string(&self.internal)?;

        fs::write(&self.path, deserialized.as_bytes())?;

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

        Some(res.to_string())
    }

    /// Clear all data in the config.
    pub fn clear(&mut self) {
        self.internal.clear();
    }

    /// Insert a key-value pair into the config and write to disk.
    pub fn write_insert<T: AsRef<str>>(&mut self, key: T, value: T) -> Result<()> {
        self.insert(key, value);

        self.write()
    }

    /// Read the system config and get a value for a key.
    pub fn read_get<T: AsRef<str>>(&mut self, query: T) -> Result<Option<String>> {
        self.read()?;

        Ok(self.get(query))
    }

    /// Clear all data in the config and write to disk.
    pub fn write_clear(&mut self) -> Result<()> {
        self.clear();

        self.write()
    }
}