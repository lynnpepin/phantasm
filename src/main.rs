use std::collections::HashMap;
use std::io::{stdin, stdout, Write};


fn input() -> String {
    let mut ss = String::new();
    // Ensure chars get printed before the prompt
    stdout().flush(); 
    stdin().read_line(&mut ss).expect("Broken string");

    // Remove newlines if the terminal brings them in
    if let Some('\n') = ss.chars().next_back() {
        ss.pop();
    }
    if let Some('\r') = ss.chars().next_back() {
        ss.pop();
    }

    ss
}

/* Phantasm numbers are currently just i64...

Soon, implement enum Number {i64, f64, arbitrary precision), etc.
*/

fn get_value(
    token: &str,
    state: &HashMap<String, i64>
) -> i64 {
    // Parse token as i64 if possible
    // Else, look up in state
    match token.parse::<i64>() {
        Ok(value) => value,
        Err(_) => *state.get(token).unwrap_or(&0)
    }
}

fn main() {
    // Store instructions (list of str), instruction index, state (variable hashmap)
    // Instructions: List of string representing pseudo-ASM instructions
    let mut instructions: Vec<String> = Vec::new();
    // Instruction index: Represents the currently running instruction
    let mut idx: usize = 0;
    // State: Variable hashmap of String key to i64 value
    let mut state: HashMap<String, i64> = HashMap::new();


    loop {
        print!(">>> ");
        let input_string: String = input();
        let input_tokens: Vec<&str> = input_string.split(" ").collect::<Vec<&str>>();

        // Operate on each line
        match input_tokens.as_slice() {
            // Direclty manipulate and view state
            ["set", kk, vv] => {
                println!("set {} = {}", kk, vv);
                state.insert(kk.to_string(), vv.parse::<i64>().expect("Not a number"));
                println!("Updated state: {:?}", state);
            },
            ["del", kk] => {
                println!("del {}", kk);
                state.remove(&kk.to_string());
                println!("Updated state: {:?}", state);
            },
            ["print"] => {
                println!("{:?}", state);
            }
            ["print", kk] => {
                println!("{}", get_value(kk, &state));
            },
            ["bprint", kk] => {
                println!("0b{:b}", get_value(kk, &state));
            },
            ["xprint", kk] => {
                println!("0x{:X}", get_value(kk, &state));
            },
            ["oprint", kk] => {
                println!("0o{:o}", get_value(kk, &state));
            }

            // Arithmetic
            ["add", kk, kx, ky] => {
                println!("add {} {} {}", kk, kx, ky);
                state.insert(
                    kk.to_string(),
                    get_value(kx, &state) + get_value(ky, &state)
                );
                println!("Updated state: {:?}", state);
            },
            ["sub", kk, kx, ky] => {
                println!("add {} {} {}", kk, kx, ky);
                state.insert(
                    kk.to_string(),
                    get_value(kx, &state) - get_value(ky, &state)
                );
                println!("Updated state: {:?}", state);
            },
            ["mul", kk, kx, ky] => {
                println!("add {} {} {}", kk, kx, ky);
                state.insert(
                    kk.to_string(),
                    get_value(kx, &state) * get_value(ky, &state)
                );
                println!("Updated state: {:?}", state);
            },
            ["div", kk, kx, ky] => {
                println!("add {} {} {}", kk, kx, ky);
                state.insert(
                    kk.to_string(),
                    get_value(kx, &state) / get_value(ky, &state)
                );
                println!("Updated state: {:?}", state);
            },
            ["exit"] => {
                println!("exit");
                break;
            },
            _ => println!("{:?}", input_tokens),
        }
    }
}
/*

Basic idea:

- Program stores:
    - Instructions list
    - Index of instruction
    - Variable hashmap
        - Values are numbers
        - Arb precision!
        - Labels to jump to are just variables
    - Metadata:
        - Cycle count

TODOs:

- Implement all non-jumping instructions. (interactive input)
// Arithmetic: add sub mul div
// Logic:      and, or, xor, not
// (Logic operators are bitwise)
// Conditions: eq, ne, lt, gt, leq, geq
// Bitwise:    shl, shr, rol, ror
// Control flow: label, jump

- Get instructions from stdin.
    - No more interactive input!
    - But now we have a list of instructions...
    - Tokenize and store in list

- Implement jumping instructions

- Implement some programs...

- Arb precision

*/
