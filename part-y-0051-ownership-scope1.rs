fn main() {

    let x: u8 = 12;

    let y: i8 = -3;

    let diff = x as i8 - y;

    println!("x - y = {diff}");

    {

        let x: i16 = 100;

        let y: i16 = -2000;

        let diff = x - y;

        println!("x - y = {diff}");
    }
}

// x - y = 15
// x - y = 2100
