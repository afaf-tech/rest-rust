use regex::Regex;

pub fn is_valid_email(email: &str) -> bool {
    // Email validation regex that allows common email formats
    // Matches: local-part@domain.tld
    // - local-part: alphanumeric, dots, hyphens, underscores, plus signs
    // - domain: alphanumeric, hyphens, dots (with proper TLD)
    let email_regex = Regex::new(
        r"^[a-zA-Z0-9]([a-zA-Z0-9._+-]*[a-zA-Z0-9])?@[a-zA-Z0-9]([a-zA-Z0-9-]*[a-zA-Z0-9])?(\.[a-zA-Z0-9]([a-zA-Z0-9-]*[a-zA-Z0-9])?)*\.[a-zA-Z]{2,}$"
    ).unwrap();

    // Additional checks to ensure no consecutive dots and basic format requirements
    if email.contains("..")
        || email.starts_with('.')
        || email.ends_with('.')
        || email.starts_with('@')
        || email.ends_with('@')
        || !email.contains('@')
        || email.chars().filter(|&c| c == '@').count() != 1
    {
        return false;
    }

    email_regex.is_match(email)
}
