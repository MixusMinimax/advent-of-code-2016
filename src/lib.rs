extern crate core;

pub const ALPHABET_COUNT: usize = (b'z' - b'a') as usize + 1;

pub use alphabet_map::AlphabetMap;
pub use index_map::IndexMap;
use num_traits::PrimInt;

pub mod graph;

pub mod alphabet_map {
    use crate::ALPHABET_COUNT;
    use std::iter::{IntoIterator, Iterator};
    use std::ops::{Index, IndexMut};

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
}

pub mod index_map {
    #![allow(private_bounds)]

    use std::error::Error;
    use std::fmt::{Debug, Formatter};
    use std::marker::PhantomData;
    use std::ops::Index;

    trait Key:
        From<u8> + Default + TryFrom<usize, Error: Error> + TryInto<usize, Error: Error>
    {
    }
    impl<T: From<u8> + Default + TryFrom<usize, Error: Error> + TryInto<usize, Error: Error>> Key
        for T
    {
    }

    #[derive(Eq, PartialEq)]
    pub struct IndexMap<K: Key, V> {
        data: Vec<Option<V>>,
        len: usize,
        key_pd: PhantomData<K>,
    }

    impl<K: Key + Clone, V: Clone> Clone for IndexMap<K, V> {
        fn clone(&self) -> Self {
            Self {
                data: Clone::clone(&self.data),
                len: Clone::clone(&self.len),
                key_pd: Clone::clone(&self.key_pd),
            }
        }
    }

    impl<K: Key, V> Default for IndexMap<K, V> {
        fn default() -> Self {
            Self {
                data: Default::default(),
                len: Default::default(),
                key_pd: Default::default(),
            }
        }
    }

    impl<K: Key + Debug, V: Debug> Debug for IndexMap<K, V> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            f.debug_map().entries(self.iter()).finish()
        }
    }

    impl<K: Key, V> IndexMap<K, V> {
        #[inline]
        #[must_use]
        pub fn new() -> Self {
            IndexMap::<K, V>::default()
        }

        pub fn insert(&mut self, k: K, v: V) {
            let k: usize = k.try_into().unwrap();
            self.ensure_capacity(k + 1);
            self.data[k] = Some(v);
        }

        pub fn contains(&self, k: K) -> bool {
            let k: usize = k.try_into().unwrap();
            self.data.len() > k && self.data[k].is_some()
        }

        pub fn get(&self, k: K) -> Option<&V> {
            let k: usize = k.try_into().unwrap();
            if self.data.len() > k {
                self.data[k].as_ref()
            } else {
                None
            }
        }

        pub fn get_mut(&mut self, k: K) -> Option<&mut V> {
            let k: usize = k.try_into().unwrap();
            if self.data.len() > k {
                self.data[k].as_mut()
            } else {
                None
            }
        }

        #[inline]
        pub fn len(&self) -> usize {
            self.data.iter().flatten().count()
        }

        #[inline]
        pub fn is_empty(&self) -> bool {
            self.len() == 0
        }

        #[inline]
        pub fn iter(&self) -> Iter<'_, K, V> {
            Iter(self.data.iter(), 0, PhantomData)
        }

        #[inline]
        pub fn values(&self) -> impl Iterator<Item = &V> {
            self.data.iter().flatten()
        }

        #[inline]
        pub fn values_mut(&mut self) -> impl Iterator<Item = &mut V> {
            self.data.iter_mut().flatten()
        }

        #[inline]
        pub fn into_values(self) -> impl Iterator<Item = V> {
            self.data.into_iter().flatten()
        }

        #[inline]
        fn ensure_capacity(&mut self, size: usize) {
            if self.data.len() < size {
                self.data.resize_with(size, || None)
            }
        }
    }

    impl<K: Key, V> Index<K> for IndexMap<K, V> {
        type Output = V;

        fn index(&self, index: K) -> &Self::Output {
            self.get(index).unwrap()
        }
    }

    pub struct IntoIter<K: Key, V>(
        <Vec<Option<V>> as IntoIterator>::IntoIter,
        usize,
        PhantomData<K>,
    );

    impl<K: Key, V> Iterator for IntoIter<K, V> {
        type Item = (K, V);

        fn next(&mut self) -> Option<Self::Item> {
            let (diff, v) = self
                .0
                .by_ref()
                .enumerate()
                .flat_map(|(i, v)| v.map(|v| (i, v)))
                .next()?;
            self.1 += diff + 1;
            Some(((self.1 - 1).try_into().unwrap(), v))
        }
    }

    pub struct Iter<'a, K: Key, V>(
        <&'a [Option<V>] as IntoIterator>::IntoIter,
        usize,
        PhantomData<K>,
    );

    impl<'a, K: Key, V> Iterator for Iter<'a, K, V> {
        type Item = (K, &'a V);

        fn next(&mut self) -> Option<Self::Item> {
            let (diff, v) = self
                .0
                .by_ref()
                .enumerate()
                .flat_map(|(i, v)| v.as_ref().map(|v| (i, v)))
                .next()?;
            self.1 += diff + 1;
            Some(((self.1 - 1).try_into().unwrap(), v))
        }
    }

    pub struct IterMut<'a, K: Key, V>(
        <&'a mut [Option<V>] as IntoIterator>::IntoIter,
        usize,
        PhantomData<K>,
    );

    impl<'a, K: Key, V> Iterator for IterMut<'a, K, V> {
        type Item = (K, &'a mut V);

        fn next(&mut self) -> Option<Self::Item> {
            let (diff, v) = self
                .0
                .by_ref()
                .enumerate()
                .flat_map(|(i, v)| v.as_mut().map(|v| (i, v)))
                .next()?;
            self.1 += diff + 1;
            Some(((self.1 - 1).try_into().unwrap(), v))
        }
    }

    impl<K: Key, V> IntoIterator for IndexMap<K, V> {
        type Item = (K, V);
        type IntoIter = IntoIter<K, V>;

        fn into_iter(self) -> Self::IntoIter {
            IntoIter(self.data.into_iter(), 0, PhantomData)
        }
    }

    impl<'a, K: Key, V> IntoIterator for &'a IndexMap<K, V> {
        type Item = (K, &'a V);
        type IntoIter = Iter<'a, K, V>;

        fn into_iter(self) -> Self::IntoIter {
            Iter(self.data.iter(), 0, PhantomData)
        }
    }

    impl<'a, K: Key, V> IntoIterator for &'a mut IndexMap<K, V> {
        type Item = (K, &'a mut V);
        type IntoIter = IterMut<'a, K, V>;

        fn into_iter(self) -> Self::IntoIter {
            IterMut(self.data.iter_mut(), 0, PhantomData)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_into_iter() {
            let mut map = IndexMap::new();
            map.insert(2u32, "Hello");
            map.insert(5u32, "World");
            map.insert(3u32, ", ");
            let entries: Vec<_> = map.into_iter().collect();
            assert_eq!(entries, [(2u32, "Hello"), (3u32, ", "), (5u32, "World")]);
        }
    }
}

pub fn vec2_hamming_dist<S: PrimInt>([a0, a1]: [S; 2], [b0, b1]: [S; 2]) -> S {
    (if a0 > b0 { a0 - b0 } else { b0 - a0 }) + (if a1 > b1 { a1 - b1 } else { b1 - a1 })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hamming_dist() {
        assert_eq!(vec2_hamming_dist::<i32>([5, 7], [-1, 2]), 11i32);
        assert_eq!(vec2_hamming_dist::<u32>([5, 7], [7, 2]), 7u32);
    }
}
