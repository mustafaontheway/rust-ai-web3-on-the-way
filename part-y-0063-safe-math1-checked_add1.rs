fn main() {

    let sum1 = u8::checked_add(200, 50);

    println!("Sum: {:?}", sum1);

    let sum2 = u8::checked_add(200, 200);

    println!("Sum: {:?}", sum2);

}

// Sum: Some(250)
// Sum: None
