fn main() {

    let proverb1 = "A rolling stone gathers no moss!";

    let slice1 = proverb1;

    println!("{proverb1}"); // A rolling stone gathers no moss!

    let proverb2 = "Out of sight, out of mind".to_string();

    //let slice2 = proverb2[0..=2]; // Error => consider borrowing here: `&`

    let slice2 = &proverb2[0..=2];

    println!("{proverb2} - {slice2}"); // Out of sight, out of mind - Out

}

