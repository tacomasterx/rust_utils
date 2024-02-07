use std::io::{self, Write};
use getch::Getch;

pub fn pause() {
    let key_input = Getch::new();
    print!("Press any key to continue...");
    if let Err(error) = io::stdout().flush() {
        panic!("{}", error)
    }
    if let Err(error) = key_input.getch() {
        panic!("{}", error)
    }

}

pub fn printf(str: String) {
    print!("{}", str);
    if let Err(error) = io::stdout().flush() {
        panic!("{error}")
    }
}

