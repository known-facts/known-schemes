// This is free and unencumbered software released into the public domain.

impl From<&url::Url> for UriScheme {
    fn from(input: &url::Url) -> Self {
        Self::from_str(input.scheme()).unwrap() // infallible
    }
}
