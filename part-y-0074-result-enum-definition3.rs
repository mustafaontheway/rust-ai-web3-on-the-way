fn main() {

    let r1 = div_nums(32 as f64, 0.0);

    match r1 {

        Ok(div_result) => println!("Div result is: {div_result}"), 
        Err(err_msg) => println!("{err_msg}") // Zero division error!
        
    }
}

fn div_nums(x: f64, y: f64) -> Result<f64, &'static str> {

    if y == 0.0 {

        Err("Zero division error!")

    } else {

        Ok(x / y)
    }
}
