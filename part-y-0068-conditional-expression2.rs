fn main() {

    let r1 = even_or_odd(12654);

    let r2 = even_or_odd(-657777);

    println!("{r1}"); // Number 12654 is even!

    println!("{r2}"); // Number -657777 is odd!

}

fn even_or_odd(num: i64) -> String {

    let result = if num % 2 == 0 {
        format!("Number {} is even!", num)
    } else {
        format!("Number {} is odd!", num)
    };

    result
}

