#[inline(always)]
pub fn is_valid_login(login: &[u8]) -> bool {
    let len: i8 = match login.len().try_into() {
        Ok(len) => len,
        _ => return false
    };
    if len < 3 {
        return false;
    }

    unsafe {
        let mut ptr: *const u8 = login.as_ptr();
        let end: *const u8 = ptr.add(len as usize);

        let first: u8 = *ptr;
        if !(
                (first >= b'a' && first <=b'z') ||
                (first >= b'A' && first <= b'Z') ||
                (first >= b'0' && first <= b'9')
            ) {
                return false;
            }
        let mut prev: u8 = first;
        ptr = ptr.add(1);

        while ptr < end {
            let b: u8 = *ptr;
            if !(
                    (b >= b'a' && b <= b'z') ||
                    (b >= b'A' && b <= b'Z') ||
                    (b >= b'0' && b <= b'9') ||
                    b == b'_' || b == b'-' || b == b'.'
                ) {
                    return false;
                }
            if (b == b'.' || b == b'-' || b == b'_') && prev == b {
                return false;
            }
            prev = b;
            ptr = ptr.add(1);
        }
    }
    true
}