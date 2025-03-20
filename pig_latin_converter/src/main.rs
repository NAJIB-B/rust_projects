use std::io;

fn main() {
    println!("Welcome to my pig latin converter");


    let mut user_input = String::new();

    io::stdin().read_line(&mut user_input).expect("something went wrong reading user input");

    let pig_latin = converter(user_input);

    println!("IN PIG LATIN ==> {pig_latin}");

}


fn converter(value: String) -> String{

    let vowels = ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];

    let mut words: Vec<String> = value.split(' ').map(|s| s.to_string()).collect();

    for word in &mut words {
        let first_letter = word.chars().next().unwrap();

        if vowels.contains(&first_letter) {

            let new_word = format!("{word}-hay");

            *word = new_word;
        } else {
            let edited_word = remove_first_letter(word);
            let new_word = format!("{edited_word}-{first_letter}ay");

            *word = new_word;
        }
    }

    words.join(" ")
}


fn remove_first_letter(word: &String) -> String {
    let mut chars:Vec<char> = word.chars().collect();

    chars.remove(0);
    chars.into_iter().collect()
}
