fn main() {

    let important_years: [u16; 6] = [1993, 1997, 2000, 2008, 2018, 2020];

    let year1 = important_years.get(3);

    match year1 {
        
        Some(year) => println!("Year is {year}"),
        None => println!("The index value must be smaller than array length")
    }

    let year2 = important_years.get(33);

    match year2 {
        
        Some(year) => println!("Year is {year}"),
        None => println!("The index value must be smaller than array length")
    }
}

// Year is 2008
// The index value must be smaller than array length
