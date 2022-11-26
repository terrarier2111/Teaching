fn main() {
    let input = input("Bitte gib deinen Namen an: ");
    println!("{}", make_hello(&*input));
}

fn make_hello(person: &str) -> String {
    let mut hello = String::from("Hello ");
    hello.push_str(person);
    hello
}

fn input(prompt: &str) -> String {
    let mut input = String::new();
    print!("{}", prompt);
    std::io::stdin().read_line(&mut line).expect("Ein Fehler ist aufgetreten!");
    input
}
