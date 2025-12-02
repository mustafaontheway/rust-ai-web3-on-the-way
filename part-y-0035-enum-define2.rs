fn main() {

    let person_ayhan = People {name: "Ayhan Bilir".to_string(), city: Cities::Izmir};

    let operation_cities: [Cities; 3] = [Cities::Ankara, Cities::Kayseri, Cities::Antalya];
}

struct People {

    name: String,
    city: Cities
}

enum Cities {

    Ankara,
    Istanbul,
    Izmir,
    Antalya,
    Bursa,
    Adana,
    Kayseri
}




