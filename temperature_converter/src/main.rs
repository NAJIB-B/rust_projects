use std::io;

fn main() {
    println!("Welcome to the temperature converter");

    let user_temp: f64 = loop {
        let mut temp = String::new();
        println!("type the value to convert");

        io::stdin()
            .read_line(&mut temp)
            .expect("Something went wrong, couldn't read user input");

        match temp.trim().parse() {
            Ok(temp) => break temp,
            Err(_) => continue,
        };
    };

    let mut op = String::new();
    let result = loop {
        println!("type 1 to convet to celcius or 2 to convert to fahrenhiet");

        io::stdin()
            .read_line(&mut op)
            .expect("something went wrong, couldn't take user input");

        let op: i32 = match op.trim().parse() {
            Ok(op) => op,
            Err(_) => continue,
        };

        if op == 1 {
            println!("user inputed value {user_temp}");
            break c_to_f(user_temp);
        } else if op == 2 {
            break f_to_c(user_temp);
        } else {
            continue;
        }
    };
    println!("{result}")
}

fn c_to_f(temp: f64) -> f64 {
    9.0 / 5.0 * temp + 32.0
}

fn f_to_c(temp: f64) -> f64 {
    5.0 / 9.0 * (temp - 32.0)
}
