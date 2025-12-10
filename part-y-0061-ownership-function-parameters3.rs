fn main() {

    let person_kagan = "Kağan Öztürk".to_string();

    println!("{:?}", greeting(&person_kagan)); // "Kağan Öztürk, welcome! Nice to meet you:)"

    println!("{:?}", person_kagan) // "Kağan Öztürk"
}

fn greeting(name: &String) -> String {

    let mut name_clone_for_greeting = name.clone();

    name_clone_for_greeting.push_str(", welcome! Nice to meet you:)");

    name_clone_for_greeting
}
