use crate::bpe::BPE;

pub struct Tokenizer {
    bpe: BPE,
}

impl Tokenizer {
    pub fn new(bpe: BPE) -> Self {
        Tokenizer { bpe }
    }
    
    pub fn encode(&self, text: &str) -> Vec<String> {
        self.bpe.tokenize(text)
    }
    
    pub fn encode_batch(&self, texts: &[String]) -> Vec<Vec<String>> {
        texts.iter().map(|text| self.encode(text)).collect()
    }
}