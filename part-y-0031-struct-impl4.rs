fn main() {

    let emp_bilge = Employee {
        name: String::from("Bilge Kul"),
        department_id: 99,
        title: String::from("Expert"),
        salary_tl: 62_300,
        married: true   
    };

    let emp_ayhan = Employee {name: "Ayhan Burada".to_string(), department_id: 17, title: "Senior Expert".to_string(), salary_tl: 96_000, married: false};

    println!("{}", emp_bilge.compare_salaries(&emp_ayhan)); // false

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

    // Self represents Employee struct

    fn compare_salaries(&self, other_employee: &Self) -> bool {

        self.salary_tl > other_employee.salary_tl
    }
}
