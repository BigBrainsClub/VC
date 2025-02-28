#[inline(always)]
pub fn is_valid_login(login: &[u8]) -> bool {
    let len = login.len();
    if len < 3 {
        return false;
    }

    unsafe {
        let mut ptr = login.as_ptr();
        let first = *ptr;
        if !first.is_ascii_alphanumeric() {
            return false;
        }

        let mut prev = first;
        ptr = ptr.add(1);
        let last_ptr = ptr.add(len - 1);

        while ptr <= last_ptr {
            let b = *ptr;

            if !(b.is_ascii_alphanumeric() || b == b'_' || b == b'-' || b == b'.') {
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
