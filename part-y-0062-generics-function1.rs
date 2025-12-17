fn main() {

    let (year, sales_yearly) = return_values(2025, 4_654_723);

    let (age, name) = return_values(17, "ayhan bilir");

}

fn return_values<T, U>(a: T, b: U) -> (T, U) {

    (a, b)
}
