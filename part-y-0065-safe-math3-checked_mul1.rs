fn main() {

    let mult1 = u8::checked_mul(20, 10);

    println!("Diff: {:?}", mult1);

    let mult2 = u8::checked_mul(20, 25);

    println!("Diff: {:?}", mult2);

}

// Diff: Some(200)
// Diff: None
