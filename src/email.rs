#[inline(always)]
pub fn is_valid_email(email: &[u8]) -> bool {
    let len: i8 = match email.len().try_into() {
        Ok(len) => len,
        _ => return false,
    };

    if len <= 5 {
        return false;
    }

    unsafe {
        let ptr: *const u8 = email.as_ptr();
        let mut at_pos: i8 = -1;
        let mut dot_pos: i8 = -1;
        let mut is_domain: bool = false;
        let mut last_char_was_dash = false;

        for i in 0..len {
            let c: u8 = *ptr.add(i as usize);

            if c == b'@' {
                if i > 0 && *ptr.add(i as usize - 1) == b'.' {
                    return false;
                }

                is_domain = true;
                if at_pos != -1 {
                    return false;
                }

                let next_ptr = *ptr.add(i as usize + 1);
                if !(
                    (next_ptr >= b'a' && next_ptr <= b'z')
                        || (next_ptr >= b'A' && next_ptr <= b'Z')
                        || (next_ptr >= b'0' && next_ptr <= b'9')
                ) {
                    return false;
                }

                at_pos = i;
            } else if c == b'.' {
                dot_pos = i;
                last_char_was_dash = false;
            } else if c <= b' ' {
                return false;
            } else if !(
                (c >= b'a' && c <= b'z')
                    || (c >= b'A' && c <= b'Z')
                    || (c >= b'0' && c <= b'9')
                    || c == b'-'
            ) && !is_domain && *ptr.add(1) == b'@'
            {
                return false;
            }

            if is_domain {
                if i == at_pos + 1 && c == b'.' {
                    return false;
                }
                if i == len - 1 && c == b'.' {
                    return false;
                }
                if c == b'-' {
                    if i == at_pos + 1 || *ptr.add(i as usize - 1) == b'.' || *ptr.add(i as usize + 1) == b'.' {
                        return false;
                    }
                    if last_char_was_dash {
                        return false;
                    }
                    last_char_was_dash = true;
                } else {
                    last_char_was_dash = false;
                }
            }
        }

        if at_pos == -1 || dot_pos == -1 {
            return false;
        }

        let domain_part = &email[(at_pos + 1) as usize..];
        let domain_parts: Vec<&[u8]> = domain_part.split(|&x| x == b'.').collect();

        if domain_parts.is_empty() {
            return false;
        }

        if let Some(last_part) = domain_parts.last() {
            let after_dot_len = last_part.len();
            if after_dot_len < 2 || after_dot_len > 3 {
                return false;
            }
        }

        if domain_part.last() == Some(&b'.') {
            return false;
        }

        for part in domain_parts {
            if part.is_empty() || !part.iter().all(|&c| (c >= b'a' && c <= b'z') || (c >= b'A' && c <= b'Z') || (c >= b'0' && c <= b'9') || c == b'-') {
                return false;
            }
        }

        if at_pos == 0 || dot_pos <= at_pos + 1 || dot_pos == (len - 1) {
            return false;
        }
    }
    true
}
