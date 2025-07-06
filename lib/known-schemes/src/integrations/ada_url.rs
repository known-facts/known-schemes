// This is free and unencumbered software released into the public domain.

impl From<&ada_url::Url> for UriScheme {
    fn from(input: &ada_url::Url) -> Self {
        (&input.scheme_type())
            .try_into()
            .unwrap_or_else(|_| Self::Other(input.protocol().strip_suffix(':').unwrap().into()))
    }
}

impl TryFrom<&ada_url::SchemeType> for UriScheme {
    type Error = ();

    fn try_from(input: &ada_url::SchemeType) -> Result<Self, Self::Error> {
        use ada_url::SchemeType;
        Ok(match input {
            SchemeType::Http => Self::Http,
            SchemeType::NotSpecial => return Err(()),
            SchemeType::Https => Self::Https,
            SchemeType::Ws => Self::Ws,
            SchemeType::Ftp => Self::Ftp,
            SchemeType::Wss => Self::Wss,
            SchemeType::File => Self::File,
        })
    }
}
