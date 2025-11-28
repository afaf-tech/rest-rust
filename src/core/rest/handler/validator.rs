use regex::Regex;

pub fn is_valid_email(email: &str) -> bool {
    // Reject emails with consecutive dots
    if email.contains("..") {
        return false;
    }
    let email_regex = Regex::new(r"^[^\s@]+@[^\s@]+\.[^\s@]+$").unwrap();
    email_regex.is_match(email)
}
