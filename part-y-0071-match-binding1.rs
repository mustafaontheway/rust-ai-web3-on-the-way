fn main() {

    month_info(4);

    month_info(17);  
}

fn month_info(m: u8) {

    match m {

        m @ 1..=12 => println!("Current month number: {m}"),
        _ => println!("Invalid month number!")
        
    }
}

// Current month number: 4
// Invalid month number!
