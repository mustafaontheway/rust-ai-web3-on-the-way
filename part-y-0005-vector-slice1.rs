fn main() {

    let city1 = String::from("İzmir");
    let city2 = String::from("Ankara");
    let city3 = String::from("İstanbul");

    let big_cities = vec![city1, city2, city3, "Antalya".to_string(), "Bursa".to_string(), "Adana".to_string()];

    let last_third_cities = &big_cities[3..];

    let three_cities = &big_cities[1..=3]; // 3 is inclusive!

    println!("{last_third_cities:?}");

    println!("{three_cities:?}");
}

// ["Antalya", "Bursa", "Adana"]
// ["Ankara", "İstanbul", "Antalya"]
