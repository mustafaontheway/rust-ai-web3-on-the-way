fn main() {

    let feelings = "👀, hi...";

    let eyes = &feelings[0..=3];

    println!("Eyes: {eyes} - eyes byte length: {}", eyes.len()) // Eyes: 👀 - eyes byte length: 4

}

