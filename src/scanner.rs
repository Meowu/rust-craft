pub struct Scanner {
    source: Vec<u8>,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Scanner {
            source: source.into_bytes(),
        }
    }
    pub fn scan_tokens(self) {
        todo!()
    }
}
