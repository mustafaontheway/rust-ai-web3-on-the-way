fn main() {

    // heap data, String type
    // but these are kept on the stack:
    // 1- pointer (heap address of data)
    // 2- capacity
    // 3- len

    let my_name = String::from("Mustafa");

    println!("Heap data address (pointer): {:p}", my_name.as_ptr()); // random -> security!

    println!("Capacity: {:?}", my_name.capacity()); 

    println!("Length: {:?}", my_name.len());
}

// Heap data address (pointer): 0x1e9dcac6c70
// Capacity: 7
// Length: 7  
