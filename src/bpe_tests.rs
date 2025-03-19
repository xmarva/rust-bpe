#[cfg(test)]
mod tests {
    use super::*;
    use crate::bpe::BPE;
    use crate::tokenizer::Tokenizer;

    #[test]
    fn test_bpe_initialization() {
        let bpe = BPE::new();
        assert_eq!(bpe.merges.len(), 0);
        assert_eq!(bpe.vocab.len(), 0);
    }

    #[test]
    fn test_bpe_training_simple() {
        let text = "hello hello world world world";
        let mut bpe = BPE::new();
        
        // Train with just 2 merges
        bpe.train(text, 2);
        
        // We should have some merges
        assert_eq!(bpe.merges.len(), 2);
        
        // Original characters plus new merged tokens
        assert!(bpe.vocab.len() > 5);
    }

    #[test]
    fn test_bpe_tokenization() {
        let text = "hello hello world world world";
        let mut bpe = BPE::new();
        
        bpe.train(text, 5);
        
        let tokens = bpe.tokenize("hello world");

        assert!(tokens.len() < "hello world".chars().count());
        
        let joined = tokens.join("");
        assert_eq!(joined, "helloworld");
    }

    #[test]
    fn test_tokenizer_encode() {
        let text = "aa bb cc aa bb dd";
        let mut bpe = BPE::new();
        bpe.train(text, 3);
        
        let tokenizer = Tokenizer::new(bpe);
        let tokens = tokenizer.encode("aa bb");
        
        assert!(tokens.len() < "aa bb".chars().count());
    }

    #[test]
    fn test_save_and_load() {
        use std::fs;
        use tempfile::NamedTempFile;
        
        let text = "hello hello world world world";
        let mut bpe = BPE::new();
        bpe.train(text, 5);
        
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path();
        bpe.save(path).unwrap();
        
        let loaded_bpe = BPE::load(path).unwrap();
        
        assert_eq!(bpe.merges.len(), loaded_bpe.merges.len());
        assert_eq!(bpe.vocab.len(), loaded_bpe.vocab.len());
        
        let original_tokens = bpe.tokenize("hello world");
        let loaded_tokens = loaded_bpe.tokenize("hello world");
        
        assert_eq!(original_tokens, loaded_tokens);
    }
    
    #[test]
    fn test_larger_corpus() {
        let text = "this is a test of the byte pair encoding algorithm. \
                    byte pair encoding is used in many natural language processing tasks. \
                    it helps to represent words as subword units.";
        
        let mut bpe = BPE::new();
        bpe.train(text, 20);
        
        // Test tokenization
        let tokens = bpe.tokenize("this is a test of byte pair encoding");
        
        assert!(tokens.len() > 0);
        assert!(tokens.len() < "this is a test of byte pair encoding".chars().count());
    }
}