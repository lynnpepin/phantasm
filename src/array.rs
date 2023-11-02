use std::collections::HashMap;
use crate::number::Number;
use std::num::ParseIntError;

// This is all very bad and should be cleaned up and fixed
// A lot of functionality is duplicated and unclear

// Ideas:
/*
We want to be able to do things like
add z.4 x.i 3     == z[4] = x[i] + 3
sub z   y.0 z     == z[0] = y.0  + 3




get_key_ii(token) -> Option<(key, Option<index>)>:
  - Use this to parse each string token for indexing
  - "z.4"  -> Some("z", Some(4))
  - "z"    -> Some("z", None)
    Interpreted as z[0] in some/most cases
  - "4"    -> None()

get_mut_ref_val(state, key, index) -> Option<&Vec<Number>>
  - Use this with Some returned from get_key(token)
  - Gets state[key][index] which can be updated
  - None() if key does not exist

*/

// Parse comma-separated string to numbers
pub fn parse_number(ss: &String) -> Result<Number, 0> {
  ss.parse::<i64>().map(Number::I64)
    .or(ss.parse::<f64>().map(Number::F64))
}

pub fn parse_numbers(ss: &String) -> Result<Vec<Number>, ()> {
  ss.split(",").map(|token| parse_number(token)).collect()
}


// Helper func for state[key][idx]
pub fn get_value_from_state(
  state: &mut HashMap<String, Vec<Number>>,
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
pub fn get_value(
  token: String, state: &mut HashMap<String, Vec<Number>>
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
          //println!("get_value({}, {:?}): Could not parse token", token, state);
          Err(())
        }
      }
    } else {
      get_value_from_state(state, token, None)
    }
  }
}

// state[key][index] = value
// TODO: Update this to expand array
pub fn set_value_in_vec_in_state(
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
pub fn set_value_in_state(
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
  //println!("set_value_in_state({}, {}, {:?})", key, index, values);
  match state.get_mut(&key) {
    // state[key] exists; update it
    Some(vec) => {
      // Resize if it's too small
      if index + values.len() > vec.len() {
        vec.resize(
          index + values.len(),
          Number::I64(0)
        )
      }
      // overwrite vec[idx:idx+values.len()] with values
      for (ii, vv) in values.into_iter().enumerate() {
        vec[index + ii] = vv;
      }
      
      Ok(())
    },
    // state[key] doesn't exist; create it
    None => {
      // todo: This duplicates above code, move to a separate function?
      let mut vec: Vec<Number> = Vec::new();
      if index + values.len() > vec.len() {
        vec.resize(
          index + values.len(),
          Number::I64(0)
        )
      }
      // overwrite vec[idx:idx+values.len()] with values
      for (ii, vv) in values.into_iter().enumerate() {
        vec[index + ii] = vv;
      }
      state.insert(
        key.to_string(),
        vec
      );
      Ok(())
    }
  }
}


// Set state[token] = [val1, val2, ...] or state[key][idx] = [val1, val2, ...],
pub fn set_value_from_string(
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
  match parse_numbers(&value) {
    Ok(values) => {
      //println!(".. values: {:?}", values);
      match token.split_once('.') {
        Some((key, index_str)) => {
          //println!(".. .. key: {}, index_str: {}", key, index_str);
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
              //println!(
              //  "set_value({:?}, {}, {}): Could not parse {} in set_value call",
              //  state, token, value, index_str
              //);
              Err("Could not parse index in set_value call")
            }
            
          }
        },
        // Handle the cases 2, 3
        None => {
          match values.len() {
            0 => {
              //println!("Could not parse empty values");
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
      // Can't parse `token` as a number
      //println!("set_value({}, {}): Could not parse values", token, value);
      Err("Could not parse values in set_value call")
    },
  }
}

// Parse idx from token, then call set_value_in_vec_in_state
pub fn set_value(
  state: &mut HashMap<String, Vec<Number>>,
  token: String,
  value: Number,
) -> Result<(), &str> {
  match token.split_once('.') {
    Some((key, index_str)) => {
      //println!(".. .. key: {}, index_str: {}", key, index_str);
      match index_str.parse::<usize>() {
        Ok(idx) => {
          set_value_in_state(
            state,
            key.to_string(),
            Some(idx),
            vec![value]
          );
          Ok(())
        },
        Err(ParseIntError) => {Err("Could not parse index in set_value call")}
      }
    },
    None => {
      // k is not indexed; overwrite entire k with value
      state.insert(
        token.to_string(),
        vec![value]
      );
      Ok(())
    }
  }
}


/*
parse_numbers(&String) -> Vec<Number>
get_value_from_state(state, key, idx=0) -> Number
get_value(token, state) -> Number
set_value_in_state(state, key, idx=0, values); // state[key][idx:] = values
set_value_from_string(state, token, value); value is string
set_value(state, token, value); 

// TODO:
1. Clean this up! A lot of code is duplicated
*/
