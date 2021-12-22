use std::collections::HashMap;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("data store disconnected")]
    NoKeyGiven,
}

pub fn interpret(code: String) -> Result<HashMap<String, String>, Error> {
    let lines = code.split('\n');
    let mut hashmap: HashMap<String, String> = HashMap::new();

    for line in lines {
        let key_value = line.split_once(':');

        if let Some(key_value) = key_value {
            if key_value.0.is_empty() {
                return Err(Error::NoKeyGiven);
            }
            hashmap.insert(key_value.0.to_string(), key_value.1.to_string());
        }
    }

    Ok(hashmap)
}
