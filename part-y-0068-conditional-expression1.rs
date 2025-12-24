fn main() {

    let sales: u32 = 6_200_000;

    let premium = if sales >= 5_000_000 {
        100_000
    } else if sales >= 3_000_000 {
        50_000
    } else {
        0
    };
    
    println!("Premium amount: {premium} ₺")
}

// Premium amount: 100000 ₺
