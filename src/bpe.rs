use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use itertools::Itertools;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct BPE {
    merges: Vec<(String, String)>,
    vocab: HashSet<String>,
}

impl BPE {
    pub fn new() -> Self {
        BPE {
            merges: Vec::new(),
            vocab: HashSet::new(),
        }
    }

    pub fn train(&mut self, text: &str, vocab_size: usize) {
        let mut words = Self::get_word_counts(text);
        let mut vocab: HashSet<String> = HashSet::new();
        
        for (word, _) in &words {
            for c in word.chars() {
                vocab.insert(c.to_string());
            }
        }
        
        let mut splits: HashMap<String, Vec<String>> = HashMap::new();
        for (word, _) in &words {
            splits.insert(word.clone(), word.chars().map(|c| c.to_string()).collect());
        }
        
        let mut num_merges = 0;
        while num_merges < vocab_size {
            let mut pair_counts: HashMap<(String, String), usize> = HashMap::new();
            for (word, count) in &words {
                let parts = splits.get(word).unwrap();
                for i in 0..parts.len() - 1 {
                    let pair = (parts[i].clone(), parts[i + 1].clone());
                    *pair_counts.entry(pair).or_insert(0) += count;
                }
            }
            
            if let Some(((p1, p2), _)) = pair_counts.iter().max_by_key(|(_, count)| *count) {
                let new_token = format!("{}{}", p1, p2);
                vocab.insert(new_token.clone());
                
                self.merges.push((p1.clone(), p2.clone()));
                
                for (word, _) in &words {
                    let mut parts = splits.get(word).unwrap().clone();
                    let mut i = 0;
                    while i < parts.len() - 1 {
                        if parts[i] == *p1 && parts[i + 1] == *p2 {
                            parts[i] = new_token.clone();
                            parts.remove(i + 1);
                        } else {
                            i += 1;
                        }
                    }
                    splits.insert(word.clone(), parts);
                }
                
                num_merges += 1;
                if num_merges % 1000 == 0 {
                    println!("Processed {} merges", num_merges);
                }
            } else {
                break;
            }
        }
        
        self.vocab = vocab;
    }
    
    pub fn tokenize(&self, text: &str) -> Vec<String> {
        let mut result = Vec::new();
        
        for word in text.split_whitespace() {
            let mut parts: Vec<String> = word.chars().map(|c| c.to_string()).collect();
            
            for (p1, p2) in &self.merges {
                let mut i = 0;
                while i < parts.len() - 1 {
                    if parts[i] == *p1 && parts[i + 1] == *p2 {
                        parts[i] = format!("{}{}", p1, p2);
                        parts.remove(i + 1);
                    } else {
                        i += 1;
                    }
                }
            }
            
            result.extend(parts);
        }
        
        result
    }
    
    pub fn save<P: AsRef<Path>>(&self, path: P) -> std::io::Result<()> {
        let mut file = File::create(path)?;
        let serialized = serde_json::to_string(self)?;
        file.write_all(serialized.as_bytes())?;
        Ok(())
    }
    
    pub fn load<P: AsRef<Path>>(path: P) -> std::io::Result<Self> {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let bpe: BPE = serde_json::from_str(&contents)?;
        Ok(bpe)
    }
    
    fn get_word_counts(text: &str) -> HashMap<String, usize> {
        text.split_whitespace()
            .map(|s| s.to_string())
            .fold(HashMap::new(), |mut acc, word| {
                *acc.entry(word).or_insert(0) += 1;
                acc
            })
    }
}