#![feature(inclusive_range_syntax)]
const BASE64: [char; 64] = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '+', '/'];

fn main() {
  println!("Hello, world!");
}

fn hextobase64(input: &str) -> Result<String, &'static str> {
  if input.len() % 2 != 0 {
    return Err("odd size hex string");
  }
  let mut sum: u32 = 0;
  let mut i = 0;
  let mut new_str = String::with_capacity(input.len());
  for c in input.bytes() {
    let val = hex_to_u8(c)? as u32;
    sum |= val;
    sum <<= 4;
    i += 1;
    if i == 6 {
      new_str.push(BASE64[((sum & 0x0fc0_0000) >> 22) as usize]);
      new_str.push(BASE64[((sum & 0x003f_0000) >> 16) as usize]);
      new_str.push(BASE64[((sum & 0x0000_fc00) >> 10) as usize]);
      new_str.push(BASE64[((sum & 0x0000_03f0) >> 4) as usize]);
      i = 0;
      sum = 0;
    }
  }
  if i != 0 {
    sum <<= 4 * (6 - i);
    new_str.push(BASE64[((sum & 0x0fc0_0000) >> 22) as usize]);
    new_str.push(BASE64[((sum & 0x003f_0000) >> 16) as usize]);
    if i == 2 {
      new_str.push('=');
    } else {
      new_str.push(BASE64[((sum & 0x0000_fc00) >> 10) as usize]);
    }
    new_str.push('=');
  }
  Ok(new_str)
}

fn fixed_xor(foo: &str, bar: &str) -> Result<String, &'static str> {
  if foo.len() != bar.len() {
    return Err("string lengths don't match");
  }
  let mut new_str = String::with_capacity(foo.len());
  for it in foo.bytes().zip(bar.bytes()) {
    let (c, x) = it;
    let c_val = hex_to_u8(c)?;
    let x_val = hex_to_u8(x)?;
    let xor_val = x_val ^ c_val;
    new_str.push(u8_to_hex_char(xor_val)?);
  }
  Ok(new_str)
}

fn best_decrypt_single_byte_xor(foo: &str) -> Result<String, &'static str> {
  let mut best_str = unsafe { std::mem::uninitialized() };
  let mut best_score = 0.0;
  for n in 0...255 {
    let new_str = decrypt_single_byte_xor(foo, n)?;
    let new_score = etaoin_shrdlu_score(&new_str);
    println!("{} {} {}", n, new_str, new_score);
    if new_score > best_score || best_score == 0.0 {
      best_score = new_score;
      best_str = new_str;
    }
  }
  Ok(best_str)
}

fn decrypt_single_byte_xor(foo: &str, key: u8) -> Result<String, &'static str> {
  let mut new_str = String::with_capacity(foo.len());
  let mut done_pair = false;
  let mut val: u8 = 0;
  for c in foo.bytes() {
    if !done_pair {
      val |= hex_to_u8(c)?;
    } else {
      val <<= 4;
      val |= hex_to_u8(c)?;
      let xor_val = key ^ val;
      new_str.push(xor_val as char);
      val = 0;
    }
    done_pair = !done_pair;
  }
  if done_pair {
    let xor_val = key ^ val;
    new_str.push(xor_val as char);
  }
  Ok(new_str)
}

fn etaoin_shrdlu_score(foo: &str) -> f64 {
  let mut score = 0.0;
  for c in foo.bytes() {
    score += match c {
      b'e' => {
        12.0
      },
      b't' => {
        11.0
      },
      b'a' => {
        10.0
      },
      b'o' => {
        9.0
      },
      b'i' => {
        8.0
      },
      b'n' => {
        7.0
      },
      b's' => {
        6.0
      },
      b'h' => {
        5.0
      },
      b'r' => {
        4.0
      },
      b'd' => {
        3.0
      },
      b'l' => {
        2.0
      },
      b'u' => {
        1.0
      },
      _ => {
        0.0
      }
    }
  }
  score /= foo.len() as f64;
  score
}

fn u8_to_hex_char(input: u8) -> Result<char, &'static str> {
  match input {
    0 => {
      Ok('0')
    },
    1 => {
      Ok('1')
    },
    2 => {
      Ok('2')
    },
    3 => {
      Ok('3')
    },
    4 => {
      Ok('4')
    },
    5 => {
      Ok('5')
    },
    6 => {
      Ok('6')
    },
    7 => {
      Ok('7')
    },
    8 => {
      Ok('8')
    },
    9 => {
      Ok('9')
    },
    10 => {
      Ok('a')
    },
    11 => {
      Ok('b')
    },
    12 => {
      Ok('c')
    },
    13 => {
      Ok('d')
    },
    14 => {
      Ok('e')
    },
    15 => {
      Ok('f')
    },
    _ => {
      Err("input out of valid hexadecimal range")
    }
  }
}

fn hex_to_u8(input: u8) -> Result<u8, &'static str> {
  match input {
    b'0' => {
      Ok(0)
    },
    b'1' => {
      Ok(1)
    },
    b'2' => {
      Ok(2)
    },
    b'3' => {
      Ok(3)
    },
    b'4' => {
      Ok(4)
    },
    b'5' => {
      Ok(5)
    },
    b'6' => {
      Ok(6)
    },
    b'7' => {
      Ok(7)
    },
    b'8' => {
      Ok(8)
    },
    b'9' => {
      Ok(9)
    },
    b'a' => {
      Ok(10)
    },
    b'b' => {
      Ok(11)
    },
    b'c' => {
      Ok(12)
    },
    b'd' => {
      Ok(13)
    },
    b'e' => {
      Ok(14)
    },
    b'f' => {
      Ok(15)
    },
    _ => {
      Err("standard in does not consist only of valid hex")
    }
  }
}

#[cfg(test)]
mod tests {
    #[test]
    fn hextobase64_exact_test() {
      use hextobase64;
      let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
      let output = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
      assert_eq!(hextobase64(input).unwrap(), output);
    }

    #[test]
    fn hextobase64_remainder_test() {
      use hextobase64;
      let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6ddd";
      let output = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t3Q==";
      assert_eq!(hextobase64(input).unwrap(), output);
    }

    #[test]
    fn fixed_xor_test() {
      use fixed_xor;
      let input1 = "1c0111001f010100061a024b53535009181c";
      let input2 = "686974207468652062756c6c277320657965";
      let output = "746865206b696420646f6e277420706c6179";
      assert_eq!(fixed_xor(input1, input2).unwrap(), output);
    }

    #[test]
    fn decrypt_single_byte_xor_test() {
      use decrypt_single_byte_xor;
      let input1 = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
      let key = 88;
      let output = "Cooking MC's like a pound of bacon";
      assert_eq!(decrypt_single_byte_xor(input1, key).unwrap(), output);
    }

    #[test]
    fn best_decrypt_single_byte_xor_test() {
      use best_decrypt_single_byte_xor;
      let input1 = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
      let output = "Cooking MC's like a pound of bacon";
      assert_eq!(best_decrypt_single_byte_xor(input1).unwrap(), output);
    }
}
