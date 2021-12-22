mod lib;
fn main() {
    dbg!(lib::interpret("x:5".to_string()).unwrap());
}