use std::ops::{Index, IndexMut};
use std::slice;

pub const ALPHABET_COUNT: usize = (b'z' - b'a') as usize + 1;
type Entry<Value> = (char, Value);
type Entries<Value> = [Entry<Value>; ALPHABET_COUNT];

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct AlphabetMap<Value> {
    entries: Entries<Value>,
}

impl<Value> IntoIterator for AlphabetMap<Value> {
    type Item = Entry<Value>;
    type IntoIter = <Entries<Value> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.entries.into_iter()
    }
}

impl<Value: Default> AlphabetMap<Value> {
    pub fn new() -> Self {
        let mut entries = Entries::<Value>::default();
        for (i, entry) in entries.iter_mut().enumerate().take(ALPHABET_COUNT) {
            entry.0 = (b'a' + i as u8) as char
        }
        Self { entries }
    }
}

impl<Value: Default> Default for AlphabetMap<Value> {
    fn default() -> Self {
        Self::new()
    }
}

impl<Value> AlphabetMap<Value> {
    pub fn iter(&self) -> slice::Iter<'_, Entry<Value>> {
        self.entries.iter()
    }

    pub fn iter_mut(&mut self) -> slice::IterMut<'_, Entry<Value>> {
        self.entries.iter_mut()
    }
}

impl<Value> From<Entries<Value>> for AlphabetMap<Value> {
    fn from(value: Entries<Value>) -> Self {
        AlphabetMap { entries: value }
    }
}

impl<Value> From<AlphabetMap<Value>> for Entries<Value> {
    fn from(value: AlphabetMap<Value>) -> Self {
        value.entries
    }
}

impl<Value> Index<char> for AlphabetMap<Value> {
    type Output = Value;

    fn index(&self, index: char) -> &Self::Output {
        assert!(index >= 'a');
        assert!(index <= 'z');
        &self.entries[(index as u8 - b'a') as usize].1
    }
}

impl<Value> IndexMut<char> for AlphabetMap<Value> {
    fn index_mut(&mut self, index: char) -> &mut Self::Output {
        assert!(index >= 'a');
        assert!(index <= 'z');
        &mut self.entries[(index as u8 - b'a') as usize].1
    }
}
