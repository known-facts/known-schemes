// This is free and unencumbered software released into the public domain.

impl From<&magnet_url::Magnet> for UriScheme {
    fn from(_input: &magnet_url::Magnet) -> Self {
        Self::Magnet
    }
}
