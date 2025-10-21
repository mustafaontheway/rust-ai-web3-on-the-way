fn main() {

    let city1 = String::from("İzmir");
    let city2 = String::from("Ankara");
    let city3 = String::from("İstanbul");

    let big_cities = vec![city1, city2, city3];

    println!("{big_cities:?}");

    //println!("İzmir: {city1}"); // Error => borrow of moved value: `city1`

    let izmir_city = &big_cities[0]; 

    println!("İzmir: {izmir_city}"); // borrowing - referencing => İzmir: İzmir

    let some_negs: Vec<i8> = vec![-100, -25, -12];

    let first_neg = some_negs[0]; // copy trait, not reference

    println!("{some_negs:?}"); // [-100, -25, -12]

    println!("{first_neg:?}"); // -100
}
