// // Fix the error with the use of define_x
// fn main() {
//     println!("{}, world", x); 
// }

// fn define_x() {
//     let x = "hello";
// }


// Fix the error with the use of define_x
fn main() {
    println!("{}, world", define_x()); 
}

fn define_x() -> String {
    let x = String::from("hello");
    x
}
