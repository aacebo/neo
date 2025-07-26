#![allow(dead_code)]

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ReplaceTable {
    entries: HashMap<String, String>,
}

impl ReplaceTable {
    pub fn new() -> Self {
        return Self {
            entries: HashMap::new(),
        };
    }

    pub fn len(&self) -> usize {
        return self.entries.len();
    }

    pub fn add(&mut self, from: &str, to: &str) {
        self.entries.insert(from.to_string(), to.to_string());
    }

    pub fn replace(&self, value: &[u8]) -> Vec<u8> {
        let mut bytes = String::from_utf8(value.to_vec()).unwrap();

        for (from, to) in self.entries.iter() {
            bytes = bytes.replace(from, to);
        }

        return bytes.as_bytes().to_vec();
    }

    pub fn replace_pair(&self, value: &(Vec<u8>, Vec<u8>)) -> (Vec<u8>, Vec<u8>) {
        return (self.replace(&value.0), self.replace(&value.1));
    }
}

impl<const N: usize> From<[(&str, &str); N]> for ReplaceTable {
    fn from(entries: [(&str, &str); N]) -> Self {
        let mut table = Self::new();

        for entry in entries {
            table.add(entry.0, entry.1);
        }

        return table;
    }
}
