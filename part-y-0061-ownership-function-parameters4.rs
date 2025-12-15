fn main() {

    let mut person_kagan = "Kağan Öztürk".to_string();

    greeting(&mut person_kagan);

    println!("{person_kagan}") // Kağan Öztürk, nice to meet you.


}

fn greeting(name: &mut String) {

    name.push_str(", nice to meet you.");
}
