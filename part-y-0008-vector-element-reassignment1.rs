fn main() {

    let mut names = vec!["Ayhan", "Bengü", "Bumin"];

    names[1] = "Bilge Kağan";

    println!("Names: {names:?}") // Names: ["Ayhan", "Bilge Kağan", "Bumin"]

    let mut years: Vec<u16> = vec![2000, 2001, 2004];

    years[0] = 2025;
}

