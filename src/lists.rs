fn main() {
    let names = input("Bitte gib deine Namen an (Vorname1, (Vorname2, ...) Nachname): ");
    let names = {
        names.split(" ").collect::<Vec<String>>()
    };
    let job = input("Bitte gib deinen Beruf an: ");
    let pet_name = input("Bitte gib den Namen deines Tieres an, oder \"-\" für kein Tier: ");
    let pet = parse_pet(pet_name);

    // | Hier kann jeder Typ angegeben werden, der geparst werden kann
    let age = input("Bitte gib dein Alter an: ").parse::<u16>().expect("Bitte gib dein alter als ganze Zahl an.");

    let person = Person {
        names,
        job,
        pet,
        age,
    };
    person.greet();
}

fn parse_pet(pet_name: String) -> Option<Pet> {
    if &*pet_name != "-" {
        let pet_type = input("Bitte gib die Art deines Tieres an: ");
        let ty = match &*pet_type {
            "cat" | "Cat" => PetType::Cat,
            "dog" | "Dog" => {
                let breed = input("Bitte gib die Rasse deines Hundes an: ");
                let dog_breed = DogBreed::parse(breed);
                PetType::Dog(dog_breed)
            },
            "mouse" | "Mouse" => PetType::Mouse,
            &_ => panic!("Bitte gib ein Gültiges Tier an: \"Mouse\", \"Dog\" oder \"Cat\"!"),
        };

        Some(Pet {
            name: pet_name,
            ty,
        })
    } else {
        None
    }
}

fn parse_breed() -> Option<DogBreed> {
    let breed = input("Bitte gib die Rasse deines Hundes an, oder `?`, falls du sie nicht weißt: ");
    match &*breed {
        "?" => None,
        "Terrier" => Some(DogBreed::Terrier),
        "Retriever" => Some(DogBreed::Retriever),
        "Bulldog" => Some(DogBreed::Bulldog),
        &_ => panic!("Bitte gib ein Gültiges Tier an: \"?\", \"Terrier\", \"Retriever\" oder \"Bulldog\"!"),
    }
}

fn make_hello(person: &Vec<String>) -> String {
    let mut hello = String::from("Hello");
    for name in person {
        hello.push_str(" ");
        hello.push_str(name);
    }
    hello
}

fn input(prompt: &str) -> String {
    let mut input = String::new();
    print!("{}", prompt);
    std::io::stdin().read_line(&mut line).expect("Ein Fehler ist aufgetreten!");
    input
}

struct Person {
    names: Vec<String>,
    job: String,
    pet: Option<Pet>,
    age: u16,
}

impl Person {

    fn greet(&self) {
        println!("{}", make_hello(&self.names));
        println!("Mit {} {} zu sein ist voll cool!", self.age, &self.job);
        if let Some(pet) = &self.pet {
            println!("Wie bist du auf {} gekommen?", pet.name);
        }
    }

}

struct Pet {
    name: String,
    ty: PetType,
}

enum PetType {
    Cat,
    Dog(Option<DogBreed>),
    Mouse,
    // Keine `None` variante mehr, weil wir ja eine `Option` haben.
}

enum DogBreed {
    Terrier,
    Retriever,
    Bulldog,
}

impl DogBreed {

    fn as_string(&self) -> &str {
        match self {
            DogBreed::Terrier => "Terrier",
            DogBreed::Retriever => "Retriever",
            DogBreed::Bulldog => "Bulldog",
        }
    }

    fn parse(breed: String) -> Option<DogBreed> {
        match &*breed {
            "?" => None,
            "Terrier" => Some(DogBreed::Terrier),
            "Retriever" => Some(DogBreed::Retriever),
            "Bulldog" => Some(DogBreed::Bulldog),
            &_ => panic!("Bitte gib ein Gültiges Tier an: \"?\", \"Terrier\", \"Retriever\" oder \"Bulldog\"!"),
        }
    }

}
