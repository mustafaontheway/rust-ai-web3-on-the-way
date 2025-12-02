fn main() {

    let izmir_operations = OperationCities::Izmir("Ayhan Bilir".to_string(), 24);

    println!("{:?}", izmir_operations) // Izmir("Ayhan Bilir", 24)

}

// let's keep head person name and number of employees
#[derive(Debug)]
enum OperationCities {

    Ankara(String, u8),
    Istanbul(String, u8),
    Izmir(String, u8),
}




