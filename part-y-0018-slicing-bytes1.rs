fn main() {

    // slice len => bytes, NOT len

    let last_name = "Büyükdereli";

    // let s1 = &last_name[0..2];

    // println!("{s1}"); // byte index 2 is not a char boundary; it is inside 'ü' (bytes 1..3) of `Büyükdereli`

    // turkish ü => 2 bytes

    let s1 = &last_name[0..=2];

    println!("{s1}"); // Bü

    println!("Length'Bü': {}", s1.len()); // Length'Bü': 3

}

