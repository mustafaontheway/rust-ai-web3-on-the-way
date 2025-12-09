fn main() {

    let my_name = String::from("Mustafa");

    println!("My name is: {}", my_name); // valid!

    drop(my_name);

    //println!("My name is: {}", my_name); // invalid!
}

