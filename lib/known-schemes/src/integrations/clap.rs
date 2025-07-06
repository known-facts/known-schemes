// This is free and unencumbered software released into the public domain.

impl clap::ValueEnum for UriScheme {
    fn value_variants<'a>() -> &'a [Self] {
        Self::ALL
    }

    fn to_possible_value<'a>(&self) -> Option<clap::builder::PossibleValue> {
        use clap::builder::PossibleValue;
        Some(PossibleValue::new(self.as_str().to_lowercase()))
    }
}
