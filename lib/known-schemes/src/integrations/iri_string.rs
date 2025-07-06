// This is free and unencumbered software released into the public domain.

impl From<&iri_string::types::IriStr> for UriScheme {
    fn from(input: &iri_string::types::IriStr) -> Self {
        Self::from_str(input.scheme_str()).unwrap() // infallible
    }
}

impl From<&iri_string::types::IriAbsoluteStr> for UriScheme {
    fn from(input: &iri_string::types::IriAbsoluteStr) -> Self {
        Self::from_str(input.scheme_str()).unwrap() // infallible
    }
}

impl TryFrom<&iri_string::types::IriReferenceStr> for UriScheme {
    type Error = ();

    fn try_from(input: &iri_string::types::IriReferenceStr) -> Result<Self, Self::Error> {
        input
            .scheme_str()
            .map(|scheme| Self::from_str(scheme).unwrap()) // infallible
            .ok_or(())
    }
}

impl From<&iri_string::types::UriStr> for UriScheme {
    fn from(input: &iri_string::types::UriStr) -> Self {
        Self::from_str(input.scheme_str()).unwrap() // infallible
    }
}

impl From<&iri_string::types::UriAbsoluteStr> for UriScheme {
    fn from(input: &iri_string::types::UriAbsoluteStr) -> Self {
        Self::from_str(input.scheme_str()).unwrap() // infallible
    }
}

impl TryFrom<&iri_string::types::UriReferenceStr> for UriScheme {
    type Error = ();

    fn try_from(input: &iri_string::types::UriReferenceStr) -> Result<Self, Self::Error> {
        input
            .scheme_str()
            .map(|scheme| Self::from_str(scheme).unwrap()) // infallible
            .ok_or(())
    }
}
