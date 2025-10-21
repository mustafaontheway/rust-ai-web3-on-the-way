fn main() {

    let ages: Vec<u8> = vec![12, 9, 21, 47, 55, 87];

    let age1 = ages[0];

    let age2 = ages.get(1);

    println!("{age1} - {age2:?}"); // 12 - Some(9)

    //let age50 = ages[49]; // panic! => index out of bounds: the len is 6 but the index is 49

    let age50 = ages.get(49);

    println!("{age50:?}"); // None

    let age2 = ages.get(1).unwrap();

    println!("{age2}"); // 9

    println!("{}", age2 - 3); // 6
}

