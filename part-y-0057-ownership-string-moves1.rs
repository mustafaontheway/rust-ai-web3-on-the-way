fn main() {

    let my_name = String::from("Mustafa");

    println!("My name is: {}", my_name); // valid!

    let owner = my_name; 
    
    // my_name goes out the scope! 

    //println!("My name is: {}", my_name); // Error! borrow of moved value: `my_name`
}

