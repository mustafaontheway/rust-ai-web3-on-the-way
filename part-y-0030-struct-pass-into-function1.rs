fn main() {

    let _emp_ayhan = set_employee("Ayhan Bilir".to_string(), 12, "Senior Expert".to_string(), 78_500, false);

    let _emp_bilge = Employee {
        name: String::from("Bilge Kul"),
        title: String::from("Expert"),
        salary_tl: 62_300,
        .._emp_ayhan
    };

    get_emp_name_salary(&_emp_ayhan); // Employee name: Ayhan Bilir and salary: 78500

    get_emp_name_salary(&_emp_bilge); // Employee name: Bilge Kul and salary: 62300

}

fn set_employee(name: String, department_id: u8, title: String, salary_tl: u32 ,married: bool) -> Employee {

    Employee { name, department_id, title, salary_tl, married }
}

fn get_emp_name_salary(emp: &Employee) {
    
    println!("Employee name: {} and salary: {}", emp.name, emp.salary_tl)
}

#[derive(Debug)]
struct Employee {

    name: String,
    department_id: u8,
    title: String,
    salary_tl: u32,
    married: bool
}
