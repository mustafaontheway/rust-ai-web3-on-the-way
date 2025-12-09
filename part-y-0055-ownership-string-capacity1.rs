fn main() {

    let mut my_name = String::with_capacity(50);

    println!("Capacity: {:?}", my_name.capacity()); 
    println!("Length: {:?}", my_name.len());

    my_name = "Mustafa".to_string();

    println!("Capacity: {:?}", my_name.capacity()); 
    println!("Length: {:?}", my_name.len());

    my_name = "Mustafa Büyükdereli".to_string();

    println!("Capacity: {:?}", my_name.capacity()); 
    println!("Length: {:?}", my_name.len());
}

// Capacity: 50
// Length: 0   
// Capacity: 7 
// Length: 7   
// Capacity: 21
// Length: 21  
