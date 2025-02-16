#[inline(always)]
pub fn is_valid_phone_number(number: &[u8]) -> bool {
    let len: i8 = match number.len().try_into() {
        Ok(len) if len >= 5 => len,
        _ => return false,
    };

    unsafe {
        let mut ptr: *const u8 = number.as_ptr();
        let end: *const u8 = ptr.add(len as usize);
        let mut has_digits: bool = false;
        let mut open_parentheses: i8 = 0;
        let mut first_plus: bool = false;
        let mut code_country: u8 = 0;
        let mut count_read: u8 = 0;
        let mut count_read_end: bool = false;
        let mut count_digit: u8 = 0;
        
        let first_ptr = *ptr;
        if first_ptr == b'+' {
            ptr = ptr.add(1);
            first_plus = true;
        }
        else if !(first_ptr >= b'0' && first_ptr <= b'9' || first_ptr == b'(') {
            return false;
        }

        
        while ptr < end {
            let b: u8 = *ptr;
            let next = *ptr.add(1);

            match b {
                b'+' => {
                    if first_plus { return false; }
                }
                b'0'..=b'9' => {
                    count_read += 1;
                    if count_read > 5 && !first_plus || count_read > 6 && first_plus {
                        count_read_end = true;
                    }
                    has_digits = true;
                    count_digit += 1;
                }
                b'(' => {
                    if open_parentheses > 0 { return false; }
                    open_parentheses += 1;
                    has_digits = false;
                }
                b')' => {
                    if open_parentheses == 0 || (count_read < 6 && count_read_end) || (count_read > 5 && !first_plus || count_read > 6 && first_plus) || code_country > 1 {
                        return false;
                    }
                    open_parentheses -= 1;
                    has_digits = false;
                    code_country += 1;
                }
                b'-' | b'.' | b' ' => {
                    if next == b { return false; }
                    has_digits = false;
                },
                _ => return false,
            }
            ptr = ptr.add(1);
        }

        has_digits && open_parentheses == 0 && (4..15).contains(&count_digit)
    }
}