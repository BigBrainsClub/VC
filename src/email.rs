#[inline(always)]
pub fn is_valid_email(email: &[u8]) -> bool {
    let len = email.len();
    if len <= 5 {
        return false;
    }

    unsafe {
        let ptr = email.as_ptr();
        let mut at_pos: i8 = -1;
        let mut dot_pos: i8 = -1;
        let mut last_char_was_dash = false;

        for i in 0..len as i8 {
            let c = *ptr.add(i as usize);

            match c {
                b'@' => {
                    if at_pos != -1 || i == 0 || *ptr.add(i as usize - 1) == b'.' {
                        return false;
                    }
                    at_pos = i;
                    if i + 1 >= len as i8 {
                        return false;
                    }
                    let next_c = *ptr.add(i as usize + 1);
                    if !next_c.is_ascii_alphanumeric() {
                        return false;
                    }
                }
                b'.' => {
                    if i == 0 || i == len as i8 - 1 || (at_pos != -1 && i == at_pos + 1) {
                        return false;
                    }
                    dot_pos = i;
                    last_char_was_dash = false;
                }
                b'-' => {
                    if i == 0 || i == len as i8 - 1 || *ptr.add(i as usize - 1) == b'.' || *ptr.add(i as usize + 1) == b'.' {
                        return false;
                    }
                    if last_char_was_dash {
                        return false;
                    }
                    last_char_was_dash = true;
                }
                0..=32 => return false,
                _ => last_char_was_dash = false,
            }
        }

        if at_pos == -1 || dot_pos <= at_pos + 1 {
            return false;
        }

        let mut part_len = 0;
        let mut domain_parts = 0;

        for i in at_pos as usize + 1..len {
            let c = *ptr.add(i);
            if c == b'.' {
                if part_len == 0 {
                    return false;
                }
                domain_parts += 1;
                part_len = 0;
            } else {
                part_len += 1;
            }
        }

        if domain_parts == 0 || part_len < 2 || part_len > 3 {
            return false;
        }
    }
    true
}
