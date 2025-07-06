// This is free and unencumbered software released into the public domain.

impl From<&uriparse::URI<'_>> for UriScheme {
    fn from(input: &uriparse::URI<'_>) -> Self {
        Self::from_str(input.scheme().as_str()).unwrap() // infallible
    }
}
