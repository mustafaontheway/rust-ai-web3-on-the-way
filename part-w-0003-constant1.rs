const TAX_RATE: f32 = 0.25;

const FIXED_COST: u32 = 64_000;

fn main() {

    let gross_sales: u32 = 82_000;

    let gross_profit = gross_sales - FIXED_COST;

    let net_profit = gross_profit as f32 * (1.0 - TAX_RATE);

    println!("Net profit: ${net_profit}") // Net profit: $13500

}

