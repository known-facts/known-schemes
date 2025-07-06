// This is free and unencumbered software released into the public domain.

impl From<&email_address::EmailAddress> for UriScheme {
    fn from(_input: &email_address::EmailAddress) -> Self {
        Self::Mailto
    }
}
