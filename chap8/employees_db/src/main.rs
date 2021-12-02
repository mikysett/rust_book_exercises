use std::collections::HashMap;
use std::io;

enum CmdType {
    Add,
    Show,
	Error,
}

#[derive(PartialEq, Eq)]
enum DepartmentName {
    All,
    Managment,
    Marketing,
    Sales,
    Engineering,
}

struct Command {
    cmd_type: CmdType,
    department: DepartmentName,
    employee: Option<String>,
}

struct Employee {
    name: String,
    department: DepartmentName,
}

fn main() {
    let mut employees: Vec<Employee> = Vec::new();
    loop {
        let mut line = String::new();

        io::stdin()
            .read_line(&mut line)
            .expect("Can't read the line");
        line = line.trim().to_string();
        if line == "q" {
            break;
        }
		let cmd = set_cmd(&line);
		match cmd.cmd_type {
			CmdType::Error => println!("Invalid Command"),
			_ => exec_cmd(&mut employees, cmd)
		}
    }
}

fn set_cmd(line: &String) -> Command {
    let words = line.split_whitespace();
    Command {
        cmd_type: CmdType::Add,
        department: DepartmentName::Sales,
        employee: Some("SAlly".to_string()),
    }
}

fn exec_cmd(employees: &mut Vec<Employee>, cmd: Command) {
    match cmd.cmd_type {
        CmdType::Add => match cmd.employee {
            Some(employee_name) => employees.push(Employee {
                name: employee_name,
                department: cmd.department,
            }),
            None => println!("Error during execution of command"),
        },
        CmdType::Show => print_department(employees, cmd.department),
		_ => println!("Unsupported cmd")
    }
}

fn print_department(employees: &mut Vec<Employee>, department: DepartmentName) {
	employees.sort_by(|a, b| b.name.cmp(&a.name));
	match department {
		DepartmentName::All => {
			for employee in employees {
				println!("{}, {}",
					employee.name,
					print_department_name(&employee.department));
			}
		},
		_ => {
			let dep_name = print_department_name(&department);
			for employee in employees {
				if employee.department == department {
					println!("{}, {}", employee.name, dep_name);
				}
			}
		}
	}
}

fn print_department_name(dep: &DepartmentName) -> String {
	match dep {
		DepartmentName::All => "All".to_string(),
		DepartmentName::Managment => "Managment".to_string(),
		DepartmentName::Sales => "Sales".to_string(),
		DepartmentName::Marketing => "Marketing".to_string(),
		DepartmentName::Engineering => "Engineering".to_string()
	}
}
