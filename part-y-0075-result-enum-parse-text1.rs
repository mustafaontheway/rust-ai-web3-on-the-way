fn main() {

    let sales_input1 = "500000";

    let sales_data1= sales_input1.parse::<u32>();

    println!("{:?}", sales_data1);

    let sales_input2 = "500000 ";

    let  sales_data2= sales_input2.parse::<u32>();

    println!("{:?}", sales_data2);
}

// Ok(500000)
// Err(ParseIntError { kind: InvalidDigit })
