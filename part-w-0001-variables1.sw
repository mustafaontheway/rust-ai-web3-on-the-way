script;

fn main() {

    let year: u16 = 2026; // immutable

    log(year);

    let mut age: u8 = 19;

    age = 99;

    log(age);
}

#[test]
fn testing() {

    main();
}

// forc test --logs
