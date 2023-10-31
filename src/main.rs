use std::collections::HashMap;
use std::io::{stdin, stdout, Write};
mod number;
use number::Number;

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

// Parse comma-separated string to numbers
fn parse_numbers(ss: &String) -> Result<Vec<Number>, ()> {
  //Helper function to parse a comma-separated list of numbers
  // If any of the numbers fail to parse, return Err ()
  // Todo: Clean this up, better E
  let mut vec: Vec<Number> = Vec::new();
  for token in ss.split(",") {
    if let Ok(num) = token.parse::<i64>() {
      vec.push(Number::I64(num));
    } else if let Ok(num) = token.parse::<f64>() {
      vec.push(Number::F64(num));
    } else {
      println!("Could not parse {} as number", token);
      return Err(())
    }
  }
  Ok(vec)
}

// Helper func for state[key][idx]
fn get_value_from_state(
  state: &HashMap<String, Vec<Number>>,
  key: String,
  idx: Option<usize>, 
) -> Result<Number, ()> {
  // Option instead of default args: stackoverflow.com/questions/24047686/
  state.get(&key)
  .map(|vec| vec[idx.unwrap_or(0)])
  .ok_or(())
}

// Interpret value as literal or variable embedded in state
// e.g. get_value(kk, state) -> 3, or get_value(kk.0, state) -> 3, or get_value(ll.4, state) -> 1.414
fn get_value(
  token: String, state: &HashMap<String, Vec<Number>>
) -> Result<Number, ()> {
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
    Ok(Number::I64(val))
  } else if let Ok(val) = token.parse::<f64>() {
    Ok(Number::F64(val))
  } else {
    // Read "and then" as "and then, if that works,"
    // Split token into key.index
    
    // token = key.index_str
    if let Some((key, index_str)) = token.split_once('.') {
      match get_value_from_state(state, key.to_string(), index_str.parse::<usize>().ok()) {
        Ok(num) => Ok(num),
        Err(_) => {
          println!("Could not parse token {} in get_value call", token);
          Err(())
        }
      }
    } else {
      get_value_from_state(state, token, None)
    }
  }
}

// state[key][index] = value
fn set_value_in_vec_in_state(
  state: &mut HashMap<String, Vec<Number>>,
  key: String,
  idx: Option<usize>, 
  value: Number,
) -> Result<(), String> {
  match state.get_mut(&key) {
    // state[key] exists; update it
    Some(vec) => {
      vec[idx.unwrap_or(0)] = value;
      Ok(())
    },
    // state[key] doesn't exist; create it
    None => {
      let mut vec: Vec<Number> = Vec::new();
      vec.resize(idx.unwrap_or(0) + 1, Number::I64(0));
      vec[idx.unwrap_or(0)] = value;
      Ok(())
    }
  }
}

// set state[key][idx:] = values, overwriting
fn set_value_in_state(
  state: &mut HashMap<String, Vec<Number>>,
  key: String,
  idx: Option<usize>, 
  values: Vec<Number>,
) -> Result<(), String> {
  /*
  Set state[key][idx:] = value,
  e.g. with state["foo"] = [0, 10, 20, 30],
  set_value_in_state("foo", 3, [1, 2, 3, 4, 5])
  yields state["foo"] = [0, 10, 20, 1, 2, 3, 4, 5]
  */
  let index = idx.unwrap_or(0);
  println!("set_value_in_state({}, {}, {:?})", key, index, values);
  match state.get_mut(&key) {
    // state[key] exists; update it
    Some(vec) => {
      // Resize if it's too small
      if index <= (vec.len() + values.len()) {
        vec.resize(
          index + values.len() + 1,
          Number::I64(0)
        )
      }
      // overwrite vec[idx:idx+values.len()] with values
      vec.splice(
        index..index + values.len(),
        values.into_iter()
      );
      
      Ok(())
    },
    // state[key] doesn't exist; create it
    None => {
      state.insert(
        key.to_string(),
        values
      );
      Ok(())
    }
  }
}

// Set state[token] = [val1, val2, ...] or state[key][idx] = [val1, val2, ...],
fn set_value(
  state: &mut HashMap<String, Vec<Number>>,
  token: String,
  value: String,
) -> Result<(), &str> {
  /*
  Helper function to set state[token] = value
  Four cases for token, value:
  1. `key.idx, number`, i.e. state[key][index] = [number,]
  2. `key
  
  
  Three cases for token, value:
  1. "key.idx", "num":      state[token][idx] = [number]
  2. "token", "num":          state[token] = [number]
  3. "token", "num1,num2", ...: state[token] = [num1, num2, ...]
  
  Consider handling fourth  case:
  4. "key.idx", "num1,num2"
  
  Should probably be `Option<>` or something
  */
  println!("set_value({}, {})", token, value);
  match parse_numbers(&token) {
    Ok(values) => {
      println!(".. values: {:?}", values);
      match token.split_once('.') {
        Some((key, index_str)) => {
          println!(".. .. key: {}, index_str: {}", key, index_str);
          match index_str.parse::<usize>() {
            Ok(idx) => {
              set_value_in_state(
                state,
                key.to_string(),
                Some(idx),
                values
              );
              Ok(())
            },
            Err(ParseIntError) => {
              println!("Could not parse {} in set_value call", index_str);
              Err("Could not parse index in set_value call")
            }
            
          }
        },
        // Handle the cases 2, 3
        None => {
          match values.len() {
            0 => {
              println!("Could not parse empty values");
              Err("Could not parse empty values")
            },
            _ => {
              state.insert(
                token.to_string(),
                values
              );
              Ok(())
            }
          }
        }
      }
    },
    Err(_) => {
      println!("Could not parse values in set_value call");
      Err("Could not parse values in set_value call")
    },
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
        set_value(&mut state, kk.to_string(), vv.to_string());
      },
      /*
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
      */
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
- Should I be using `&str` or `String`?
- No panics, proper error handling
- Stricter grammar 
- Proper compiler
- Test driven dev
- Docstrings
- Review all the functions, redundancy, etc.

DONEs:
- Implement set, state on i64
- Implement add, etc. on i64
- Implement "Number" over i64, f64
- Implement parsing Number from string
- Change get state


*/
