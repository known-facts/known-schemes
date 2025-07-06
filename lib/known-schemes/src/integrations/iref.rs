// This is free and unencumbered software released into the public domain.

impl From<&iref::Iri> for UriScheme {
    fn from(input: &iref::Iri) -> Self {
        Self::from_str(input.scheme().as_str()).unwrap() // infallible
    }
}

impl From<&iref::IriBuf> for UriScheme {
    fn from(input: &iref::IriBuf) -> Self {
        Self::from_str(input.scheme().as_str()).unwrap() // infallible
    }
}

impl From<&iref::Uri> for UriScheme {
    fn from(input: &iref::Uri) -> Self {
        Self::from_str(input.scheme().as_str()).unwrap() // infallible
    }
}

impl From<&iref::UriBuf> for UriScheme {
    fn from(input: &iref::UriBuf) -> Self {
        Self::from_str(input.scheme().as_str()).unwrap() // infallible
    }
}
