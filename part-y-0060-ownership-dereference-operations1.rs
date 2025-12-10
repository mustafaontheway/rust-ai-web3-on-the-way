fn main() {

    let year: u16 = 2025;

    let year_reference = &year;

    //println!("Year + 2 = {:?}", year_reference + 2); // it works! syntactic sugar...

    println!("Year + 2 = {:?}", *year_reference + 2);
}
