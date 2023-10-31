use std::collections::HashMap;
use std::io::{stdin, stdout, Write};
mod number;
use number::Number;

/*
todo: Implement array

0. Document reading array as `key.index`
1. `get_value` tries I64, then F64, then key.index as state[key][index], then state[key][0]
2. `set_value` for key -> [val,]
3. `set_value` for key -> [val1, val2, val3, ...]
4. `set_value` for key.index
5. Common datastructure functions: Push, pop, len, queue, dequeue, etc.

*/

// Get input string from stdin
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

fn get_value(
    token: &str,
    state: &HashMap<String, Vec<Number>>
  ) -> Option<Number> {
    if let Ok(val) = token.parse::<i64>() {
        Some(Number::I64(val))
    } else if let Ok(val) = token.parse::<f64>() {
        Some(Number::F64(val))
    } else {
        // Read "and then" as "and then, if that works,"
        // Split token into key.index
        token.split_once('.').and_then(
            // Parse `idx` as an index
            |(key, idx)| idx.parse::<usize>().ok().and_then(
                // Get the vec from state[key]
                |idx| state.get(key).and_then(
                    // Get vec[idx]
                    |vec| vec.get(idx))
                )
            ).cloned()
        .or_else(
            || state.get(token).and_then(|vec| vec.get(0)).cloned()
        )
    }
}


  

fn main() {
    // Store instructions (list of str), instruction index, state (variable hashmap)
    // Instructions: List of string representing pseudo-ASM instructions
    let mut instructions: Vec<String> = Vec::new();
    // Instruction index: Represents the currently running instruction
    let mut idx: usize = 0;
    // State: Variable hashmap of String key to i64 value
    let mut state: HashMap<String, Vec<Number>> = HashMap::new();


    loop {
        print!(">>> ");
        let input_string: String = input();
        let input_tokens: Vec<&str> = input_string.split(" ").collect::<Vec<&str>>();

        // Operate on each line
        match input_tokens.as_slice() {
            // Directly manipulate and view state
            ["set", kk, vv] => {
                //state.insert(kk.to_string(), Number::from(vv.to_string()));
                // But we want Vec<Number> not Number, so:
                state.insert(kk.to_string(), vec![Number::from(vv.to_string())]);
                // todo: update this and below to allow kk of format kk[index]


            },
            ["del", kk] => {
                state.remove(&kk.to_string());
            },
            ["print"] => { println!("{:?}", state) },
            ["print", kk] => { println!("{:?}", get_value(kk, &state)); },
            // todo: need to figure out how to just derive all of these from primitive
            //["bprint", kk] => { println!("0b{:b}", get_value(kk, &state)) },
            //["xprint", kk] => { println!("0x{:X}", get_value(kk, &state)) },
            //["oprint", kk] => { println!("0o{:o}", get_value(kk, &state)) }

            // Arithmetic
            // (todo: consider ["add", kx, ky] to print for interactive?)
            ["add", kk, kx, ky] => {
                state.insert(
                    kk.to_string(),
                    get_value(kx, &state).unwrap() + get_value(ky, &state).unwrap()
                );
            },
            ["sub", kk, kx, ky] => {
                state.insert(
                    kk.to_string(),
                    get_value(kx, &state).unwrap() - get_value(ky, &state).unwrap()
                );
            },
            ["mul", kk, kx, ky] => {
                state.insert(
                    kk.to_string(),
                    get_value(kx, &state).unwrap() * get_value(ky, &state).unwrap()
                );
            },
            ["div", kk, kx, ky] => {
                state.insert(
                    kk.to_string(),
                    get_value(kx, &state).unwrap() / get_value(ky, &state).unwrap()
                );
            },
            ["and", kk, kx, ky] => {
                state.insert(
                    kk.to_string(),
                    get_value(kx, &state).unwrap() & get_value(ky, &state).unwrap()
                );
            },
            ["or", kk, kx, ky] => {
                state.insert(
                    kk.to_string(),
                    get_value(kx, &state).unwrap() | get_value(ky, &state).unwrap()
                );
            },
            ["xor", kk, kx, ky] => {
                state.insert(
                    kk.to_string(),
                    get_value(kx, &state).unwrap() ^ get_value(ky, &state).unwrap()
                );
            },
            ["not", kk, kx] => {
                state.insert(
                    kk.to_string(),
                    !get_value(kx, &state).unwrap()
                );
            },
            ["exit"] => {
                println!("exit");
                break;
            },
            _ => println!("{:?}", input_tokens),
        }

    println!("Updated state: {:?}", state);
    idx += 1;
    }
}

/*
TODOs:

- Implement `array`
    - Indexed list of numbers stored in `state`
- Implement `input`

- Implement all non-jumping instructions. (interactive input)
// Arithmetic: add sub mul div
// Logic:      and, or, xor, not
// (Logic operators are bitwise)
// Conditions: eq, ne, lt, gt, leq, geq
// Bitwise:    shl, shr, rol, ror
// Control flow: label, jump
// Comments! 

- Get instructions from stdin.
    - No more interactive input!
    - But now we have a list of instructions...
    - Tokenize and store in list


Big things:

- Metadata (cycle count)
- Conditions and branches
- Arrays
- Basic programs
- Instructions from stdin. (let someone file > program)
    - Instruction index
- Arb precision
- To Wasm
- Webpage IDE
- honestly i should learn rust tdd

DONEs:
- Implement set, state on i64
- Implement add, etc. on i64
- Implement "Number" over i64, f64
- Implement parsing Number from string
- Change get state


*/
