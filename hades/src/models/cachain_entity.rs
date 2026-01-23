#[derive(Debug, Clone)]
pub struct CaChain {
    cert: Vec<u8>,
    key: Vec<u8>,
}
impl CaChain {
    pub fn new(cert: Vec<u8>, key: Vec<u8>) -> Self {
        CaChain { cert, key }
    }
    pub fn get_cert(&self) -> Vec<u8> {
        self.cert.clone()
    }
    pub fn get_key(&self) -> Vec<u8> {
        self.key.clone()
    }
}