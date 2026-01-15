use std::iter::{IntoIterator, Iterator};
use std::ops::{Index, IndexMut};

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

impl<Value: Default> FromIterator<Entry<Value>> for AlphabetMap<Value> {
    fn from_iter<T: IntoIterator<Item = Entry<Value>>>(iter: T) -> Self {
        let mut result = Self::new();
        for (k, v) in iter {
            result[k] = v;
        }
        result
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
    pub fn iter(&self) -> impl Iterator<Item = &'_ Entry<Value>> {
        self.entries.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &'_ mut Entry<Value>> {
        self.entries.iter_mut()
    }

    pub fn into_keys(self) -> impl Iterator<Item = char> {
        self.entries.into_iter().map(|(k, _)| k)
    }

    pub fn into_values(self) -> impl Iterator<Item = Value> {
        self.entries.into_iter().map(|(_, v)| v)
    }
}

impl<Value> AlphabetMap<Value> {
    pub fn from_entries_unchecked(value: Entries<Value>) -> Self {
        AlphabetMap { entries: value }
    }
}

impl<Value: Default, const N: usize> From<[(char, Value); N]> for AlphabetMap<Value> {
    fn from(value: [(char, Value); N]) -> Self {
        Self::from_iter(value)
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_hash_map_conversion() {
        let m = HashMap::from([('x', 5usize), ('b', 7usize)]);
        let am: AlphabetMap<_> = m.into_iter().collect();
        assert_eq!(am['b'], 7);
        assert_eq!(am['x'], 5);
        let m: HashMap<_, _> = am.into_iter().collect();
        assert_eq!(m[&'b'], 7);
        assert_eq!(m[&'x'], 5);
    }
}
