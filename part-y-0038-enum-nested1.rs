fn main() {

    let most_sold = Products::Carpet(ProductionCities::Antalya);

    println!("{:?}", most_sold); // Carpet(Antalya)
}

#[derive(Debug)]
enum Products {

    Fabric(ProductionCities),
    Carpet(ProductionCities),
    Pillow(ProductionCities),
    Curtain(ProductionCities)
}

#[derive(Debug)]
enum ProductionCities {
    Ankara,
    Adana,
    Antalya,
    Kahramanmaras
}
