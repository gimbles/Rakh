use std::collections::HashMap;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("data store disconnected")]
    NoKeyGiven,
}

pub fn interpret(code: &str) -> Result<HashMap<&str, &str>, Error> {
    let mut hashmap: HashMap<&str, &str> = HashMap::new();

    for line in code.lines() {
        match line.split_once(':') {
            Some((key, value)) => {
                if key.is_empty() {
                    return Err(Error::NoKeyGiven);
                }

                hashmap.insert(key, value);
            }

            None => {
                return Err(Error::NoKeyGiven);
            }
        };
    }

    Ok(hashmap)
}

#[cfg(test)]
mod tests {
    use crate::interpret;

    #[test]
    fn parsing_works() {
        let config =
            interpret("rust_is_awesome:true\ndeno_is_uncool:false\nname_of_cute_crab:Ferris!")
                .unwrap();

        assert_eq!(config.get("rust_is_awesome").unwrap(), &"true");
        assert_eq!(config.get("deno_is_uncool").unwrap(), &"false");
        assert_eq!(config.get("name_of_cute_crab").unwrap(), &"Ferris!");
    }
}
