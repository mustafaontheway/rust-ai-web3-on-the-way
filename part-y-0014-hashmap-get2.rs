use std::collections::HashMap;

fn main() {

    let mut employees: HashMap<&str, &str> = HashMap::new();

    let person1_id = "ab006587".to_string();

    let person1_dep = "Finance".to_string();

    employees.insert(&person1_id, &person1_dep);

    employees.insert("ku009878", "HR");

    let emp1 = employees.get("zr009664").copied().unwrap_or("The key doesn't exist"); 

    println!("{emp1:?}"); // // "The key doesn't exist"

    let emp2 = employees.get("ku009878");

    println!("{emp2:?}"); // Some("HR")

    println!("{:?}", emp2.copied().unwrap_or("The key doesn't exist")); // "HR"  

}

