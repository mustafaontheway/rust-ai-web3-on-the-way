fn main() {

    let _emp_ayhan = Employee {
        name: "Ayhan Bilir".to_string(),
        department_id: 12,
        title: "Senior Expert".to_string(),
        salary_tl: 148_500,
        married: false
    };

    println!("{_emp_ayhan:?}") // Employee { name: "Ayhan Bilir", department_id: 12, title: "Senior Expert", salary_tl: 148500, married: false }

}

#[derive(Debug)]
struct Employee {

    name: String,
    department_id: u8,
    title: String,
    salary_tl: u32,
    married: bool
}
