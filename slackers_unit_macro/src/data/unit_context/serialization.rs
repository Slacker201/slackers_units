#![allow(unused)]
use std::{array::from_fn, collections::HashMap, ptr};

use quote::format_ident;
use syn::Ident;

use crate::data::{
    Decimal, Id,
    unit_context::{UnitContext, prefix_trie::PrefixTrie},
};

pub trait ToBytes {
    fn to_bytes(&self, writer: &mut Writer<'_>);
    fn from_bytes(reader: &mut Reader<'_>) -> Self;
}

pub struct Writer<'a> {
    slice: &'a mut [u8],
    idx: usize,
    is_fake: bool,
}

impl<'a> Writer<'a> {
    fn new(slice: &'a mut [u8]) -> Self {
        Self {
            slice,
            idx: 0,
            is_fake: false,
        }
    }
    fn new_fake() -> Self {
        Self {
            slice: &mut [],
            idx: 0,
            is_fake: true,
        }
    }
    fn write_usize(&mut self, value: usize) {
        self.write_bytes(&value.to_be_bytes());
    }
    fn write_u64(&mut self, value: u64) {
        self.write_bytes(&value.to_be_bytes());
    }
    fn write_bytes(&mut self, data: &[u8]) {
        if !self.is_fake {
            let src = data.as_ptr();
            let dest = self.slice[self.idx..].as_ptr().cast_mut();
            unsafe {
                ptr::copy_nonoverlapping(src, dest, data.len());
            }
        }
        self.idx += data.len();
    }
    fn write_decimal(&mut self, data: Decimal) {
        self.write_bytes(&data.to_be_bytes());
    }
    fn write_len_prefixed_str(&mut self, s: &str) {
        let len = s.len();
        self.write_usize(len);
        self.write_bytes(s.as_bytes());
    }
    pub fn serialize_data<T>(data: &T) -> Box<[u8]>
    where
        T: ToBytes,
    {
        let mut fake = Self::new_fake();
        data.to_bytes(&mut fake);
        let len = fake.idx;
        let mut backer: Vec<u8> = vec![0; len];
        let mut real = Writer::new(&mut backer);
        data.to_bytes(&mut real);
        backer.into_boxed_slice()
    }
}

pub struct Reader<'a> {
    buffer: &'a [u8],
    idx: usize,
}

impl<'a> Reader<'a> {
    fn new(buffer: &'a [u8]) -> Self {
        Self { buffer, idx: 0 }
    }
    fn read_usize(&mut self) -> usize {
        let bytes: [u8; size_of::<usize>()] = from_fn(|idx| self.buffer[self.idx + idx]);
        self.idx += size_of::<usize>();
        usize::from_be_bytes(bytes)
    }
    fn read_str(&mut self) -> &str {
        let len = self.read_usize();
        let s = unsafe { str::from_utf8_unchecked(&self.buffer[self.idx..self.idx + len]) };
        self.idx += len;
        s
    }
    fn read_u64(&mut self) -> u64 {
        let bytes: [u8; size_of::<u64>()] = from_fn(|idx| self.buffer[self.idx + idx]);
        self.idx += size_of::<u64>();
        u64::from_be_bytes(bytes)
    }
    fn read_decimal(&mut self) -> Decimal {
        let bytes: [u8; size_of::<Decimal>()] = from_fn(|idx| self.buffer[self.idx + idx]);
        self.idx += size_of::<Decimal>();
        Decimal::from_be_bytes(bytes)
    }
    fn deserialize<T: ToBytes>(bytes: &'a [u8]) -> T {
        let mut reader = Self::new(bytes);
        T::from_bytes(&mut reader)
    }
}

impl ToBytes for Box<[Ident]> {
    fn to_bytes(&self, writer: &mut Writer<'_>) {
        let ident_count = self.len();
        writer.write_usize(ident_count);
        for ident in self {
            writer.write_len_prefixed_str(&ident.to_string());
        }
    }
    fn from_bytes(reader: &mut Reader<'_>) -> Self {
        let len = reader.read_usize();
        let mut s = Self::new_uninit_slice(len);
        for idx in 0..len {
            s[idx].write(format_ident!("{}", reader.read_str()));
        }
        unsafe { s.assume_init() }
    }
}

impl ToBytes for Box<[(String, Id)]> {
    fn to_bytes(&self, writer: &mut Writer<'_>) {
        writer.write_usize(self.len());
        for (s, id) in self {
            writer.write_len_prefixed_str(s);
            writer.write_u64(id.0);
        }
    }

    fn from_bytes(reader: &mut Reader<'_>) -> Self {
        let len = reader.read_usize();
        let mut s = Self::new_uninit_slice(len);
        for idx in 0..len {
            let string = reader.read_str().to_string();
            let id = Id(reader.read_u64());
            s[idx].write((string, id));
        }
        unsafe { s.assume_init() }
    }
}

impl ToBytes for HashMap<Id, f64> {
    fn to_bytes(&self, writer: &mut Writer<'_>) {
        let len = self.len();
        writer.write_usize(len);
        for (id, decimal) in self {
            writer.write_u64(id.0);
            writer.write_decimal(*decimal);
        }
    }

    fn from_bytes(reader: &mut Reader<'_>) -> Self {
        let len = reader.read_usize();
        let mut s = Self::new();
        for _ in 0..len {
            let id = reader.read_u64();
            let decimal = reader.read_decimal();
            s.insert(Id(id), decimal);
        }
        s
    }
}

impl ToBytes for UnitContext {
    fn to_bytes(&self, writer: &mut Writer<'_>) {
        self.prefix_data.to_bytes(writer);
        self.unit_data.to_bytes(writer);
        self.prefix_id_to_multiplier.to_bytes(writer);
    }

    fn from_bytes(reader: &mut Reader<'_>) -> Self {
        let prefix_data: Box<[(String, Id)]> = Box::from_bytes(reader);
        let unit_data: Box<[(String, Id)]> = Box::from_bytes(reader);
        let prefix_id_to_multiplier = HashMap::<Id, f64>::from_bytes(reader);
        let _ = reader;
        let mut unit_trie = PrefixTrie::default();
        let mut prefix_trie = PrefixTrie::default();

        for (key, id) in &unit_data {
            unit_trie.add_item(key.as_bytes(), *id);
        }

        for (key, id) in &prefix_data {
            prefix_trie.add_item(key.as_bytes(), *id);
        }

        Self {
            unit_trie,
            prefix_trie,
            prefix_id_to_multiplier,
            prefix_data,
            unit_data,
        }
    }
}

impl UnitContext {
    pub fn serialize(&self) -> Box<[u8]> {
        Writer::serialize_data(self)
    }
    pub fn deserialize(data: &[u8]) -> Self {
        Reader::deserialize(data)
    }
}
