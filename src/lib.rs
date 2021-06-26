use std::{collections::HashMap, path::PathBuf};

mod error;

pub use error::Error;

#[derive(Clone)]
pub struct Config {
    internal: HashMap<String, String>,
    pub path: PathBuf,
}

impl Config {
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
                    None => return Err(Error::File(format!("Couldn't write text to {}", path.to_str().unwrap()))),
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

    pub fn read(&mut self) -> Result<(), Error> {       
        let contents = match fstream::read_text(self.path.clone()) {
            Some(data) => data,
            None => return Err(Error::File("Couldn't read text".to_string())),
        };

        let internal: HashMap<String, String> = match serde_yaml::from_str(contents.as_str()) {
            Ok(data) => data,
            Err(error) => return Err(Error::Parse(error.to_string())),
        };

        self.internal = internal;

        Ok(())
    }

    pub fn write(&self) -> Result<(), Error> {
        let deserialized = serde_yaml::to_string(&self.clone().internal).unwrap();

        match fstream::write_text(self.clone().path, deserialized, true) {
            Some(_) => (),
            None => return Err(Error::File(format!("Couldn't write text to {}", self.path.to_str().unwrap()))),
        };

        Ok(())
    }

    pub fn insert<T: AsRef<str>>(&mut self, key: T, value: T) {
        self.internal.insert(key.as_ref().to_string(), value.as_ref().to_string());
    }

    pub fn get<T: AsRef<str>>(&self, query: T) -> Option<String> {
        let res = match self.internal.get(&query.as_ref().to_string()) {
            Some(data) => data,
            None => return None,
        };

        return Some(res.to_string());
    }

    pub fn clear(&mut self) -> Result<(), Error> {
        self.internal.clear();

        let deserialized = serde_yaml::to_string(&self.internal).unwrap();

        match fstream::write_text(self.clone().path, deserialized, true) {
            Some(_) => (),
            None => return Err(Error::File(format!("Couldn't write text to {}", self.path.to_str().unwrap()))),
        };

        Ok(())
    }
}