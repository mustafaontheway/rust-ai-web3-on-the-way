fn main() {

    let important_years: [u16; 6] = [1993, 1997, 2000, 2008, 2018, 2020];

    let year1 = important_years.get(3);

    get_year(year1);

    let year2 = important_years.get(33);

    get_year(year2);
}

fn get_year(year: Option<&u16>) {

    match year {
        Some(y) => println!("Year is {y}"),
        None => println!("Unable to retrieve the year info...")
    }
}
