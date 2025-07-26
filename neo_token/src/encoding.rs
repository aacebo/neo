use crate::{MergeTable, ReplaceTable, TokenTable};
use neo_core::result::Result;

#[derive(Debug, Clone)]
pub struct Encoding {
    /// token table
    /// example 0 => ("!", "IQ==")
    tokens: TokenTable,

    /// replacement bytes
    /// example: ' ' => 'Ġ'
    replacements: ReplaceTable,

    /// merge table
    /// example ('a', 'b') => 0 (rank)
    merges: MergeTable,
}

impl Encoding {
    pub fn new() -> Self {
        return Self {
            tokens: TokenTable::new(),
            replacements: ReplaceTable::from([(" ", "Ġ")]),
            merges: MergeTable::new(),
        };
    }

    pub fn encode(&self, data: &[u8]) -> Vec<usize> {
        if data.is_empty() {
            return vec![];
        }

        let regex =
            fancy_regex::Regex::new(r"'| ?\p{L}+| ?\p{N}+| ?[^\s\p{L}\p{N}]+|\s+(?!\S)|\s+")
                .unwrap();
        let utf8 = str::from_utf8(data).unwrap();
        let mut chunks: Vec<usize> = vec![];

        for part in regex
            .find_iter(utf8)
            .map(|m| m.unwrap().as_str())
            .collect::<Vec<_>>()
        {
            let bytes = part.as_bytes();
            let encoded = self.byte_pair_encode(bytes);

            for chunk in encoded.iter() {
                let id = self.tokens.get_id(chunk).unwrap();
                chunks.push(*id);
            }
        }

        return chunks;
    }
}

impl Encoding {
    pub fn load(vocab: &str) -> Result<Self> {
        let mut encoding = Self::new();
        encoding.tokens = TokenTable::load(vocab)?;
        encoding.merges = MergeTable::load(vocab)?;
        return Ok(encoding);
    }
}

impl Encoding {
    fn byte_pair_encode(&self, data: &[u8]) -> Vec<Vec<u8>> {
        let mut tokens: Vec<Vec<u8>> = vec![];

        for v in data {
            tokens.push(vec![*v]);
        }

        loop {
            if tokens.len() < 2 {
                break;
            }

            let mut best_pair: (Vec<u8>, Vec<u8>) = (vec![], vec![]);
            let mut best_pair_rank = u32::MAX;
            let mut best_pair_index = 0;

            for i in 0..tokens.len() - 1 {
                let pair = (tokens[i].clone(), tokens[i + 1].clone());

                if !self
                    .tokens
                    .has_token(&[pair.0.clone(), pair.1.clone()].concat())
                {
                    continue;
                }

                let rank = match self.merges.get(&self.replacements.replace_pair(&pair)) {
                    None => u32::MAX,
                    Some(v) => *v,
                };

                if rank < best_pair_rank {
                    best_pair_rank = rank;
                    best_pair = pair.clone();
                    best_pair_index = i;
                }
            }

            if best_pair_rank == u32::MAX {
                break;
            }

            tokens.splice(
                best_pair_index..best_pair_index + 2,
                vec![[best_pair.0.clone(), best_pair.1.clone()].concat()],
            );
        }

        return tokens;
    }
}
