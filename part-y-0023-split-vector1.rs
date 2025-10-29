fn main() {

    let cities = "Ankara, Izmir, Istanbul, Antalya, Adana";

    let cities_vec: Vec<&str> = cities.split(",").collect();

    println!("Cities vector: {cities_vec:?}") // Cities vector: ["Ankara", " Izmir", " Istanbul", " Antalya", " Adana"]

}
