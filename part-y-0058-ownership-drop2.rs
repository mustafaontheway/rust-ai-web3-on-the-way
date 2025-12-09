fn main() {

    let cities: Vec<&str> = vec!["adana", "ankara", "antalya", "izmir", "istanbul", "kayseri", "kahramanmaraş", "ağrı"];

    let birth_city = cities.get(6);

    println!("His birth city is {:?}", birth_city.unwrap());

    drop(cities);

    let _birth_year: u16 = 1947;
}

