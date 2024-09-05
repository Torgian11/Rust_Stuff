use std::io::{self, Write};

fn main() {
    let department_names = [String::from("Sales"), String::from("Engineering")];
    let mut new_company = create_company(String::from("Company A"), &department_names);

    let (employee_name, department_name) = prompt_user();

    let department = new_company.departments.iter_mut().find(|dept| dept.name == department_name);
    
    if department.is_some() {
        let dept = department.unwrap();
        let dept_employee = dept.employees.iter().any(|emp| emp.name == employee_name);
        if dept_employee {
            println!("Employee is already in that department!");
            return;
        }

        dept.employees.push(Employee{name: employee_name});
    } else {
        new_company.departments.push(Department {name: department_name, employees: vec![Employee {name: employee_name}]});
    }

    

}

struct Employee {
    name: String,
}

struct Department {
    name: String,
    employees: Vec<Employee>,
}

struct Company {
    name: String,
    departments: Vec<Department>,
}

fn create_company(company_name: String, department_names: &[String]) -> Company {
    let mut departments: Vec<Department> = Vec::new();
    for department_name in department_names {
        let department = Department {
            name: String::from(department_name.trim()),
            employees: Vec::new(),
        };
        departments.push(department);
    }

    departments[0].employees.push(Employee {name: String::from("Ashley")});

    let new_company: Company = Company {
        name: company_name,
        departments
    };
    
    return new_company;
}

fn prompt_user() -> (String, String) {
    let name_input = String::new();
    let department_input = String::new();
    
    print!("Enter the employee's name: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut name_input.trim().to_string()).expect("Error!");
    print!("Enter the department name: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut department_input.trim().to_string()).expect("Error!");

    return (name_input, department_input);
}
