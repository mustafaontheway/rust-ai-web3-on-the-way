fn main() {

    let x: u8 = 12;

    let y = x;

    println!("Stack address of x: {:p}", &x);

    println!("Stack address of y: {:p}", &y);
}

// remember the addresses are not constant, they are random!
// Stack address of x: 0x86a38ff8f6
// Stack address of y: 0x86a38ff8f7
