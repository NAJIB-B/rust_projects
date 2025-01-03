use std::collections::HashMap;

fn main() {
    let mut  v = vec![5, 5, 5, 5, 4, 3, 2, 2, 2, 1];
    v.sort();

    let median = median(&v);
    let mode = mode(&v);

    println!("the median is {median}");
    println!("the mode is {mode}");
}

fn median(list: &Vec<i32>) -> f64 {

    let len = list.len();

    if len % 2 == 1 {
        list[len / 2] as f64
        
    } else {
        let md = len / 2;

        let el = (list[md] + list[md - 1]) as f64 / 2.0;
        
        el as f64
    }
}

fn mode(list: &Vec<i32>) -> i32 {

    let mut occurences = HashMap::new();


    for value in list {
        let count = occurences.entry(value).or_insert(0);

        *count += 1;
    }

    let mut highest = 0;
    for (k, v) in &occurences {

        if *v > highest {
            highest = **k
        }

    }
    return highest
}




