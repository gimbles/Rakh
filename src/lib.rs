use std::collections::HashMap;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("data store disconnected")]
    NoKeyGiven,
}

pub fn interpret(code: &str) -> Result<HashMap<&str, &str>, Error> {
    let lines = code.split('\n');
    let mut hashmap: HashMap<&str, &str> = HashMap::new();

    for line in lines {
        let key_value = line.rsplit_once(':').unwrap();
        let key = key_value.0;
        let value = key_value.1;

        if key.is_empty() {
            return Err(Error::NoKeyGiven);
        }

        hashmap.insert(key, value);
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

    #[test]
    fn colon_in_key() {
        let config =
            interpret("this:key:has:a:colon:in:its:keyname:THIS IS THE ACTUAL VALUE OF THE KEY")
                .unwrap();

        assert_eq!(
            config.get("this:key:has:a:colon:in:its:keyname").unwrap(),
            &"THIS IS THE ACTUAL VALUE OF THE KEY"
        );
    }
}
