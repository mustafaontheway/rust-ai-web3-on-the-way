fn main() {

    let achieved_sales: Result<u32, &str> = Ok(657_000);

    let unexpected_sales: Result<u32, &str> = Err("Something went wrong");

    println!("{:?}", achieved_sales);

    println!("{:?}", unexpected_sales);
}

// Ok(657000)
// Err("Something went wrong")
