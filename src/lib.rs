use std::collections::HashMap;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("data store disconnected")]
    NoKeyGiven,
}

/// # Examples
///
/// configuration lines must contain a ':' and use a space as a delimiter
/// ```
/// let config = crate::rakh::interpret("deno_is_uncool:false\nname_of_funny_crab:Ferris").unwrap();
/// assert_eq!(config.get("name_of_funny_crab").unwrap().eq(&"Ferris"), true);
/// ```
pub fn interpret(code: &str) -> Result<HashMap<&str, &str>, Error> {
    let lines = code.split('\n');
    let mut hashmap: HashMap<&str, &str> = HashMap::new();

    for line in lines {
        let key_value = line.split_once(':');

        if let Some(key_value) = key_value {
            if key_value.0.is_empty() {
                return Err(Error::NoKeyGiven);
            }
            hashmap.insert(key_value.0, key_value.1);
        }
    }

    Ok(hashmap)
}

#[cfg(test)]
mod tests {
    use crate::interpret;

    #[test]
    fn parsing_works() {
        let config =
            interpret("rust_is_awesome:true\ndeno_is_uncool:false\nname_of_funny_crab:Ferris")
                .unwrap();

        assert_eq!(config.get("rust_is_awesome").unwrap().eq(&"true"), true);
        assert_eq!(config.get("deno_is_uncool").unwrap().eq(&"false"), true);
        assert_eq!(
            config.get("name_of_funny_crab").unwrap().eq(&"Ferris"),
            true
        );
    }
}
