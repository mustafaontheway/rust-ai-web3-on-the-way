fn main() {

    let year: u16 = 2025;

    let year_ref = &year;

    //let res = plus_two(year_ref); // error => consider dereferencing the borrow: `*`

    let res = plus_two(*year_ref);
}

fn plus_two(number: u16) -> u16 {

    number + 2
}
