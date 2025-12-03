fn main() {

    let fintech_operations_city = OperationCities::Adana {num_of_emp: 12};

    give_some_info(fintech_operations_city); // Number of FinTech employees 12

    give_some_info(OperationCities::Antalya); // Hot summers...

    give_some_info(OperationCities::Kahramanmaras { head: "Ayhan Burada" }); // Head of Kahramanmaraş operations is Ayhan Burada.
}

#[derive(Debug)]
enum OperationCities {
    Ankara,
    Adana {num_of_emp: u8},
    Antalya,
    Kahramanmaras {head: &'static str},
}

fn give_some_info(oc: OperationCities) {

    match oc {
        OperationCities::Adana { num_of_emp } => println!("Number of FinTech employees {num_of_emp}"),
        OperationCities::Ankara => println!("Capital!"),
        OperationCities::Antalya => println!("Hot summers..."),
        OperationCities::Kahramanmaras { head } => println!("Head of Kahramanmaraş operations is {head}.")
    }
}


