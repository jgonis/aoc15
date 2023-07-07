use anyhow::Result;
use md5::{Digest, Md5};

pub fn solve(input: String) -> Result<String> {
    let mut hasher = Md5::new();
    let mut count: usize = 1;

    loop {
        let input_string = format!("{}{}", input, count);
        hasher.update(input_string);
        let result = hasher.finalize_reset();
        let hex_string: String = create_hex_string(result.as_ref());
        if &hex_string[0..6] == "000000" {
            break;
        }
        count += 1;
    }

    Ok(count.to_string())
}

fn create_hex_string(hash_result: &[u8; 16]) -> String {
    let mut result_string = String::new();
    for elem in hash_result {
        let hex_string_of_elem = format!("{:04x}", elem);
        result_string.push_str(&hex_string_of_elem[2..]);
    }
    result_string
}
