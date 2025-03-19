use simple_bpe::bpe::BPE;
use simple_bpe::tokenizer::Tokenizer;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_end_to_end() {
    let temp_dir = TempDir::new().unwrap();
    let input_path = temp_dir.path().join("input.txt");
    let model_path = temp_dir.path().join("model.json");
    let output_path = temp_dir.path().join("output.txt");

    // Create a test corpus
    let corpus = "Hello world! This is a test corpus for BPE.\n\
                  BPE stands for Byte Pair Encoding.\n\
                  It is a simple form of data compression.\n\
                  We will train on this corpus and then tokenize it.";
    
    fs::write(&input_path, corpus).unwrap();
    
    let mut bpe = BPE::new();
    bpe.train(&corpus, 10);
    bpe.save(&model_path).unwrap();
    
    let loaded_bpe = BPE::load(&model_path).unwrap();
    let tokenizer = Tokenizer::new(loaded_bpe);
    let tokens = tokenizer.encode(&corpus);
    
    fs::write(&output_path, tokens.join(" ")).unwrap();
    
    let output_content = fs::read_to_string(&output_path).unwrap();
    let read_tokens: Vec<&str> = output_content.split_whitespace().collect();
    
    assert!(!tokens.is_empty());
    assert_eq!(tokens.len(), read_tokens.len());
}

#[test]
fn test_unicode_support() {
    // Test with non-ASCII text
    let text = "Привет мир! こんにちは世界! 你好，世界!";
    
    let mut bpe = BPE::new();
    bpe.train(text, 5);
    
    let tokens = bpe.tokenize("Привет мир!");
    
    assert!(!tokens.is_empty());
    assert!(tokens.len() < "Привет мир!".chars().count());
    
    let joined = tokens.join("");
    assert_eq!(joined, "Приветмир!");
}