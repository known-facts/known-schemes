// This is free and unencumbered software released into the public domain.

use super::prelude::{fmt, FromStr, String};

/// An enumerated URI scheme.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum UriScheme {
    About,
    Android,
    Bitcoin,
    Chrome,
    ChromeExtension,
    Dat,
    Data,
    Did,
    Doi,
    Example,
    File,
    Ftp,
    Ftps,
    Git,
    Http,
    Https,
    Imap,
    Imaps,
    Ipfs,
    Irc,
    Ircs,
    Mailto,
    Matrix,
    Redis,
    Rediss,
    Scp,
    Sftp,
    Sms,
    Smtp,
    Smtps,
    Ssh,
    Stdin,
    Tag,
    Tel,
    Telnet,
    Tftp,
    Urn,
    Ws,
    Wss,
    Xmpp,
    Other(String),
}

impl UriScheme {
    pub fn as_str(&self) -> &str {
        use UriScheme::*;
        match self {
            About => "about",
            Android => "android",
            Bitcoin => "bitcoin",
            Chrome => "chrome",
            ChromeExtension => "chrome-extension",
            Dat => "dat",
            Data => "data",
            Did => "did",
            Doi => "doi",
            Example => "example",
            File => "file",
            Ftp => "ftp",
            Ftps => "ftps",
            Git => "git",
            Http => "http",
            Https => "https",
            Imap => "imap",
            Imaps => "imaps",
            Ipfs => "ipfs",
            Irc => "irc",
            Ircs => "ircs",
            Mailto => "mailto",
            Matrix => "matrix",
            Redis => "redis",
            Rediss => "rediss",
            Scp => "scp",
            Sftp => "sftp",
            Sms => "sms",
            Smtp => "smtp",
            Smtps => "smtps",
            Ssh => "ssh",
            Stdin => "stdin",
            Tag => "tag",
            Tel => "tel",
            Telnet => "telnet",
            Tftp => "tftp",
            Urn => "urn",
            Ws => "ws",
            Wss => "wss",
            Xmpp => "xmpp",
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
            Imap => 143,
            Imaps => 993,
            Irc => 6667,
            Ircs => 6697,
            Matrix => 8448,
            Redis => 6379,
            Rediss => 6380,
            Scp => 22,
            Sftp => 22,
            Smtp => 25,
            Smtps => 587,
            Ssh => 22,
            Telnet => 23,
            Tftp => 69,
            Ws => 80,
            Wss => 443,
            Xmpp => 5222,
            About | Android | Bitcoin | Chrome | ChromeExtension | Dat | Data | Did | Doi
            | Example | File | Ipfs | Mailto | Sms | Stdin | Tag | Tel | Urn | Other(_) => {
                return None
            }
        })
    }
}

impl FromStr for UriScheme {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        use UriScheme::*;
        Ok(match input {
            "about" => About,
            "android" => Android,
            "bitcoin" => Bitcoin,
            "chrome" => Chrome,
            "chrome-extension" => ChromeExtension,
            "dat" => Dat,
            "data" => Data,
            "did" => Did,
            "doi" => Doi,
            "example" => Example,
            "file" => File,
            "ftp" => Ftp,
            "ftps" => Ftps,
            "git" => Git,
            "http" => Http,
            "https" => Https,
            "imap" => Imap,
            "imaps" => Imaps,
            "ipfs" => Ipfs,
            "irc" => Irc,
            "ircs" => Ircs,
            "mailto" => Mailto,
            "matrix" => Matrix,
            "redis" => Redis,
            "rediss" => Rediss,
            "scp" => Scp,
            "sftp" => Sftp,
            "sms" => Sms,
            "smtp" => Smtp,
            "smtps" => Smtps,
            "ssh" => Ssh,
            "stdin" => Stdin,
            "tel" => Tel,
            "urn" => Urn,
            "ws" => Ws,
            "wss" => Wss,
            "xmpp" => Xmpp,
            scheme => Other(scheme.into()),
        })
    }
}

impl fmt::Display for UriScheme {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[cfg(feature = "fluent-uri")]
impl From<fluent_uri::Uri<&str>> for UriScheme {
    fn from(input: fluent_uri::Uri<&str>) -> Self {
        let input_scheme = input.scheme().as_str();
        Self::from_str(input_scheme).unwrap_or_else(|_| Self::Other(input_scheme.into()))
    }
}

#[cfg(feature = "fluent-uri")]
impl From<fluent_uri::Uri<String>> for UriScheme {
    fn from(input: fluent_uri::Uri<String>) -> Self {
        let input_scheme = input.scheme().as_str();
        Self::from_str(input_scheme).unwrap_or_else(|_| Self::Other(input_scheme.into()))
    }
}

#[cfg(feature = "url")]
impl From<url::Url> for UriScheme {
    fn from(input: url::Url) -> Self {
        let input_scheme = input.scheme();
        Self::from_str(input_scheme).unwrap_or_else(|_| Self::Other(input_scheme.into()))
    }
}
