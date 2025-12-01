fn main() {

    let playing_time = ElapsedTime(3, 42, 57);

    println!("Hours: {} - minutes: {} - seconds: {}", playing_time.0, playing_time.1, playing_time.2); // Hours: 3 - minutes: 42 - seconds: 57

}

// hours, minutes and seconds
struct ElapsedTime(u32, u32, u32);


