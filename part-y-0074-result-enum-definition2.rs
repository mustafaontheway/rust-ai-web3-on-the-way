fn main() {

    let r1 = div_nums(32 as f64, 24.17);

    println!("Div result 1: {:?}", r1); // Div result 1: Ok(1.3239553165080677)

    let r2= div_nums(14.74, 0.0) ;

    println!("Div result 2: {:?}", r2); // Div result 2: Err("Zero division error!")

}

fn div_nums(x: f64, y: f64) -> Result<f64, &'static str> {

    if y == 0.0 {

        Err("Zero division error!")

    } else {

        Ok(x / y)
    }
}
