fn main() {

    let city1 = String::from("İzmir");
    let city2 = String::from("Ankara");
    let city3 = String::from("İstanbul");

    let big_cities = vec![city1, city2, city3];

    println!("{big_cities:?}");

    //println!("İzmir: {city1}"); // Error => borrow of moved value: `city1`
}

