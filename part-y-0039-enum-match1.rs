fn main() {

    let fintech_operations_city = OperationCities::Adana;

    println!("Our FinTech Manager is: {:?}", find_managers(fintech_operations_city));
}

#[derive(Debug)]
enum OperationCities {
    Ankara,
    Adana,
    Antalya,
    Kahramanmaras
}

fn find_managers(pc: OperationCities) -> &'static str {

    match pc {
        OperationCities::Adana => "Mustafa Buyukdereli",
        OperationCities::Ankara => "Ayhan Bilir",
        OperationCities::Antalya => "Kutluk Kagan",
        OperationCities::Kahramanmaras => "Mukan Göktürk"
    }
}



