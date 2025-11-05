fn main() {

    let mut emp_ayhan = Employee {
        name: "Ayhan Bilir".to_string(),
        department_id: 12,
        title: "Senior Expert".to_string(),
        salary_tl: 78_500,
        married: false
    };

    emp_ayhan.salary_tl = 96_000;
    emp_ayhan.married = true;

    println!("{emp_ayhan:?}") // Employee { name: "Ayhan Bilir", department_id: 12, title: "Senior Expert", salary_tl: 96000, married: true }

}

#[derive(Debug)]
struct Employee {

    name: String,
    department_id: u8,
    title: String,
    salary_tl: u32,
    married: bool
}
