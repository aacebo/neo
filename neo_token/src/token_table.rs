#![allow(dead_code)]

use base64::prelude::*;
use neo_core::result::Result;
use std::{collections::HashMap, fs, path::Path};

#[derive(Debug, Clone)]
pub struct TokenTable {
    /// mapping of tokens to ids
    /// example: "!" => 0
    token_to_id: HashMap<Vec<u8>, usize>,

    /// mapping of ids to tokens
    /// example: 0 => ("!", "IQ==")
    id_to_token: HashMap<usize, (Vec<u8>, Vec<u8>)>,
}

impl TokenTable {
    pub fn new() -> Self {
        return Self {
            token_to_id: HashMap::new(),
            id_to_token: HashMap::new(),
        };
    }

    pub fn len(&self) -> usize {
        return self.id_to_token.len();
    }

    pub fn has(&self, id: &usize) -> bool {
        return self.id_to_token.contains_key(id);
    }

    pub fn has_token(&self, value: &[u8]) -> bool {
        return self.token_to_id.contains_key(value);
    }

    pub fn get(&self, id: &usize) -> Option<&(Vec<u8>, Vec<u8>)> {
        return self.id_to_token.get(id);
    }

    pub fn get_id(&self, token: &[u8]) -> Option<&usize> {
        return self.token_to_id.get(&token.to_vec());
    }

    pub fn add(&mut self, id: &usize, value: &(Vec<u8>, Vec<u8>)) {
        self.id_to_token.insert(id.clone(), value.clone());
        self.token_to_id.insert(value.0.clone(), id.clone());
    }
}

impl TokenTable {
    pub fn load(vocab: &str) -> Result<Self> {
        let mut table = Self::new();

        let bytes = fs::read(Path::new(
            format!("vocabs/{vocab}/{vocab}.tiktoken").as_str(),
        ))?;

        let lines: Vec<_> = bytes.split(|b| *b == b'\n').collect();

        for line in lines {
            if line.is_empty() {
                continue;
            }

            let (id, decoded, unicode) = Self::parse_line(line);
            table.add(&id, &(decoded, unicode));
        }

        return Ok(table);
    }

    fn parse_line(bytes: &[u8]) -> (usize, Vec<u8>, Vec<u8>) {
        let i = bytes.iter().position(|b| *b == b' ').unwrap();
        let unicode = &bytes[..i];
        let decoded = BASE64_STANDARD.decode(unicode).unwrap();
        let value = std::str::from_utf8(&bytes[i + 1..]).unwrap();
        let id = value.parse::<usize>().unwrap();
        return (id, decoded, unicode.to_vec());
    }
}
