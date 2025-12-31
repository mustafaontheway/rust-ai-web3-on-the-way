fn main() {

    let season: u8 = 8;

    let season_name = match season {

        1 | 2 | 12 => "Winter",
        3..=5 => "Spring",
        6..=8 => "Summer",
        9..=11 => "Autumn",
        _ => "Invalid!"
    };

    println!("Current season is: {season_name:?}")
}

// Current season is: "Summer"
