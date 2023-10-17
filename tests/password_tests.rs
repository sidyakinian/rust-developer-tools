#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn password_has_correct_length() {
        let length = 16;
        let password = generate_password(length);
        assert_eq!(password.len(), length);
    }

    #[test]
    fn password_contains_number() {
        let password = generate_password(12);
        assert!(password.chars().any(char::is_numeric));
    }

    #[test]
    fn password_contains_upper_and_lower_case() {
        let password = generate_password(12);
        let contains_uppercase = password.chars().any(|c| c.is_ascii_uppercase());
        let contains_lowercase = password.chars().any(|c| c.is_ascii_lowercase());
        assert!(contains_uppercase && contains_lowercase);
    }

    #[test]
    fn password_contains_special_char() {
        let password = generate_password(12);
        let special_chars = "!@#$%^&*()-=_+[]{}|;:,.<>?";
        assert!(password.chars().any(|c| special_chars.contains(c)));
    }
}