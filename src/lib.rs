mod email;
mod number;
mod login;

pub use number::is_valid_phone_number;
pub use login::is_valid_login;
pub use email::is_valid_email;

#[cfg(test)]
mod test_number {
    use super::is_valid_phone_number;

    #[test]
    fn test_valid_phone_numbers() {
        assert!(is_valid_phone_number(b"+1 (123) 456-7890"));
        assert!(is_valid_phone_number(b"123-456-7890"));
        assert!(is_valid_phone_number(b"123 456 7890"));
        assert!(is_valid_phone_number(b"(123) 456-7890"));
        assert!(is_valid_phone_number(b"123.456.7890"));
        assert!(is_valid_phone_number(b"+(123)-456.7890"));
    }

    #[test]
    fn test_invalid_phone_numbers() {
        assert!(!is_valid_phone_number(b"123"));
        assert!(!is_valid_phone_number(b"123-abc-7890"));
        assert!(!is_valid_phone_number(b"123) 456-7890"));
        assert!(!is_valid_phone_number(b"(123 456-7890"));
        assert!(!is_valid_phone_number(b"+1 (123) 456-7890)"));
        assert!(!is_valid_phone_number(b"++1 (123) 456-7890"));
        assert!(!is_valid_phone_number(b"123 (456) 7890"));
        assert!(!is_valid_phone_number(b"999999999999999999999"));
        assert!(!is_valid_phone_number(b""));
        assert!(!is_valid_phone_number(b"+"));
        assert!(!is_valid_phone_number(b"1  2  3 4 5 6  7 8 9 0"));
        assert!(!is_valid_phone_number(b"(123) (456) 7890"));
    }
}

#[cfg(test)]
mod test_email {
    use super::is_valid_email;
    #[test]
    fn test_valid_emails() {
        assert!(is_valid_email(b"test@example.com"));
        assert!(is_valid_email(b"user.name@domain.co"));
        assert!(is_valid_email(b"first.last@subdomain.example.org"));
        assert!(is_valid_email(b"simple@domain.com"));
        assert!(is_valid_email(b"hello@world.net"));
        assert!(is_valid_email(b"alice.bob@company.biz"));
        assert!(is_valid_email(b"contact@my-site.com"));
        assert!(is_valid_email(b"jane_doe123@service.org"));
        assert!(is_valid_email(b"johndoe@web.co.uk"));
        assert!(is_valid_email(b"info@sub.domain.edu"));
    }
    #[test]
    fn test_invalid_emails() {
        assert!(!is_valid_email(b"testexample.com"));
        assert!(!is_valid_email(b"test@examplecom"));
        assert!(!is_valid_email(b"test @example.com"));
        assert!(!is_valid_email(b"test@@example.com"));
        assert!(!is_valid_email(b"test@.example.com"));
        assert!(!is_valid_email(b"test.@example.com"));
        assert!(!is_valid_email(b"test@example.com."));
        assert!(!is_valid_email(b"t@e.c"));
        assert!(is_valid_email(b"this.is.a.longer.email@example.com"));
        assert!(!is_valid_email(b"test@dododo.comd"))
    }
}

#[cfg(test)]
mod test_login {
    use super::is_valid_login;

    #[test]
    fn test_valid_logins() {
        assert!(is_valid_login(b"testuser"));
        assert!(is_valid_login(b"user123"));
        assert!(is_valid_login(b"john_doe"));
        assert!(is_valid_login(b"jane_doe99"));
        assert!(is_valid_login(b"first.last"));
        assert!(is_valid_login(b"simple_user"));
        assert!(is_valid_login(b"unique_user1"));
        assert!(is_valid_login(b"nickname123"));
        assert!(is_valid_login(b"another.user"));
        assert!(is_valid_login(b"user_name_1"));
    }

    #[test]
    fn test_invalid_logins() {
        assert!(!is_valid_login(b""));
        assert!(!is_valid_login(b"user@123"));
        assert!(!is_valid_login(b"user!name"));
        assert!(!is_valid_login(b"john#doe"));
        assert!(!is_valid_login(b"first..last"));
        assert!(!is_valid_login(b"__username"));
        assert!(!is_valid_login(b"-username"));
        assert!(!is_valid_login(b"johndoe@"));
        assert!(!is_valid_login(b"user$123"));
    }
}