fn main() {
    // minimum initial memory allocation

    let mut managers: Vec<&'static str> = Vec::with_capacity(15);

    println!("Vector length: {} - vector capacity: {}", managers.len(), managers.capacity());

    managers.push("Ayhan Bilir");

    println!("Vector length: {} - vector capacity: {}", managers.len(), managers.capacity());
}

// Vector length: 0 - vector capacity: 15
// Vector length: 1 - vector capacity: 15
