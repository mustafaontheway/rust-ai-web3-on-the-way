fn main() {

    let mayor_ankara = Mayors {name: String::from("Mansur Yavaş"), vote_rate: 0.57}; 

    let ankara = Cities::Ankara(mayor_ankara);

    println!("{:?}", ankara); // Ankara(Mayors { name: "Mansur Yavaş", vote_rate: 0.57 })
}

#[derive(Debug)]
enum Cities {

    Ankara(Mayors),
    Istanbul(Mayors),
    Izmir {name: String, vote_rate: f32}, // interesting syntax. right?
}

#[derive(Debug)]
struct Mayors {

    name: String,
    vote_rate: f32
}




