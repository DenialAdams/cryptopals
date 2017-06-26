use std::io;

const BASE64: [char; 64] = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '+', '/'];

fn main() {
  let mut input = String::new();
  io::stdin().read_line(&mut input).unwrap();
  input.pop(); // Remove newline
}

fn hextobase64(input: String) -> Result<String, &'static str> {
  let mut sum: u16 = 0;
  let mut i = 0;
  // This is reserving too much, TODO
  let mut new_str = String::with_capacity(input.len());
  for c in input.chars() {
    let val = hex_to_u16(c)?;
    sum |= val;
    sum <<= 4;
    i += 1;
    if i == 3 {
      new_str.push(BASE64[((sum & 0b1111_1100_0000_0000) >> 10) as usize]);
      new_str.push(BASE64[((sum & 0b0000_0011_1111_0000) >> 4) as usize]);
      i = 0;
      sum = 0;
    }
  }
  if i != 0 {
    sum <<= 4 * (3 - i);
    new_str.push(BASE64[((sum & 0b1111_1100_0000_0000) >> 10) as usize]);
    new_str.push(BASE64[((sum & 0b0000_0011_1111_0000) >> 4) as usize]);
  }
  Ok(new_str)
}

fn hex_to_u16(input: char) -> Result<u16, &'static str> {
  match input {
    '0' => {
      Ok(0)
    },
    '1' => {
      Ok(1)
    },
    '2' => {
      Ok(2)
    },
    '3' => {
      Ok(3)
    },
    '4' => {
      Ok(4)
    },
    '5' => {
      Ok(5)
    },
    '6' => {
      Ok(6)
    },
    '7' => {
      Ok(7)
    },
    '8' => {
      Ok(8)
    },
    '9' => {
      Ok(9)
    },
    'a' => {
      Ok(10)
    },
    'b' => {
      Ok(11)
    },
    'c' => {
      Ok(12)
    },
    'd' => {
      Ok(13)
    },
    'e' => {
      Ok(14)
    },
    'f' => {
      Ok(15)
    },
    _ => {
      Err("standard in does not consist only of valid hex")
    }
  }
}

#[cfg(test)]
mod tests {
    use hextobase64;

    #[test]
    fn hextobase64_test() {
      let input = String::from("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d");
      let output = String::from("SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t");
      assert_eq!(hextobase64(input).unwrap(), output);
    }
}
