fn main() {

    let mut some_years: Vec<u16> = vec![2011, 2004, 2021, 2025];

    some_years.insert(2, 2000);

    println!("{some_years:?}"); // [2011, 2004, 2000, 2021, 2025]

    some_years.remove(0);

    println!("{some_years:?}"); // [2004, 2000, 2021, 2025]

}

