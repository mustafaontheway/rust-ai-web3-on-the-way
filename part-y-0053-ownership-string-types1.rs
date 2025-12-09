fn main() {

    // read-only data, string literals

    let name = "Mustafa"; 

    let city: &'static str = "Antalya";

    // heap data, String type

    let my_name = String::from(name);

    let new_city = "Ankara".to_string();

    let empty_name = String::new();
}

