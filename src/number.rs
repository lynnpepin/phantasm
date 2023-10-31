use std::fmt;
use std::ops::{Add, Sub, Mul, Div, BitAnd, BitOr, BitXor, Not, Shl, Shr};

// Dynamic-typed number
#[derive(Debug, Clone)]
pub enum Number {
    F64(f64),
    I64(i64),
}


impl fmt::Display for Number {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      match self {
          Number::I64(val) => val.fmt(f),
          Number::F64(val) => val.fmt(f),
      }
  }
}

// todo: n identical `impl` for n types. i must learn macros
impl From<i64> for Number {
  fn from(item: i64) -> Self {
      Number::I64(item)
  }
}

impl From<f64> for Number {
  fn from(item: f64) -> Self {
      Number::F64(item)
  }
}

impl Add for Number {
  type Output = Number;
  fn add(self, other: Number) -> Number {
      match (self, other) {
          // todo: 2^n lines for n types, figure out a better way
          (Number::I64(a), Number::I64(b)) => Number::I64(a + b),
          (Number::I64(a), Number::F64(b)) => Number::F64(a as f64 + b),
          (Number::F64(a), Number::I64(b)) => Number::F64(a + b as f64),
          (Number::F64(a), Number::F64(b)) => Number::F64(a + b),
      }
  }
}

impl Sub for Number {
  type Output = Number;
  fn sub(self, other: Number) -> Number {
      match (self, other) {
          // Could probably re-use add, but
          // 1. I don't know how to
          // 2. I don't know if (a - b) == (a + -b) will be true in all primitives when we add arb prec
          (Number::I64(a), Number::I64(b)) => Number::I64(a - b),
          (Number::I64(a), Number::F64(b)) => Number::F64(a as f64 - b),
          (Number::F64(a), Number::I64(b)) => Number::F64(a - b as f64),
          (Number::F64(a), Number::F64(b)) => Number::F64(a - b),
      }
  }
}

impl Mul for Number {
  type Output = Number;
  fn mul(self, other: Number) -> Number {
      match (self, other) {
          (Number::I64(a), Number::I64(b)) => Number::I64(a * b),
          (Number::I64(a), Number::F64(b)) => Number::F64(a as f64 * b),
          (Number::F64(a), Number::I64(b)) => Number::F64(a * b as f64),
          (Number::F64(a), Number::F64(b)) => Number::F64(a * b),
      }
  }
}

impl Div for Number {
  type Output = Number;
  fn div(self, other: Number) -> Number {
      // Could be a oneliner by a better rustacean
      // and I am so sleepy
      match (self, other) {
          (Number::I64(a), Number::I64(b)) => Number::F64(a as f64 / b as f64),
          (Number::I64(a), Number::F64(b)) => Number::F64(a as f64 / b),
          (Number::F64(a), Number::I64(b)) => Number::F64(a / b as f64),
          (Number::F64(a), Number::F64(b)) => Number::F64(a / b),
      }
  }
}

impl BitAnd for Number {
  type Output = Number;
  fn bitand(self, other: Number) -> Number {
      match (self, other) {
          (Number::I64(a), Number::I64(b)) => Number::I64(a & b),
          // i64 as u64: -1 becomes 2^64 - 1, not panic
          (Number::I64(a), Number::F64(b)) => Number::I64((a as u64    & b.to_bits()) as i64),
          (Number::F64(a), Number::I64(b)) => Number::I64((a.to_bits() & b as u64) as i64),
          (Number::F64(a), Number::F64(b)) => Number::I64((a.to_bits() & b.to_bits()) as i64),
      }
  }
}

impl BitOr for Number {
  type Output = Number;
  fn bitor(self, other: Number) -> Number {
      match (self, other) {
          (Number::I64(a), Number::I64(b)) => Number::I64(a | b),
          (Number::I64(a), Number::F64(b)) => Number::I64((a as u64    | b.to_bits()) as i64),
          (Number::F64(a), Number::I64(b)) => Number::I64((a.to_bits() | b as u64) as i64),
          (Number::F64(a), Number::F64(b)) => Number::I64((a.to_bits() | b.to_bits()) as i64),
      }
  }
}

impl BitXor for Number {
  type Output = Number;
  fn bitxor(self, other: Number) -> Number {
      match (self, other) {
          (Number::I64(a), Number::I64(b)) => Number::I64(a ^ b),
          (Number::I64(a), Number::F64(b)) => Number::I64((a as u64    ^ b.to_bits()) as i64),
          (Number::F64(a), Number::I64(b)) => Number::I64((a.to_bits() ^ b as u64) as i64),
          (Number::F64(a), Number::F64(b)) => Number::I64((a.to_bits() ^ b.to_bits()) as i64),
      }
  }
}

impl Not for Number {
  type Output = Number;
  fn not(self) -> Number {
      match (self) {
          (Number::I64(a)) => Number::I64(!a),
          (Number::F64(a)) => Number::I64(!a.to_bits() as i64),
      }
  }
}

impl From<&str> for Number {
  fn from(ss: &str) -> Self {
      if let Ok(vv) = ss.parse::<i64>() {
          Number::I64(vv)
      } else if let Ok(vv) = ss.parse::<f64>() {
          Number::F64(vv)
      } else {
          // todo: do something other than panic
          panic!("yeah i didn't think that would work either");
      }
  }
}

impl From<String> for Number {
  fn from(ss: String) -> Self {
      if let Ok(vv) = ss.parse::<i64>() {
          Number::I64(vv)
      } else if let Ok(vv) = ss.parse::<f64>() {
          Number::F64(vv)
      } else {
          // todo: do something other than panic
          panic!("yeah i didn't think that would work either");
      }
  }
}