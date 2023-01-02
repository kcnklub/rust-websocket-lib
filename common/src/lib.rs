struct DataFrame
{
    pub is_final_message: bool,
    pub op_code: OpCode,
    pub is_masked: bool,
    pub payload_length: u8,
    pub payload_length_extended: u64,
    pub masking_key: u8,
    pub payload: Vec<u8>,
}

enum OpCode
{
    Continuation,
    Text,
    Binary,
    NonFurtherControl,
    Close,
    Ping,
    Pong,
    Control,
}
