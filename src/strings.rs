fn main() {
    println!("{}", make_hello("Gustav"));
}

fn make_hello(person: &str) -> String {
    let mut hello = String::from("Hello ");
    hello.push_str(person);
    hello
}
