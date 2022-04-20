#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
    let choosen_pet = choose_pet("Cat".to_string());
    let winner_pet = Pet::Cat;

    match choosen_pet {
        Pet::Dog => { println!("You have a nice dog!")},
        Pet::Cat => { println!("You have a cute cat!")},
        Pet::Ditto => { println!("You have a little!")}
    }

    if choosen_pet == winner_pet {
        println!("You win!");
    } else {
        println!("You lose!");
    }
}
