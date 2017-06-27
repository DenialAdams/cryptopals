#![feature(inclusive_range_syntax)]

use std::fs::File;
use std::io::{self, BufReader, BufRead};

const BASE64: [char; 64] = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '+', '/'];
const HEX:    [&'static str; 256] = ["00", "01", "02", "03", "04", "05", "06", "07", "08", "09", "0a", "0b", "0c", "0d", "0e", "0f", "10", "11", "12", "13", "14", "15", "16", "17", "18", "19", "1a", "1b", "1c", "1d", "1e", "1f", "20", "21", "22", "23", "24", "25", "26", "27", "28", "29", "2a", "2b", "2c", "2d", "2e", "2f", "30", "31", "32", "33", "34", "35", "36", "37", "38", "39", "3a", "3b", "3c", "3d", "3e", "3f", "40", "41", "42", "43", "44", "45", "46", "47", "48", "49", "4a", "4b", "4c", "4d", "4e", "4f", "50", "51", "52", "53", "54", "55", "56", "57", "58", "59", "5a", "5b", "5c", "5d", "5e", "5f", "60", "61", "62", "63", "64", "65", "66", "67", "68", "69", "6a", "6b", "6c", "6d", "6e", "6f", "70", "71", "72", "73", "74", "75", "76", "77", "78", "79", "7a", "7b", "7c", "7d", "7e", "7f", "80", "81", "82", "83", "84", "85", "86", "87", "88", "89", "8a", "8b", "8c", "8d", "8e", "8f", "90", "91", "92", "93", "94", "95", "96", "97", "98", "99", "9a", "9b", "9c", "9d", "9e", "9f", "a0", "a1", "a2", "a3", "a4", "a5", "a6", "a7", "a8", "a9", "aa", "ab", "ac", "ad", "ae", "af", "b0", "b1", "b2", "b3", "b4", "b5", "b6", "b7", "b8", "b9", "ba", "bb", "bc", "bd", "be", "bf", "c0", "c1", "c2", "c3", "c4", "c5", "c6", "c7", "c8", "c9", "ca", "cb", "cc", "cd", "ce", "cf", "d0", "d1", "d2", "d3", "d4", "d5", "d6", "d7", "d8", "d9", "da", "db", "dc", "dd", "de", "df", "e0", "e1", "e2", "e3", "e4", "e5", "e6", "e7", "e8", "e9", "ea", "eb", "ec", "ed", "ee", "ef", "f0", "f1", "f2", "f3", "f4", "f5", "f6", "f7", "f8", "f9", "fa", "fb", "fc", "fd", "fe", "ff"];

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

fn best_decrypt_single_byte_xor_file(file_name: &str) -> Result<String, &'static str> {
  let f = File::open(file_name).unwrap();
  let f = BufReader::new(f);

  let mut best_str = String::from("test");
  let mut best_score = 0.0;
  for line in f.lines() {
    let (new_str, new_score) = best_decrypt_single_byte_xor(&line.unwrap()).unwrap();
    if new_score > best_score || best_score == 0.0 {
      best_score = new_score;
      best_str = new_str;
    }
  }

  Ok(best_str)
}

fn repeating_key_xor(foo: &str, key: &str) -> Result<String, &'static str> {
  let mut new_str = String::with_capacity(foo.len());
  for it in foo.bytes().zip(key.bytes().cycle()) {
    let (c, x) = it;
    let xor_val = c ^ x;
    new_str.push_str(HEX[xor_val as usize]);
    println!("{} {} {:02x}", c as char, x as char, xor_val);
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

fn best_decrypt_single_byte_xor(foo: &str) -> Result<(String, f64), &'static str> {
  let mut best_str = String::from("test");
  let mut best_score = 0.0;
  for n in 0...255 {
    let new_str = decrypt_single_byte_xor(foo, n)?;
    let new_score = etaoin_shrdlu_score(&new_str);
    if new_score > best_score || best_score == 0.0 {
      best_score = new_score;
      best_str = new_str;
    }
  }
  Ok((best_str, best_score))
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
  // TODO UNTESTED
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
      assert_eq!(best_decrypt_single_byte_xor(input1).unwrap().0, output);
    }

    #[test]
    fn best_decrypt_single_byte_xor_file_test() {
      use best_decrypt_single_byte_xor_file;
      let output = "Now that the party is jumping\n";
      assert_eq!(best_decrypt_single_byte_xor_file("4.txt").unwrap(), output);
    }

    #[test]
    fn repeating_key_xor_test() {
      use repeating_key_xor;
      let input = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
      let key = "ICE";
      let output = "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f";
      assert_eq!(repeating_key_xor(input, key).unwrap(), output);
    }
}
