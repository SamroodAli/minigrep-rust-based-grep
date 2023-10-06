use std::env;
use std::fs;

fn main() {
    let args: Vec<String> =  env::args().collect();

    let Some(search_string) = args.get(1) else {
        return eprintln!("Please provide a search string as the first argument");
    };

    let Some(file_path) = args.get(2) else {
        return eprintln!("Please provide a filename as the second argument");
    };

    let Ok(content) = fs::read_to_string(file_path) else {
        return eprintln!("Error parsing the file");
    };

    println!("{content}");
}
