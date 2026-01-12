fn main() {

    let rec1 = Rectangle {w: 3.35, h: 3.35};

    println!("Comparision 1: {}", rec1.w == rec1.h as f32);

    let rec2 = Rectangle {w: 22.0/7.0, h: 22.0/7.0};

     println!("Comparision 2: {}", rec2.w == rec2.h as f32);
}

#[derive(Debug)]
#[derive(PartialEq)]
struct Rectangle {
    w: f32,
    h: f64
}
