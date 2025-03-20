use std::{collections::HashMap, io};

fn main() {
    println!("welcome to oraganization manager");
    println!("To add a staff to a department ");
    println!("Type: Add <staff_name> to <department_name>");
    println!("example: Add najib to engineering");
    println!("To remove a staff from a department ");
    println!("Type: Remove <staff_name> from <department_name>");
    println!("To get list of staffs in a department");
    println!("Type: List of <department_name>");
    println!("To get List of all staffs");
    println!("Type: get all staffs");
    println!("To exit");
    println!("Type: exit");
    println!("----------- Starting -------------");

    let mut company: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        println!("Please input your command");

        let mut user_input = String::new();

        io::stdin()
            .read_line(&mut user_input)
            .expect("failed to read user input");

        let word_count = user_input.split_whitespace().count();

        if word_count > 4 {
            println!("You inputted a wrong command, please try again");
            continue;
        }

        let user_input_lower = user_input.to_lowercase();

        let words: Vec<&str> = user_input_lower.split_whitespace().collect();

        if word_count == 1 {
            if words[0] == "exit" {
                println!("Exiting...");
                break;
            } else {
                println!("You inputted a wrong command, please try again");
                continue;
            }
        }

        if word_count == 3 {
            if words[0] == "get" && words[1] == "all" && words[2] == "staffs" {
                println!("Getting all staffs...");
                get_all_staffs(&mut company);
            } else if words[0] == "list" && words[1] == "of" {
                println!("Getting all staffs in {}...", words[2]);
                get_dept_staff(&mut company, words[2].to_string());
            } else {
                println!("You inputted a wrong command, please try again");
                continue;
            }
        }

        if word_count == 4 {
            if words[0] == "add" && words[2] == "to" {
                println!("adding {} to {}...", words[1], words[3]);
                add_staff(&mut company, words[1].to_string(), words[3].to_string());
            } else if words[0] == "remove" && words[2] == "from" {
                println!("removing {} to {}...", words[1], words[3]);
                remove_staff(&mut company, words[1].to_string(), words[3].to_string());
            } else {
                println!("You inputted a wrong command, please try again");
                continue;
            }
        } else {
            println!("You inputted a wrong command, please try again");
            continue;
        }
    }
}

fn add_staff(company: &mut HashMap<String, Vec<String>>, name: String, department: String) {
    let new_dept: Vec<String> = Vec::new();
    let dept = company.entry(department.clone()).or_insert(new_dept);

    dept.push(name);
}

fn remove_staff(company: &mut HashMap<String, Vec<String>>, name: String, department: String) {
    if check_for_dept(company, department.clone()) {
        if let Some(staff_list) = company.get_mut(&department) {
            if let Some(index) = staff_list.iter().position(|staff| *staff == name) {
                staff_list.remove(index);
            } else {
                println!("{} is not in {} department", name, department);
            }
        }
    } else {
        println!("No such department exists");
    }
}

fn get_dept_staff(company: &mut HashMap<String, Vec<String>>, department: String) {
    if check_for_dept(company, department.clone()) {
        if let Some(staff_list) = company.get_mut(&department) {
            staff_list.sort();

            println!("{}", department);
            println!("-----------");

            for staff in staff_list {
                println!("{}", staff);
            }
        }
    } else {
        println!("No such department exists");
    }
}

fn check_for_dept(company: &mut HashMap<String, Vec<String>>, department: String) -> bool {
    let dept = match company.get(&department) {
        Some(dept) => true,
        None => false,
    };
    dept
}

fn get_all_staffs(company: &mut HashMap<String, Vec<String>>) {
    for (dept, staff) in company {
        staff.sort();
        println!("{}", dept);
        println!("------");
        for n in staff {
            println!("{}", n);
            println!("")
        }
    }
}
