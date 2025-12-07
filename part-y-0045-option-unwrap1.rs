fn main() {

    let important_years: [u16; 6] = [1993, 1997, 2000, 2008, 2018, 2020];

    let year1 = important_years.get(3);

    println!("Year 1 is: {:?}", year1);

    let year1 = important_years.get(3).unwrap();

    println!("Year 1 is: {year1}");

}

// Year 1 is: Some(2008)
// Year 1 is: 2008
