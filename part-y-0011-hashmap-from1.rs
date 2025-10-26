use std::collections::HashMap;

fn main() {

    // years and sales array

    let raw_data: [(u16, u32); 4] = [(2014, 650_000), (2015, 642_000), (2016, 830_000), (2017, 870_000)];

    let years_sales = HashMap::from(raw_data);

    println!("{years_sales:?}"); // {2015: 642000, 2016: 830000, 2017: 870000, 2014: 650000}

}

