fn main() {

    let _years: [u16; 5] = [1990, 2012, 2021, 2024, 2026];

    let mut all_fives: [i8; 12] = [5; 12];

    all_fives[2] = -10;

    println!("{:?}", all_fives); // [5, 5, -10, 5, 5, 5, 5, 5, 5, 5, 5, 5]

}

