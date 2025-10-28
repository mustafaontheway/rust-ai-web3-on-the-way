fn main() {

    let mut some_years: [u16; 5] = [2018, 2020, 2022, 2023, 2025];

    let s1 = &mut some_years[..=2];

    s1[1] = 1997;

    println!("{s1:?}");

    println!("{some_years:?}")
}

// [2018, 1997, 2022]
// [2018, 1997, 2022, 2023, 2025]
