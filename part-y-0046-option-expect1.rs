fn main() {

    let important_years: [u16; 6] = [1993, 1997, 2000, 2008, 2018, 2020];

    let year1 = important_years.get(33).expect("The index value must be smaller than array length"); 
    
    // thread 'main' panicked at src\main.rs:5:41: The index value must be smaller than array length

}
