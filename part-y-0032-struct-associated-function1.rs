fn main() {

    let emp_bengu = Employee::new("Bengü Bulur".to_string(), 17, "Assistant".to_string(), 34_000, false);

    emp_bengu.print_salary_info(); // Employee name: Bengü Bulur - salary: 34000 ₺

    let emp_bilge = Employee {
        name: String::from("Bilge Kul"),
        department_id: 99,
        title: String::from("Expert"),
        salary_tl: 62_300,
        married: true   
    };

    let emp_ayhan = Employee {name: "Ayhan Burada".to_string(), department_id: 17, title: "Senior Expert".to_string(), salary_tl: 96_000, married: false};
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

    // Self represents Employee struct

    fn new(name: String, department_id: u8, title: String, salary_tl: u32, married: bool) -> Self {

        Employee { name, department_id, title, salary_tl, married }
    }

    fn print_salary_info(&self) {
        
        println!("Employee name: {} - salary: {} ₺", self.name, self.salary_tl)
    }

    fn compare_salaries(&self, other_employee: &Self) -> bool {

        self.salary_tl > other_employee.salary_tl
    }
}
