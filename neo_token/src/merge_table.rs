#![allow(dead_code)]

use neo_core::result::Result;
use std::{collections::HashMap, fs, path::Path};

#[derive(Debug, Clone)]
pub struct MergeTable {
    entries: HashMap<(Vec<u8>, Vec<u8>), u32>,
}

impl MergeTable {
    pub fn new() -> MergeTable {
        return Self {
            entries: HashMap::new(),
        };
    }

    pub fn len(&self) -> usize {
        return self.entries.len();
    }

    pub fn has(&self, key: &(Vec<u8>, Vec<u8>)) -> bool {
        return self.entries.contains_key(key);
    }

    pub fn get(&self, key: &(Vec<u8>, Vec<u8>)) -> Option<&u32> {
        return self.entries.get(key);
    }
}

impl MergeTable {
    pub fn load(vocab: &str) -> Result<Self> {
        let mut table = Self::new();
        let bytes = fs::read(Path::new(
            format!("vocabs/{vocab}/{vocab}.merge.tiktoken").as_str(),
        ))?;

        let lines: Vec<_> = bytes.split(|b| *b == b'\n').collect();
        let mut rank = 0;

        for line in lines {
            if line.starts_with(&[b'#']) || line.is_empty() {
                continue;
            }

            let key = Self::parse_line(line);
            table.entries.insert(key, rank);
            rank += 1;
        }

        return Ok(table);
    }

    fn parse_line(bytes: &[u8]) -> (Vec<u8>, Vec<u8>) {
        let i = bytes.iter().position(|b| *b == b' ').unwrap();
        let a = &bytes[..i];
        let b = &bytes[i + 1..];
        return (a.to_vec(), b.to_vec());
    }
}
