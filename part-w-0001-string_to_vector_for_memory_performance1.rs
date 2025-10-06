fn main() {

    // we can use vector for higher perf.

    let my_name_bytes: Vec<u8> = b"Mustafa".to_vec();

    println!("{:?}", my_name_bytes); // [77, 117, 115, 116, 97, 102, 97]

    // if we want string 

    let string_name = String::from_utf8(my_name_bytes);

    println!("{:?}", string_name); // Ok("Mustafa")

    println!("{:?}", string_name.unwrap()); // "Mustafa"

}
