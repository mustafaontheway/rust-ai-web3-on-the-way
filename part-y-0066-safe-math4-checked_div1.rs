fn main() {

    let div1 = u8::checked_div(20, 10);

    println!("Division: {:?}", div1);

    let div2 = u8::checked_div(20, 0);

    println!("Division: {:?}", div2);

}

// Division: Some(2)
// Division: None
