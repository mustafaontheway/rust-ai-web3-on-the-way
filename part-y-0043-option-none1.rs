fn main() {

    //let total_sales = Option::None; // error[E0282]: type annotations needed for `Option<_>`

    let total_sales: Option<u32> = Option::None;

    let mut expected_profit: Option<u64> = Option::None;

    expected_profit = Some(3_450_000);
}

