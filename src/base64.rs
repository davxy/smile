use crate::bindings::*;

pub fn encode(input: &[u8]) -> String {
    let len;
    let mut output: Vec<u8> = vec![0; 2*input.len()];
    unsafe {
        let out_p = &mut output[0] as *mut u8 as *mut i8;
        let in_p = &input[0] as *const u8 as *const i8;
        len = cry_base64_encode(out_p, in_p, input.len() as u64);
    }
    output.resize(len as usize, 0);
    let output = String::from_utf8(output).unwrap_or(String::new());
    output
}

pub fn decode(input: &str) -> Vec<u8> {
    let len;
    let mut output = vec![0; input.len()];
    unsafe {
        let out_p = &mut output[0] as *mut u8 as *mut i8;
        let in_p = &input.as_bytes()[0] as *const u8 as *const i8;
        len = cry_base64_decode(out_p, in_p, input.len() as u64);
    }
    output.resize(len as usize, 0);
    output
}

#[cfg(test)]
mod test {

    #[test]
    fn encode() {
        let input = "HelloWorld";

        let output = super::encode(input.as_bytes());

        assert_eq!("SGVsbG9Xb3JsZA==", output);
    }

    #[test]
    fn decode() {
        let input = "SGVsbG9Xb3JsZA==";

        let output = super::decode(input);

        assert_eq!("HelloWorld", String::from_utf8_lossy(&output));
    }
}
