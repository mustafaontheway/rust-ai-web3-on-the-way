use std::collections::HashMap;

fn main() {

    let mut employees = HashMap::new();

    let person1_id = "ab006587".to_string();

    let person1_dep = "Finance".to_string();

    //employees.insert(person1_id, person1_dep);

    //println!("{:?}", person1_dep); // Error! => borrow of moved value: `person1_dep`

    employees.insert(&person1_id, &person1_dep);

    println!("{:?}", person1_dep); // "Finance"

    println!("{:?}", employees); // {"ab006587": "Finance"}

}

