fn main() {

    let mut emp_bengu = Employee::new("Bengü Bulur".to_string(), 17, "Assistant".to_string(), 34_000, false);

    // let's use new method

    emp_bengu.update_salary(38_200);

    println!("{:#?}", emp_bengu)
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
    
    fn new(name: String, department_id: u8, title: String, salary_tl: u32, married: bool) -> Self {

        Self { name, department_id, title, salary_tl, married } // We can use Self, b/c resilient to change (assume we change Employee -> Employees)
    }
}


impl Employee {

    // mutable!

    fn update_salary(&mut self, new_salary_tl: u32) {

        self.salary_tl = new_salary_tl
    }

    fn print_salary_info(&self) {
        
        println!("Employee name: {} - salary: {} ₺", self.name, self.salary_tl)
    }

    fn compare_salaries(&self, other_employee: &Self) -> bool {

        self.salary_tl > other_employee.salary_tl
    }
}
