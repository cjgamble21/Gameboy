struct CartridgeHeader {
    data: Vec<u8>,
}

impl CartridgeHeader {
    pub fn new(data: Vec<u8>) -> Self {
        Self { data }
    }
}
