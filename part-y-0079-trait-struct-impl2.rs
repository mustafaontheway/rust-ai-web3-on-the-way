fn main() {

    let mut company_akis_department_fintech = TechCompany {
        company_name: "Company Akis",
        department_name: "FinTech",
        dep_emp_count: 14,
        department_head_message: String::from("We will do our best!")
    };

    company_akis_department_fintech.get_head_msg(); // Department head message: We will do our best!

    company_akis_department_fintech.set_head_msg(String::from("We will do our best! Let's do it!"));

    company_akis_department_fintech.get_head_msg(); // Department head message: We will do our best! Let's do it!

    let set_emp_num = company_akis_department_fintech.set_department("FinTech", 18);

    let dep_info = company_akis_department_fintech.get_department_info();

    println!("{:?}", dep_info); // "Company department: FinTech - department employee count: 18"

}

trait Company {
    
    fn get_department_info(&self) -> String;

    fn set_department(&mut self, name: &'static str, num_of_emp: u8);
}

struct TechCompany {
    company_name: &'static str,
    department_name: &'static str,
    dep_emp_count: u8,
    department_head_message: String
}

impl TechCompany {

    fn get_head_msg(&self) {

        println!("Department head message: {}", self.department_head_message)
    }

    fn set_head_msg(&mut self, new_msg: String) {

        self.department_head_message = new_msg
    }
}

impl Company for TechCompany {

    fn get_department_info(&self) -> String {
        
        format!("Company department: {} - department employee count: {}", self.department_name, self.dep_emp_count)
    }

    fn set_department(&mut self, name: &'static str, num_of_emp: u8) {
        
        self.department_name = name;
        self.dep_emp_count = num_of_emp;
    }
}
