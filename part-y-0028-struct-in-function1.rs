fn main() {

    let _emp_ayhan = set_employee("Ayhan Bilir".to_string(), 12, "Senior Expert".to_string(), 78_500, false);

}

fn set_employee(name: String, department_id: u8, title: String, salary_tl: u32 ,married: bool) -> Employee {

    Employee { name, department_id, title, salary_tl, married }
}

struct Employee {

    name: String,
    department_id: u8,
    title: String,
    salary_tl: u32,
    married: bool
}
