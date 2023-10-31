use std::collections::HashMap;
use std::io::{stdin, stdout, Write};
mod number;
use number::Number;

/*
todo: Implement array

1. Take number-parsing out of get_value
    - &str -> Vec<Number>
    - Can be a single value or a comma-separated list
2. Rewrite get_value
3. Finish set_value

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

// Get value from state
fn get_value(
    token: &str,
    state: &HashMap<String, Vec<Number>>
  ) -> Option<Number> {
    /*
    Helper function to read token from state.
    `token` can be one of:
        1. Literal, does not touch state:
            1. Parse as i64
            2. Parse as f64
        
        2. Variable, stored in state:
            1. key.idx, indexing array
            2. key (same as key.0), reading a variable
    */
    if let Ok(val) = token.parse::<i64>() {
        Some(Number::I64(val))
    } else if let Ok(val) = token.parse::<f64>() {
        Some(Number::F64(val))
    } else {
        // Read "and then" as "and then, if that works,"
        // Split token into key.index
        // (todo: Handle case where idx > state[key]
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


// no return type
fn set_value(
    token: &str,
    value: Vec<Number>,
    state: &HashMap<String, Vec<Number>>
) -> () {
    /*
    Helper function to set state[token] = value
    Four cases for token, value:
    1. `key.idx, number`, i.e. state[key][index] = [number,]
    2. `key


    Three cases for token, value:
    1. key.value, number:      state[token][value] = [number]
    2. token, number:          state[token] = [number]
    3. token, num1, num2, ...: state[token] = [num1, num2, ...]

    Should probably be `Option<>` or something
    */

    // token = key.index_str
    if let Some((key, index_str)) = token.split_once('.') {
        
        // token = key.index_str, and index_str is like u64
        if let Ok(index) = index_str.parse::<usize>() {    

            // state[key] exists. Set state[key][idx] = value, resizing if necessary
            if let Some(vec) = state.get_mut(key) {
                // todo: Handle >=1 value
                if index >= vec.len() {
                    // todo: what to do when value.len() > 1?
                    vec.resize(
                        index + 1,
                        value.get(0).cloned().unwrap()
                    );
                } else {
                    vec[index] = value.get(0).cloned().unwrap();
                }
            
            // state[key] doesn't exist. Set state[key][index] = [value]
            } else {
                // Create new vector with length index + 1, value at index
                /*
                let mut vec: Vec<Number> = Vec::new();
                vec.resize(index + 1, value.get(0).cloned().unwrap());
                state.insert(key.to_string(), vec);
                */
                // oneliner hehe 
                // todo: handle the case where [value] can be many
                state.insert(
                    key.to_string(),
                    vec![value.get(0).cloned().unwrap(); index + 1]
                );
            }

        // token = key.index_str, but index_str is not like u64
        } else {
            panic!("Couldn't set array value for some reason");
        }
    } else {
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


- Get instructions from stdin.
    - No more interactive input!
    - But now we have a list of instructions...
    - Tokenize and store in list


Big things:

- Metadata (cycle count), PC in instructions
- Arrays
- Conditions and branches
- Basic programs
- Instructions from stdin. (let someone file > program)
    - Instruction index
- Array float index
- Arb precision
- To Wasm
- Webpage IDE

Longterm things:
- No panics, proper error handling
- Stricter grammar 
- Proper compiler
- Test driven dev

DONEs:
- Implement set, state on i64
- Implement add, etc. on i64
- Implement "Number" over i64, f64
- Implement parsing Number from string
- Change get state


*/
