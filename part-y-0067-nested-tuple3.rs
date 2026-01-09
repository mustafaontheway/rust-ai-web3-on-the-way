fn main() {

    let her_data: (u16, u8, (&str, &str, bool)) = (3200, 35, ("Aygün Kaplan", "ak006587", false));

    let (salary_usd, age, (_, _, _)) = her_data;

    //let (salary_usd, age, ..) = her_data;

    println!("Her salary: ${salary_usd}"); // Her salary: $3200

    println!("Her age: {age}"); // Her age: 35

    println!("Her ID: {}", (her_data.2).1) // Her ID: ak006587
 
}

