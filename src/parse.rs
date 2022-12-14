fn main() {
    let name = input("Bitte gib deinen Namen an: ");
    let job = input("Bitte gib deinen Beruf an: ");
    let pet = input("Bitte gib den Namen deines Tieres an: ");

                                                                  // | Hier kann jeder Typ angegeben werden, der geparst werden kann
    let age = input("Bitte gib dein Alter an: ").parse::<u16>().expect("Bitte gib dein alter als ganze Zahl an.");

    let person = Person {
        name,
        job,
        pet,
        age,
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
    age: u16,
}

impl Person {

    // wir nehmen nur eine referenz, weil wir ja die Person vielleicht nocheinmal ansprechen wollen, nachdem wir sie gegrüßt haben.
    fn greet(&self) {
        println!("{}", make_hello(&*self.name));
        println!("Mit {} {} zu sein ist voll cool!", self.age, &*self.job); // wir können einfach 2 Sachen hintereinander ausgeben
        println!("Wie bist du auf {} gekommen?", &*self.pet);
    }

}
