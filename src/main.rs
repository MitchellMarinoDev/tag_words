mod letters;

fn main() {
    let word = std::env::args().skip(1).fold("".to_owned(), |acc, arg| acc + &arg + " ");
    if word.trim().is_empty() {
        println!("Please pass in a string to use.");
        return;
    }

    let tagged_word = letters::map_str(word.trim()).expect("cannot format the given string");

    for line in tagged_word {
        println!("{}", line);
    }
}
