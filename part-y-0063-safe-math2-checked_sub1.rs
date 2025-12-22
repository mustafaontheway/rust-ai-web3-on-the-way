fn main() {

    let diff1 = u8::checked_sub(200, 50);

    println!("Diff: {:?}", diff1);

    let diff2 = u8::checked_add(200, 250);

    println!("Diff: {:?}", diff2);

}

// Diff: Some(150)
// Diff: None
