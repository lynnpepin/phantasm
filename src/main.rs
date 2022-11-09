use std::io::{stdin,stdout,Write};

fn main() {
    loop {
        print!(">>> ");
        let ss:String = String::from("hello"); //input();
        print!("\n... {}\n", ss);
    }
}

/*
todos
 - loop over input, printing response
 - tokenize each line
 - implement 'set x 0'
 - use arbitrary precision for each value
*/
