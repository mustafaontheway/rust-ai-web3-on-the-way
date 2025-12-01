fn main() {

    let mut emp_bengu = Employee::new("Bengü Bulur".to_string(), 17, "Assistant".to_string(), 34_000, false);

    emp_bengu.update_and_return_salary(39_000);

    println!("{:?}", emp_bengu)

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

        Self { name, department_id, title, salary_tl, married } 
    }
}


impl Employee {

    fn update_and_return_salary(&mut self, new_salary_tl: u32) -> &mut Self {

        self.salary_tl = new_salary_tl;
        self
    }

    fn print_salary_info(&self) {
        
        println!("Employee name: {} - salary: {} ₺", self.name, self.salary_tl)
    }

    fn compare_salaries(&self, other_employee: &Self) -> bool {

        self.salary_tl > other_employee.salary_tl
    }
}
