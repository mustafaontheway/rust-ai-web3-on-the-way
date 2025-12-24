fn main() {

    let mut year: u16 = 2025;

    loop {

        year += 1;

        if year == 2030 {
            continue;
        }

        if year == 2035 {
            break;
        }

        println!("Year is {year}");
    }

    println!("Loop ended!");
}

// Year is 2026
// Year is 2027
// Year is 2028
// Year is 2029
// Year is 2031
// Year is 2032
// Year is 2033
// Year is 2034
