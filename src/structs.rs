fn main() {
    let name = input("Bitte gib deinen Namen an: ");
    let job = input("Bitte gib deinen Beruf an: ");
    let pet = input("Bitte gib den Namen deines Tieres an: ");
    let person = Person {
        name,
        job,
        pet,
    };
    greet(person);
}

fn greet(person: Person) {
    println!("{}", make_hello(&*person));
    println!("{} zu sein ist voll cool!", person.job);
    println!("Wie bist du auf {} gekommen?", person.pet);
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

struct Person {
    name: String,
    job: String,
    pet: String,
}
