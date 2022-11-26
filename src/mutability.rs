fn main() {
    println!("{}", make_hello(String::from("Gustav")));
}

fn make_hello(person: String) -> String {
    let mut hello = String::from("Hello ");
    hello.push_str(person.as_str());
    hello // ist das Gleiche, wie:
    // return hello;
}
