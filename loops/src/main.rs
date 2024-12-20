fn main() {
    let mut x = 5;

    let res = loop {
        x += 1;

        if x == 10 {
            break x
        }
    };
    println!("the is the result {}", res);
}
