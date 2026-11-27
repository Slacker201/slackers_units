#![allow(unused)]
use std::{array::from_fn, collections::HashMap, hash::Hash, ptr, sync::Arc};

use num_rational::Ratio;
use proc_macro2::Span;
use quote::format_ident;
use syn::Ident;

use crate::data::{
    RatioRepr, NumberRepr, Id,
    parser_data::{Aliases, PrefixClasses, PrefixDefinition, UnitDefinition},
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
    fn write_decimal(&mut self, data: RatioRepr) {
        let (numer, denom) = data.into_raw();
        self.write_decimal_part(numer);
        self.write_decimal_part(denom);
    }
    fn write_decimal_part(&mut self, part: NumberRepr) {
        self.write_bytes(&part.to_be_bytes());
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
    fn read_decimal_part(&mut self) -> NumberRepr {
        let bytes: [u8; size_of::<NumberRepr>()] = from_fn(|idx| self.buffer[self.idx + idx]);
        self.idx += size_of::<NumberRepr>();
        NumberRepr::from_be_bytes(bytes)
    }
    fn read_decimal(&mut self) -> RatioRepr {
        let numer = self.read_decimal_part();
        let denom = self.read_decimal_part();
        RatioRepr::new(numer, denom)
    }
    fn read_bytes(&mut self, len: usize) -> &[u8] {
        let bytes = &self.buffer[self.idx..self.idx + len];
        self.idx += len;
        bytes
    }
    fn deserialize<T: ToBytes>(bytes: &'a [u8]) -> T {
        let mut reader = Self::new(bytes);
        T::from_bytes(&mut reader)
    }
}

impl<T: ToBytes> ToBytes for Box<[T]> {
    fn to_bytes(&self, writer: &mut Writer<'_>) {
        writer.write_usize(self.len());
        for value in self {
            value.to_bytes(writer);
        }
    }

    fn from_bytes(reader: &mut Reader<'_>) -> Self {
        let count = reader.read_usize();
        let mut s = Self::new_uninit_slice(count);
        for item in &mut s {
            item.write(T::from_bytes(reader));
        }

        unsafe { s.assume_init() }
    }
}

impl<K: ToBytes + Ord + Hash, V: ToBytes> ToBytes for HashMap<K, V> {
    fn to_bytes(&self, writer: &mut Writer<'_>) {
        writer.write_usize(self.len());
        let mut keys: Box<[_]> = self.iter().collect();
        keys.sort_by_key(|(k, v)| *k);
        for (key, value) in keys {
            key.to_bytes(writer);
            value.to_bytes(writer);
        }
    }

    fn from_bytes(reader: &mut Reader<'_>) -> Self {
        let len = reader.read_usize();
        let mut s = Self::new();
        for _ in 0..len {
            let key = K::from_bytes(reader);
            let value = V::from_bytes(reader);
            s.insert(key, value);
        }

        s
    }
}

impl ToBytes for Id {
    fn to_bytes(&self, writer: &mut Writer<'_>) {
        writer.write_u64(self.0);
    }

    fn from_bytes(reader: &mut Reader<'_>) -> Self {
        Self(reader.read_u64())
    }
}

impl ToBytes for String {
    fn to_bytes(&self, writer: &mut Writer<'_>) {
        writer.write_len_prefixed_str(self);
    }

    fn from_bytes(reader: &mut Reader<'_>) -> Self {
        let len = reader.read_usize();
        unsafe { Self::from_utf8_unchecked(reader.read_bytes(len).into()) }
    }
}

impl ToBytes for Ident {
    fn to_bytes(&self, writer: &mut Writer<'_>) {
        self.to_string().to_bytes(writer);
    }

    fn from_bytes(reader: &mut Reader<'_>) -> Self {
        format_ident!("{}", String::from_bytes(reader))
    }
}

impl<T: ToBytes, U: ToBytes> ToBytes for (T, U) {
    fn to_bytes(&self, writer: &mut Writer<'_>) {
        self.0.to_bytes(writer);
        self.1.to_bytes(writer);
    }

    fn from_bytes(reader: &mut Reader<'_>) -> Self {
        (T::from_bytes(reader), U::from_bytes(reader))
    }
}

impl ToBytes for RatioRepr {
    fn to_bytes(&self, writer: &mut Writer<'_>) {
        writer.write_decimal(*self);
    }

    fn from_bytes(reader: &mut Reader<'_>) -> Self {
        reader.read_decimal()
    }
}

impl ToBytes for i32 {
    fn to_bytes(&self, writer: &mut Writer<'_>) {
        writer.write_bytes(&self.to_be_bytes());
    }

    fn from_bytes(reader: &mut Reader<'_>) -> Self {
        let bytes = reader.read_bytes(size_of::<i32>());

        i32::from_be_bytes(from_fn(|idx| bytes[idx]))
    }
}

impl ToBytes for PrefixClasses {
    fn to_bytes(&self, writer: &mut Writer<'_>) {
        self.allowed_groups.to_bytes(writer);
    }

    fn from_bytes(reader: &mut Reader<'_>) -> Self {
        Self {
            allowed_groups: ToBytes::from_bytes(reader),
        }
    }
}

impl ToBytes for UnitContext {
    fn to_bytes(&self, writer: &mut Writer<'_>) {
        self.prefix_data.to_bytes(writer);
        self.unit_data.to_bytes(writer);
        self.prefix_id_to_multiplier.to_bytes(writer);
        self.unit_id_to_vector.to_bytes(writer);
        self.prefix_id_to_grouping.to_bytes(writer);
        self.unit_id_to_grouping.to_bytes(writer);
        writer.write_u64(self.null_id.0);
    }

    fn from_bytes(reader: &mut Reader<'_>) -> Self {
        let prefix_data: Box<[(String, Id)]> = Box::from_bytes(reader);
        let unit_data: Box<[(String, Id)]> = Box::from_bytes(reader);
        let prefix_id_to_multiplier = HashMap::<Id, RatioRepr>::from_bytes(reader);
        let unit_id_to_vector = ToBytes::from_bytes(reader);
        let prefix_id_to_grouping = ToBytes::from_bytes(reader);
        let unit_id_to_grouping = ToBytes::from_bytes(reader);
        let null_id = reader.read_u64();
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
            null_id: Id(null_id),
            unit_trie,
            prefix_trie,
            prefix_id_to_multiplier,
            unit_id_to_vector,
            prefix_data,
            unit_data,
            prefix_id_to_grouping,
            unit_id_to_grouping,
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
