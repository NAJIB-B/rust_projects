use std::io;

fn main() {
    println!("welcome to fibonacci maker");

    let user_value: u32 = loop {
        let mut value = String::new();

        println!("Please input the length of fibonacci numbers you want to generate");

        io::stdin()
            .read_line(&mut value)
            .expect("Something went wrong taking user input");

        match value.trim().parse() {
            Ok(value) => break value,
            Err(_) => continue,
        };
    };

    println!("Starting now");
    println!("____________");
    fibonacci(user_value);
}

fn fibonacci(value: u32) {
    if value == 1 {
        println!("{}", 0);
        return;
    }

    if value == 2 {
        println!("{}", 0);
        println!("{}", 1);
        return;
    }

    let mut prev = 0;
    let mut curr = 1;
    let mut len = value - 2;
    if value > 2 {
        println!("{}", 0);
        println!("{}", 1);
        while len > 0 {
            println!("{}", prev + curr);
            let temp = curr;
            curr = prev + temp;
            prev = temp;
            len -= 1;
        }
    }
}
