use std::collections::HashMap;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("No Key Given")]
    NoKeyGiven,
}

#[derive(Debug, PartialEq)]
pub enum Types {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Array(Vec<Types>),
    Object(HashMap<String, Types>),
}

pub fn interpret(code: &str) -> Result<HashMap<&str, Types>, Error> {
    let mut hashmap: HashMap<&str, Types> = HashMap::new();
    for line in code.lines() {
        if line.is_empty() {
            continue;
        };
        match line.split_once(':') {
            Some((key, value)) => {
                if key.is_empty() {
                    panic!("No key given");
                }
                if value.starts_with('"') && value.ends_with('"') {
                    hashmap.insert(key, Types::String(value.to_string().replace("\"", "")));
                } else {
                    let new_value = value.parse::<i64>();
                    if let Ok(nv) = new_value {
                        hashmap.insert(key, Types::Integer(nv));
                    } else {
                        let float_value = value.parse::<f64>();
                        if let Ok(nv) = float_value {
                            hashmap.insert(key, Types::Float(nv));
                        } else {
                            let bool_value = value.parse::<bool>();
                            if let Ok(nv) = bool_value {
                                hashmap.insert(key, Types::Boolean(nv));
                            }
                        }
                    }
                }
            }
            None => {
                panic!("No key given");
            }
        };
    }
    Ok(hashmap)
}

#[cfg(test)]
mod tests {
    use crate::interpret;
    use crate::Types;

    #[test]
    fn parsing_works() {
        let config =
            interpret("hello:\"world\"\nrust_is_awesome:true\nrust_version:1.58\nrust_is_number:1\ndeno_is_uncool:false\nname_of_cute_crab:\"Ferris!\"")
                .unwrap();
        assert_eq!(
            config.get("rust_is_awesome").unwrap(),
            &Types::Boolean(true)
        );
        assert_eq!(config.get("rust_version").unwrap(), &Types::Float(1.58));
        assert_eq!(config.get("rust_is_number").unwrap(), &Types::Integer(1));
        assert_eq!(
            config.get("deno_is_uncool").unwrap(),
            &Types::Boolean(false)
        );
        assert_eq!(
            config.get("name_of_cute_crab").unwrap(),
            &Types::String("Ferris!".to_string())
        );
        assert_eq!(
            config.get("hello").unwrap(),
            &Types::String("world".to_string())
        );
    }
}
