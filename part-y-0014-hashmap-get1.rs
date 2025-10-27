use std::collections::HashMap;

fn main() {

    let mut employees: HashMap<&str, &str> = HashMap::new();

    let person1_id = "ab006587".to_string();

    let person1_dep = "Finance".to_string();

    employees.insert(&person1_id, &person1_dep);

    employees.insert("ku009878", "HR");

    //let emp1 = employees["zr009664"]; // panic! if the key doesn't exist

    let emp1 = employees.get("zr009664");

    println!("{emp1:?}") // None

}

