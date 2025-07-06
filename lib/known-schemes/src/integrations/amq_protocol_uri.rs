// This is free and unencumbered software released into the public domain.

impl From<&amq_protocol_uri::AMQPUri> for UriScheme {
    fn from(input: &amq_protocol_uri::AMQPUri) -> Self {
        (&input.scheme).into()
    }
}

impl From<&amq_protocol_uri::AMQPScheme> for UriScheme {
    fn from(input: &amq_protocol_uri::AMQPScheme) -> Self {
        use amq_protocol_uri::AMQPScheme;
        match input {
            AMQPScheme::AMQP => Self::Amqp,
            AMQPScheme::AMQPS => Self::Amqps,
        }
    }
}
