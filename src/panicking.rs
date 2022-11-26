fn main() {
    let name = input("Bitte gib deinen Namen an: ");
    let job = input("Bitte gib deinen Beruf an: ");
    let pet_name = input("Bitte gib den Namen deines Tieres an: ");
    let pet_type = input("Bitte gib die Art deines Tieres an: ");
    let age = input("Bitte gib dein Alter an: ").parse::<u16>().expect("Bitte gib dein alter als ganze Zahl an.");

    let person = Person {
        name,
        job,
        pet: Pet {
            name: pet_name,
            ty: match &*pet_type {
                "cat" | "Cat" => PetType::Cat,
                "dog" | "Dog" => PetType::Dog,
                "mouse" | "Mouse" => PetType::Mouse,
                &_ => panic!("Bitte gib ein Gültiges Tier an: \"Mouse\", \"Dog\" oder \"Cat\"!"),
            },
        },
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
    pet: Pet,
    age: u16,
}

impl Person {

    fn greet(&self) {
        println!("{}", make_hello(&*self.name));
        println!("Mit {} {} zu sein ist voll cool!", self.age, &*self.job);
        println!("Wie bist du auf {} gekommen?", &*self.pet.name);
    }

}

struct Pet {
    name: String,
    ty: PetType,
}

enum PetType {
    Cat,
    Dog,
    Mouse,
}
