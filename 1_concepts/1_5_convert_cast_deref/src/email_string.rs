use std::borrow::Borrow;
use std::str::FromStr;
use regex::Regex;

const EMAIL_RE: &str = r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})";

struct EmailString(String);

impl FromStr for EmailString {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let email_regex = Regex::new(EMAIL_RE).unwrap();
        if email_regex.is_match(s) {
            Ok(EmailString(s.to_string()))
        } else {
            Err("Not a valid email")
        }
    }
}

impl AsRef<str> for EmailString {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Borrow <str> for EmailString {
    fn borrow(&self) -> &str {
        &self.0[..]
    }
}

fn fn_that_use_str(s: &str) {
    println!("fn_that_use_str: {}", s);
}

#[cfg(test)]
mod tests {
    use crate::email_string::{EmailString, fn_that_use_str};
    use std::str::FromStr;
    use std::borrow::Borrow;

    #[test]
    fn test_email_string() {
        let valid_emails = vec![
                                "test@gmail.com",
                                "_bar@bar.com",
                                "foo_@bar.com",
        ];

        for email in valid_emails {
            let email_string = EmailString::from_str(email);
            assert!(email_string.is_ok());
        }

        let invalid_emails = vec![
                                "test@gmail",
                                "test@.com",
                                "test@com",
                                "test@.com",
                                "test@com",
                                "test@com.",
                                "test@com.a",
                                ];

        for email in invalid_emails {
            let email_string = EmailString::from_str(email);
            assert!(email_string.is_err());
        }
    }

    #[test]
    fn test_email_string_as_ref() {
        let email_string = EmailString::from_str("test@gmail.com").unwrap();
        assert_eq!(email_string.as_ref(), "test@gmail.com");

        fn_that_use_str(email_string.as_ref());
    }

    #[test]
    fn test_email_string_borrow() {
        let email_string = EmailString::from_str("test@gmail.com").unwrap();
        let email_string_ref: &str = email_string.borrow();
        assert_eq!(email_string_ref, "test@gmail.com");

        fn_that_use_str(email_string.borrow());
    }
}
