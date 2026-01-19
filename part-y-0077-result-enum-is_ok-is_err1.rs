fn main() {

    let r1 = div_nums(32 as f64, 16.80);

    println!("Div result 1: {:?}", r1.is_ok()); 

    let r2 = div_nums(14.32, 0.0);

    println!("Div result 2: {:?}", r2.is_err()); 

    let r3 = div_nums(14.32, 0.0);

    println!("Div result 3: {:?}", r3.is_ok());

}

fn div_nums(x: f64, y: f64) -> Result<f64, &'static str> {

    if y == 0.0 {

        Err("Zero division error!")

    } else {

        Ok(x / y)
    }
}

// Div result 1: true
// Div result 2: true
// Div result 3: false
