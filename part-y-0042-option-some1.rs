fn main() {

    #[allow(unused_variables)]
    
    let year: Option<u16> = Option::Some(2018);

    let num = Option::Some(100); // i32

    let is_ready = Option::Some(false);

    let mut name = Option::Some("Hakan Bilir".to_string());

    name = Option::Some("Hakan Bilici".to_string());

    println!("{year:?}, {name:?}") // Some(2018), Some("Hakan Bilici")
}

