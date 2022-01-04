# Rakh! A dead simple configuration language.
No seriously, it's simple. With only 57 lines of code, it's one of the tiniest configuration languages there is.

# Show me some examples then!
```
key:value
x:5
order_reached:true
rust_is_awesome:true
```

# How to use it?
Rakh is a Rust crate and it just has a *single* function -- `interpret()`. It returns a `Result<HashMap<String, String>, Error>`. You just pass Rakh code into it. Like this -
```rs
use rakh;

fn main() {
	let config = rakh::interpret("rust_is_awesome:true").unwrap();

	println!("{}", config.get("rust_is_awesome")); // true
}
```

# LICENSE
Rakh is under the very permissive Blue Oak 1.0.0 license, take a look at `LICENSE.md` for more information.
