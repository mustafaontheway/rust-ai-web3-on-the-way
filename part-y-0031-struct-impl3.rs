fn main() {

    let mut emp_bilge = Employee {
        name: String::from("Bilge Kul"),
        department_id: 99,
        title: String::from("Expert"),
        salary_tl: 62_300,
        married: true   
    };

    emp_bilge.salary_increase(17000);

    println!("{:?}", emp_bilge); // Employee { name: "Bilge Kul", department_id: 99, title: "Expert", salary_tl: 79300, married: true }

}


#[derive(Debug)]
struct Employee {
    name: String,
    department_id: u8,
    title: String,
    salary_tl: u32,
    married: bool
}

impl Employee {

    fn print_salary_info(&self) {
        
        println!("Employee name: {} - salary: {} ₺", self.name, self.salary_tl)
    }

    fn salary_increase(&mut self, amount: u32) {

        self.salary_tl += amount
    }
}
