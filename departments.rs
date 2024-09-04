use std::io::{self, Write};

fn main() {
    let department_names = [String::from("Sales"), String::from("Engineering")];
    let mut new_company = create_company(String::from("Company A"), &department_names);
    println!("{new_company:?}");

    let input_result = prompt_user();

    let employee_name = input_result[0].trim();
    let department_name = input_result[1].trim();

    let department = new_company.departments.iter_mut().find(|dept| dept.name == department_name);
    
    if department.is_some() {
        let dept = department.unwrap();
        let dept_employee = dept.employees.iter().any(|emp| emp.name == employee_name);
        if dept_employee {
            println!("Employee is already in that department!");
            return;
        }

        dept.employees.push(Employee{name: employee_name.to_string()});
    } else {
        new_company.departments.push(Department {name: department_name.to_string(), employees: vec![Employee {name: employee_name.to_string()}]});
    }

    println!("{new_company:?}");

}

#[derive(Debug)]
struct Employee {
    name: String,
}

#[derive(Debug)]
struct Department {
    name: String,
    employees: Vec<Employee>,
}

#[derive(Debug)]
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
    println!("{new_company:?}");
    return new_company;
}

fn prompt_user() -> [String; 2] {
    let mut name_input = String::new();
    let mut department_input = String::new();
    
    print!("Enter the employee's name: ");
    let _=io::stdout().flush();
    let _ = io::stdin().read_line(&mut name_input);
    print!("Enter the department name: ");
    let _=io::stdout().flush();
    let _=io::stdin().read_line(&mut department_input);

    return [name_input, department_input];
}
