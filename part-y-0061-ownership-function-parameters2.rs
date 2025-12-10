fn main() {

    let her_name = String::from("Merve Günay");

    print_name(&her_name); // Your name is Merve Günay!

    println!("{her_name}") // Merve Günay
}

fn print_name(name: &String) {

    println!("Your name is {name}!")
}
