fn main() {

    let mut my_name = String::with_capacity(50);

    println!("Capacity: {:?}", my_name.capacity()); 
    println!("Length: {:?}", my_name.len());

    my_name.push_str("Mustafa");

    println!("Capacity: {:?}", my_name.capacity()); 
    println!("Length: {:?}", my_name.len());

    my_name.shrink_to_fit();

    println!("Capacity: {:?}", my_name.capacity()); 
    println!("Length: {:?}", my_name.len());
}

// Capacity: 50
// Length: 0   
// Capacity: 50
// Length: 7   
// Capacity: 7 
// Length: 7   
