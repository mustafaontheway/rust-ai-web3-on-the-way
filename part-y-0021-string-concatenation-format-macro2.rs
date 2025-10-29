fn main() {

    let first_name = String::from("Mustafa");

    let last_name = "Büyükdereli".to_string();

    let full_name = format!("{first_name} {last_name}");

    println!("{full_name}");

    println!("{first_name}"); 

    let more_name = format!("{0} {1} {0}", first_name, last_name);

    println!("{more_name}"); 

}

// Mustafa Büyükdereli
// Mustafa
// Mustafa Büyükdereli Mustafa
