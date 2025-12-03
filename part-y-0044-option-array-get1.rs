fn main() {

    let important_years: [u16; 6] = [1993, 1997, 2000, 2008, 2018, 2020];

    let year1 = important_years.get(3);

    let year2 = important_years.get(47);

    println!("{year1:?}, {year2:?}") // Some(2008), None
}

