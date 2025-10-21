fn main() {

    let mut some_names: Vec<&str> = Vec::new();

    some_names.push("ayhan");
    some_names.push("hakan");
    some_names.push("aygün");

    println!("Names: {:?}", some_names); // Names: ["ayhan", "hakan", "aygün"]

    let last_name = some_names.pop(); 

    println!("Last name: {:?}", last_name); // Option! => Last name: Some("aygün") 

    println!("Names: {:?}", some_names); // Names: ["ayhan", "hakan"]

    some_names.pop();

    some_names.pop();

    let any_name = some_names.pop();

    println!("Any name: {:?}", any_name); // Option! => Any name: None

}

