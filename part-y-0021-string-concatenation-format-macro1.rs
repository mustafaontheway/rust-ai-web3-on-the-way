fn main() {

    let first_name = String::from("Mustafa");

    let last_name = "Büyükdereli".to_string();

    let full_name = first_name + " " + &last_name;

    println!("{full_name}"); // Mustafa Büyükdereli

    //println!("{first_name}"); // Error => borrow of moved value: `first_name`

}

