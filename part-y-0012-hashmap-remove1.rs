use std::collections::HashMap;

fn main() {

    // years and sales array

    let raw_data: [(&str, u32); 4] = [("2014", 650_000), ("2015", 642_000), ("2016", 830_000), ("2017", 870_000)];

    let mut years_sales = HashMap::from(raw_data);

    let sales_second_year = years_sales.remove("2015");

    println!("2015 sales: {:?}", sales_second_year); // 2015 sales: Some(642000)

    println!("2015 sales: {:?}", sales_second_year.unwrap()); // 2015 sales: 642000

    let sales_sixth_year = years_sales.remove("2019");

    println!("2019 sales: {:?}", sales_sixth_year); // 2019 sales: None  

    println!("Sales data remaining: {years_sales:?}") // Sales data remaining: {"2017": 870000, "2014": 650000, "2016": 830000}

}

