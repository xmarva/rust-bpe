# Minimal BPE

A minimal implementation of the BPE (Byte Pair Encoding) algorithm in Rust.

## Overview

Byte Pair Encoding is a data compression technique that iteratively replaces the most frequent pair of bytes in a sequence with a single, unused byte. BPE operates on character or unicode code point sequences rather than bytes, merging the most frequent adjacent pairs to form a new token.

## Project Structure

```
simple_bpe/
├── Cargo.toml
├── src/
│   ├── main.rs       - Command-line interface
│   ├── lib.rs        - Library exports
│   ├── bpe.rs        - Core BPE algorithm
│   ├── tokenizer.rs  - Tokenizer wrapper
│   ├── utils.rs      - Utility functions
│   └── bpe_tests.rs  - Unit tests
└── tests/
    └── integration_tests.rs 
```

## Build & Test

To build the project, run:

```bash
cargo build --release
```

To run the unit tests:

```bash
cargo test
```

To run a specific test:

```bash
cargo test test_bpe_tokenization
```

## How to run

### Training a BPE Model

```bash
cargo run -- train -i path/to/corpus.txt -o path/to/model.json -v 10000
```

Parameters:
- `-i, --input`: Path to the input text file for training
- `-o, --output`: Path where the trained model will be saved
- `-v, --vocab-size`: Number of merge operations to learn (default: 10000)

### Tokenizing Text

```bash
cargo run -- tokenize -i path/to/input.txt -m path/to/model.json -o path/to/tokens.txt
```

Parameters:
- `-i, --input`: Path to the text file to tokenize
- `-m, --model`: Path to the trained BPE model
- `-o, --output`: Path where tokenized output will be saved

## Using as a Library

You can also use Simple BPE as a library in your Rust projects:

```rust
use simple_bpe::bpe::BPE;
use simple_bpe::tokenizer::Tokenizer;

// Train a new model
let mut bpe = BPE::new();
bpe.train("your training text", 1000);

// Save the model
bpe.save("model.json").unwrap();

// Load an existing model
let bpe = BPE::load("model.json").unwrap();

// Create a tokenizer
let tokenizer = Tokenizer::new(bpe);

// Tokenize text
let tokens = tokenizer.encode("text to tokenize");
```