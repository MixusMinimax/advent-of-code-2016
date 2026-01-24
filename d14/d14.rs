#![feature(bstr)]

use itertools::Itertools;
use md5::{Digest, Md5};
use std::bstr::ByteString;
use std::io::Write;
use std::marker::PhantomData;

trait KeyHasher {
    fn last_input(&self) -> &[u8];
    fn hash_hex<'b>(&mut self, i: u64, out: &'b mut [u8]) -> &'b [u8];
}

struct DigestHasher<D: Digest> {
    input_buffer: Vec<u8>,
    salt_len: usize,
    phantom_data: PhantomData<D>,
}

impl<D: Digest> DigestHasher<D> {
    fn new(salt: &[u8]) -> Self {
        Self {
            input_buffer: salt.to_vec(),
            salt_len: salt.len(),
            phantom_data: PhantomData,
        }
    }
}

impl<D: Digest> KeyHasher for DigestHasher<D> {
    fn last_input(&self) -> &[u8] {
        &self.input_buffer
    }

    fn hash_hex<'b>(&mut self, i: u64, out: &'b mut [u8]) -> &'b [u8] {
        self.input_buffer.truncate(self.salt_len);
        write!(&mut self.input_buffer, "{}", i).unwrap();
        base16ct::lower::encode(&Md5::digest(&self.input_buffer), out).unwrap();
        out
    }
}

struct MultiDigestHasher<D: Digest> {
    input_buffer: Vec<u8>,
    salt_len: usize,
    repetitions: u32,
    phantom_data: PhantomData<D>,
}

impl<D: Digest> MultiDigestHasher<D> {
    fn new(salt: &[u8], repetitions: u32) -> Self {
        Self {
            input_buffer: salt.to_vec(),
            salt_len: salt.len(),
            phantom_data: PhantomData,
            repetitions,
        }
    }
}

impl<D: Digest> KeyHasher for MultiDigestHasher<D> {
    fn last_input(&self) -> &[u8] {
        &self.input_buffer
    }

    fn hash_hex<'b>(&mut self, i: u64, out: &'b mut [u8]) -> &'b [u8] {
        let [mut buf_a, mut buf_b] = [[0u8; 32]; 2];

        self.input_buffer.truncate(self.salt_len);
        write!(&mut self.input_buffer, "{}", i).unwrap();

        if self.repetitions != 0 {
            base16ct::lower::encode(&Md5::digest(&self.input_buffer), &mut buf_a).unwrap();
        }

        for rep in 1..self.repetitions {
            if rep & 1 == 1 {
                base16ct::lower::encode(&Md5::digest(buf_a), &mut buf_b).unwrap();
            } else {
                base16ct::lower::encode(&Md5::digest(buf_b), &mut buf_a).unwrap();
            }
        }

        out.copy_from_slice(if self.repetitions & 1 == 1 {
            &buf_a
        } else {
            &buf_b
        });

        out
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Default)]
struct Key {
    index: u64,
    triple: u8,
    value: ByteString,
    hash_hex: ByteString,
    validated_at: u64,
    validation_hash: ByteString,
}

fn get_keys<K: KeyHasher>(mut key_hasher: K, count: usize) -> Vec<Key> {
    let mut result = Vec::new();

    let mut checking = Vec::new();

    let mut out_buf = [0u8; 32];
    let mut out_buf_2 = [0u8; 32];
    let mut completed_at = None;
    for i in 0u64.. {
        key_hasher.hash_hex(i, &mut out_buf);

        if !checking.is_empty() {
            for (v, ..) in out_buf
                .iter()
                .copied()
                .tuple_windows()
                .filter(|&(a, b, c, d, e)| [a, b, c, d, e].into_iter().all_equal())
            {
                checking.retain(|&(created_at, c_v)| {
                    if i - created_at > 1000 {
                        return false;
                    }
                    if v == c_v {
                        // re-constructing the hash is probably cheaper than storing allocated
                        // byte strings in [checking], so we do that here.
                        key_hasher.hash_hex(created_at, &mut out_buf_2);
                        result.push(Key {
                            index: created_at,
                            triple: c_v,
                            value: ByteString(key_hasher.last_input().to_vec()),
                            hash_hex: ByteString(out_buf_2.to_vec()),
                            validated_at: i,
                            validation_hash: ByteString(out_buf.to_vec()),
                        });
                        return false;
                    }

                    true
                });
            }
        }

        // make sure to add something to checking _after_ we do the checking,
        // because otherwise we would validate words using themselves.
        if let Some((t_c, _, _)) = out_buf
            .iter()
            .copied()
            .tuple_windows()
            .find(|&(a, b, c)| a == b && b == c)
        {
            checking.push((i, t_c));
        }

        if completed_at.is_none() && result.len() >= count {
            completed_at = Some(i);
        } else if let Some(completed_at) = completed_at
            && i > completed_at + 1000
        {
            result.sort_by_key(|k| k.index);
            result.truncate(count);
            return result;
        }
    }

    unreachable!()
}

fn debug_result(res: &[Key]) {
    println!("index,value,hash,triple,validated_at,validated_hash");
    for k in res {
        println!(
            "{},{},{},{},{},{}",
            k.index, k.value, k.hash_hex, k.triple as char, k.validated_at, k.validation_hash,
        );
        assert!(k.index + 1000 >= k.validated_at);
        assert!(
            k.hash_hex
                .iter()
                .copied()
                .tuple_windows::<(_, _, _)>()
                .contains(&(k.triple, k.triple, k.triple))
        )
    }
}

fn main() {
    // let input = b"abc";
    let input = b"yjdafjpo";

    println!("Running Part 1...");
    let res = get_keys(DigestHasher::<Md5>::new(input), 64);
    debug_result(&res);

    println!();

    println!("Running Part 2...");
    let res = get_keys(MultiDigestHasher::<Md5>::new(input, 2017), 64);
    debug_result(&res);
}
