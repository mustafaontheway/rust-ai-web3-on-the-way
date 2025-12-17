fn main() {

    let (year, sales_yearly): (u16, u32) = return_values(2025, 4_654_723);

    let (age, name): (u8, &'static str) = return_values(17, "ayhan bilir");

}

fn return_values<T, U>(a: T, b: U) -> (T, U) {

    (a, b)
}
