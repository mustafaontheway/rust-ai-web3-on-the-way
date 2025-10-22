fn main() {

    // You can have either one mutable reference OR any number of immutable references, but not both at the same time

    let mut names: Vec<String> = vec!["Ayhan".to_string(), "Bengü".to_string(), "Bumin".to_string()];

    let last_name = &names[names.len() - 1];

    // names.push("Bilge".to_string()); // Error! => cannot borrow `names` as mutable because it is also borrowed as immutable mutable borrow occurs

    println!("Last name is: {last_name}"); // Last name is: Bumin

    names.push("Bilge".to_string());

    println!("Names are: {names:?}") // Names are: ["Ayhan", "Bengü", "Bumin", "Bilge"]
  
}

