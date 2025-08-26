#[cfg(test)]
mod tests {
    use afaf_rest_rust::core::rest::handler::validator::is_valid_email;

    #[test]
    fn test_valid_emails() {
        assert!(is_valid_email("user@example.com"));
        assert!(is_valid_email("user.name@example.co.uk"));
        assert!(is_valid_email("user+tag@example.com"));
        assert!(is_valid_email("123@example.com"));
    }

    #[test]
    fn test_invalid_emails() {
        assert!(!is_valid_email(""));
        assert!(!is_valid_email("invalid.email"));
        assert!(!is_valid_email("user@"));
        assert!(!is_valid_email("@example.com"));
        assert!(!is_valid_email("user@.com"));
        assert!(!is_valid_email("user space@example.com"));
        assert!(!is_valid_email("user@example..com"));
    }
} 