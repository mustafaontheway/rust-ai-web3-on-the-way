fn main() {
    
    OperationCities::Ankara.give_some_info();
    OperationCities::Antalya.give_some_info();
    OperationCities::Adana { num_of_emp: 12 }.give_some_info();
    OperationCities::Kahramanmaras { head: "Ayhan Burada" }.give_some_info();
}

#[derive(Debug)]
enum OperationCities {
    Ankara,
    Adana {num_of_emp: u8},
    Antalya,
    Kahramanmaras {head: &'static str},
}

impl OperationCities {
    
    fn give_some_info(&self) {

        match self {
            OperationCities::Adana { num_of_emp } => println!("Number of FinTech employees {num_of_emp}"),
            OperationCities::Ankara => println!("Capital!"),
            OperationCities::Antalya => println!("Hot summers..."),
            OperationCities::Kahramanmaras { head } => println!("Head of Kahramanmaraş operations is {head}.")
        }
    }
}

// fn give_some_info(oc: OperationCities) {

//     match oc {
//         OperationCities::Adana { num_of_emp } => println!("Number of FinTech employees {num_of_emp}"),
//         OperationCities::Ankara => println!("Capital!"),
//         OperationCities::Antalya => println!("Hot summers..."),
//         OperationCities::Kahramanmaras { head } => println!("Head of Kahramanmaraş operations is {head}.")
//     }
// }


