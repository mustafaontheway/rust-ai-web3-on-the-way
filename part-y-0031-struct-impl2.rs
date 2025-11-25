fn main() {

    let mut emp_bilge = Employee {
        name: String::from("Bilge Kul"),
        department_id: 99,
        title: String::from("Expert"),
        salary_tl: 62_300,
        married: true   
    };

    emp_bilge.print_salary_info(); // Employee name: Bilge Kul - salary: 62300 ₺

    emp_bilge.salary_increase();

    emp_bilge.print_salary_info(); // Employee name: Bilge Kul - salary: 72300 ₺

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

    fn salary_increase(&mut self) {

        self.salary_tl += 10000
    }
}
