// This is free and unencumbered software released into the public domain.

impl<T: core::ops::Deref<Target = str>> From<&oxiri::Iri<T>> for UriScheme {
    fn from(input: &oxiri::Iri<T>) -> Self {
        Self::from_str(input.scheme()).unwrap() // infallible
    }
}

impl<T: core::ops::Deref<Target = str>> TryFrom<&oxiri::IriRef<T>> for UriScheme {
    type Error = ();

    fn try_from(input: &oxiri::IriRef<T>) -> Result<Self, Self::Error> {
        input
            .scheme()
            .map(|scheme| Self::from_str(scheme).unwrap()) // infallible
            .ok_or(())
    }
}
