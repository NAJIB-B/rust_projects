use std::{io, collections::HashMap};

fn main() {
    println!("Welcome to mini Org");

    let mut command: String;

    let mut company: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        println!("Add staffs to department like this 'Add Sally to Engineering' or input 'get all' to get all staffs or exit to exit");

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
                command = user_input;
                println!("Exiting...");
                break;
            } else {
                println!("You inputted a wrong command, please try again");
                continue;
            }
        }

        if word_count == 2 {
            if words[0] == "get" && words[1] == "all" {
                println!("Getting all users...");
                get_all_staffs(&mut company);
            } else {
                println!("You inputted a wrong command, please try again");
                continue;
            }
        }

        if word_count == 4 {
            if words[0] == "add" && words[2] == "to" {
                println!("adding {} to {}...", words[1], words[3]);
                add_staff(&mut company, words[1].to_string(), words[3].to_string());
            } else {
                println!("You inputted a wrong command, please try again");
                continue;
            }
        } else {
                println!("You inputted a wrong command, please try again");
                continue;
        }
    }
    println!("command received: {}", command);
}

fn add_staff (company: &mut HashMap<String, Vec<String>>, name: String, department: String) {

    let new_dept: Vec<String> = Vec::new();
    let dept = company.entry(department.clone()).or_insert(new_dept);

    dept.push(name);

}

fn get_all_staffs (company: &mut HashMap<String, Vec<String>>) {

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
