use std::{error, fmt};

use crate::error::NeoError;

#[derive(Debug)]
pub struct ErrorGroup(Vec<NeoError>);

impl ErrorGroup {
    pub fn new() -> Self {
        return Self(vec![]);
    }

    pub fn len(&self) -> usize {
        return self.0.len();
    }

    pub fn get(&self, i: usize) -> Option<&NeoError> {
        return self.0.get(i);
    }

    pub fn iter(&self) -> Iter {
        return Iter::from(self);
    }

    pub fn add(&mut self, err: NeoError) {
        self.0.push(err);
    }
}

impl fmt::Display for ErrorGroup {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for child in self.0.iter() {
            write!(f, "{}", child)?;
        }

        return Ok(());
    }
}

impl error::Error for ErrorGroup {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        return Some(self);
    }
}

#[derive(Debug, Clone)]
pub struct Iter<'a> {
    index: usize,
    group: &'a ErrorGroup,
}

impl<'a> From<&'a ErrorGroup> for Iter<'a> {
    fn from(group: &'a ErrorGroup) -> Self {
        return Self { index: 0, group };
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a NeoError;

    fn next(&mut self) -> Option<Self::Item> {
        self.index += 1;
        return self.group.get(self.index);
    }
}
