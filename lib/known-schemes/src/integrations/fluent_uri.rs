// This is free and unencumbered software released into the public domain.

impl From<&fluent_uri::Uri<&str>> for UriScheme {
    fn from(input: &fluent_uri::Uri<&str>) -> Self {
        Self::from_str(input.scheme().as_str()).unwrap() // infallible
    }
}

impl From<&fluent_uri::Uri<String>> for UriScheme {
    fn from(input: &fluent_uri::Uri<String>) -> Self {
        Self::from_str(input.scheme().as_str()).unwrap() // infallible
    }
}
