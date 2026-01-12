fn main() {

    let stra1 = Strategy::Buy(5600);

    let stra2 = Strategy::Keep(5600);

    println!("Strategy comparision: {}", stra1 == stra2); // Strategy comparision: false
}

#[derive(Debug)]
#[derive(PartialEq)]
enum Strategy {
    Keep(u32),
    Buy(u32),
    Sell(u32)
}
