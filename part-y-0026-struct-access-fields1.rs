fn main() {

    let _emp_ayhan = Employee {
        name: "Ayhan Bilir".to_string(),
        department_id: 12,
        title: "Senior Expert".to_string(),
        salary_tl: 148_500,
        married: false
    };

    let emp_name = _emp_ayhan.name;

    println!("Employee name is {emp_name}."); // Employee name is Ayhan Bilir.

    //println!("Employee name is {}.", _emp_ayhan.name); // Error! => borrow of moved value: `_emp_ayhan.name`

}

#[derive(Debug)]
struct Employee {

    name: String,
    department_id: u8,
    title: String,
    salary_tl: u32,
    married: bool
}
