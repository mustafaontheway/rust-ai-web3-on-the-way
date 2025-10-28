fn main() {

    let some_years: [u16; 5] = [2018, 2020, 2022, 2023, 2025];

    let s1 = &some_years[..=2];

    println!("{s1:?}"); // [2018, 2020, 2022]

    let _s2 = &some_years[4];
}

