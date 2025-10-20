fn main() {

    let sales: u32 = 525_000;

    let fixed_cost: u32 = 600_000;

    if sales < fixed_cost {

        panic!("We need profit! Sales must exceed fixed cost!")
    }

    println!("Profit amount: {}", sales - fixed_cost)

}

