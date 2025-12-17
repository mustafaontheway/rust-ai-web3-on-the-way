fn main() {

    let dep_fintech = Departments::FinTech("01_fintech"); 

    let dep_sales = Departments::Sales(102);

    println!("{:?}", dep_sales)
}

#[derive(Debug)]
enum Departments<T> {
    
    FinTech(T),
    Sales(T),
    Operations(T),
    Finance(T),
    Audit(T)
}

