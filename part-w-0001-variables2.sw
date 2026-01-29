script;

fn main() {

    let greet: str[33] = __to_str_array("Hi! Mustafa Buyukdereli was here!");

    log(greet);
}

#[test]
fn testing() {

    main();
}

// forc test --logs
