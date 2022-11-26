fn main() {
    let name = input("Bitte gib deinen Namen an: ");
    let job = input("Bitte gib deinen Beruf an: ");
    let pet = input("Bitte gib den Namen deines Tieres an: ");
    let person = Person {
        name,
        job,
        pet,
    };
    person.greet();
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

impl Person {

    // wir nehmen nur eine referenz, weil wir ja die Person vielleicht nocheinmal ansprechen wollen, nachdem wir sie gegrüßt haben.
    fn greet(&self) {
        println!("{}", make_hello(&*self.name));
        println!("{} zu sein ist voll cool!", &*self.job);
        println!("Wie bist du auf {} gekommen?", &*self.pet);
    }

}
