use std::collections::HashMap;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("data store disconnected")]
    NoKeyGiven,
}

#[derive(Debug,PartialEq)]
pub enum Types {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Array(Vec<Types>),
    Object(HashMap<String, Types>),
}


pub fn interpret(code: &str) ->  Result<HashMap<&str, Types>, Error> {
    let mut hashmap: HashMap<&str,Types> = HashMap::new();
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
                    if new_value.is_ok() {
                        hashmap.insert(key, Types::Integer(new_value.unwrap()));
                    } else {
                        let float_value = value.parse::<f64>();
                        if float_value.is_ok() {
                            hashmap.insert(key, Types::Float(float_value.unwrap()));
                        } else {
                            let bool_value = value.parse::<bool>();
                            if bool_value.is_ok() {
                                hashmap.insert(key, Types::Boolean(bool_value.unwrap()));
                            }
                        }
                    }
                }
            },
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
        let config = interpret("name:\"Rust\"\ndata:15\ntest:true").unwrap();
        assert_eq!(config.get("name").unwrap(), &Types::String("Rust".to_string()));
        assert_eq!(config.get("data").unwrap(), &Types::Integer(15));
        assert_eq!(config.get("test").unwrap(), &Types::Boolean(true));
    }
}
