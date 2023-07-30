use bindumprs::binary::Object;
fn main() {
    let path = std::env::args().nth(1).expect("Requires one arg");
    let obj = Object::load(path);
    match obj {
        Ok(obj) => println!("{}", obj),
        Err(e) => println!("Error {}", e),
    }
}
