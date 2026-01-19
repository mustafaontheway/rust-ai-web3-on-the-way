fn main() {

    let r1 = div_nums(32 as f64, 16.80);

    println!("Div result 1: {:?}", r1.unwrap()); // Div result 1: 1.9047619047619047

    // let r2 = div_nums(14.32, 0.0);

    // println!("Div result 2: {:?}", r2.unwrap()); // thread 'main' panicked at main.rs:9:40:

    let r3 = div_nums(14.32, 0.0);

    println!("Div result 3: {:?}", r3.unwrap_or(0.0)); // Div result 3: 0.0

}

fn div_nums(x: f64, y: f64) -> Result<f64, &'static str> {

    if y == 0.0 {

        Err("Zero division error!")

    } else {

        Ok(x / y)
    }
}
