use std::collections::HashMap;

fn main() {

    let mut salary_by_id: HashMap<&'static str, u16> = HashMap::new();

    salary_by_id.insert("au004296", 46_000);

    salary_by_id.insert("mab007598", 37_000);

    println!("{salary_by_id:?}"); // {"mab007598": 37000, "au004296": 46000}

}

