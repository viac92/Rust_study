enum Pet {
    Dog,
    Cat,
    Ditto,
}

fn choose_pet(s: String) -> Pet {
    if s == "Dog" {
        Pet::Dog
    } else if s == "Cat" {
        Pet::Cat
    } else {
        Pet::Ditto
    }
}

fn main() {
    let choosen_pet = choose_pet("".to_string());

    match choosen_pet {
        Pet::Dog => { println!("You have a nice dog!")},
        Pet::Cat => { println!("You have a cute cat!")},
        Pet::Ditto => { println!("You have a little ditto!")}
    }
}
