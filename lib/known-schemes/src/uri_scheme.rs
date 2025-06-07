// This is free and unencumbered software released into the public domain.

use super::prelude::{FromStr, String, fmt};

/// An enumerated URI scheme.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum UriScheme {
    Data,
    File,
    Ftp,
    Ftps,
    Git,
    Http,
    Https,
    Ipfs,
    Scp,
    Stdin,
    Other(String),
}

impl UriScheme {
    pub fn as_str(&self) -> &str {
        use UriScheme::*;
        match self {
            Data => "data",
            File => "file",
            Ftp => "ftp",
            Ftps => "ftps",
            Git => "git",
            Http => "http",
            Https => "https",
            Ipfs => "ipfs",
            Scp => "scp",
            Stdin => "stdin",
            Other(scheme) => scheme.as_str(),
        }
    }

    pub fn to_port(&self) -> Option<u16> {
        use UriScheme::*;
        Some(match self {
            Ftp => 21,
            Ftps => 990,
            Git => 9418,
            Http => 80,
            Https => 443,
            Scp => 22,
            Data | File | Ipfs | Stdin | Other(_) => return None,
        })
    }
}

impl FromStr for UriScheme {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        use UriScheme::*;
        Ok(match input {
            "data" => Data,
            "file" => File,
            "ftp" => Ftp,
            "ftps" => Ftps,
            "git" => Git,
            "http" => Http,
            "https" => Https,
            "ipfs" => Ipfs,
            "scp" => Scp,
            "stdin" => Stdin,
            scheme => Other(scheme.into()),
        })
    }
}

impl fmt::Display for UriScheme {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
