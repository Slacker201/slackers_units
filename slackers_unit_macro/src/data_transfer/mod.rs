use std::{array::from_fn, hash::{DefaultHasher, Hash, Hasher}};

use num_rational::Ratio;
use num_traits::{ConstZero, FromPrimitive};
use proc_macro2::Span;
use quote::{ToTokens, quote};
use syn::{LitByteStr, LitInt, parse::Parse};

use crate::unit_system::NumberRepr;




#[derive(Debug, Clone, PartialEq, Hash)]
pub(crate) struct ParserData {
    dimension_count: usize,
    unit_vectors: Box<[(String, Box<[i64]>)]>,
    prefix_multipliers: Box<[(String, Ratio<NumberRepr>)]>,
}

struct Writer<'a> {
    buf: &'a mut [u8],
    pos: usize,
}

impl<'a> Writer<'a> {
    #[inline]
    fn write_usize(&mut self, v: usize) {
        let bytes = v.to_be_bytes();
        self.write_slice(&bytes);
    }
    #[inline]
    fn write_slice(&mut self, slice: &[u8]) {
        let end = self.pos + slice.len();
        self.buf[self.pos..end].copy_from_slice(slice);
        self.pos = end;
    }
    #[inline]
    fn write_vector(&mut self, vec: &Box<[i64]>) {
        for dim in vec {
            self.write_slice(&dim.to_be_bytes());
        }
    }
    #[inline]
    fn write_str(&mut self, s: &str) {
        self.write_slice(s.as_bytes());
    }
    #[inline]
    fn write_len_prefixed_str(&mut self, s: &str) {
        self.write_usize(s.len());
        self.write_str(s);
    } 
}

struct Reader<'a> {
    buf: &'a [u8],
    pos: usize,
}

impl<'self_lifetime, 'slice_lifetime> Reader<'slice_lifetime> {
    fn read_usize(&mut self) -> usize {
        let bytes: [u8; size_of::<usize>()] = from_fn(|idx| self.buf[self.pos + idx]);
        self.pos += size_of::<usize>();
        usize::from_be_bytes(bytes)
    }
    fn read_array<const N: usize>(&mut self) -> [u8; N] {
        let b = from_fn(|idx| self.buf[self.pos + idx]);
        self.pos += N;
        b
    }
    fn read_slice(&'self_lifetime mut self, size: usize) -> &'slice_lifetime [u8] {
        let r = &self.buf[self.pos..self.pos + size];
        self.pos += size;
        r
    }
    fn read_prefixed_string(&'self_lifetime mut self) -> &'slice_lifetime str {
        let len = self.read_usize();
        let slice = self.read_slice(len);
        str::from_utf8(slice).expect("Expected utf8 string")
    }
}

impl ParserData {
    pub(crate) fn new(dim_count: usize, unit_vectors: Box<[(String, Box<[i64]>)]>, prefix_multipliers: Box<[(String, Ratio<NumberRepr>)]>) -> Self {
        Self { dimension_count: dim_count, unit_vectors, prefix_multipliers }
    }
    pub(crate) fn to_bytes(&self) -> Box<[u8]> {
        let mut size = 0;
        // add size of dimension count
        size += size_of::<usize>();
        // add unit vector count
        size += size_of::<usize>();
        // add the size of unit abbreviations and length prefixes
        size += self.unit_vectors.iter().fold(0, |size, (abbrev, _)| size + abbrev.len() + size_of::<usize>());
        // add the size of unit vectors
        size += self.unit_vectors.len() * self.dimension_count * size_of::<i64>();
        // add prefix count
        size += size_of::<usize>();
        // add the size of prefix abbreviations and length prefixes
        size += self.prefix_multipliers.iter().fold(0, |size, (abbrev, _)| size + abbrev.len() + size_of::<usize>());
        // add the size of NumberRepr ratio
        size += self.prefix_multipliers.len() * size_of::<NumberRepr>() * 2;
        let mut vec = Vec::with_capacity(size);
        unsafe { vec.set_len(size); }
        let mut writer: Writer = Writer { buf: &mut vec, pos: 0 };

        writer.write_usize(
            self.dimension_count
        );
        writer.write_usize(
            self.unit_vectors.len()
        );
        for (abbrev, vec) in &self.unit_vectors {
            writer.write_len_prefixed_str(abbrev);
            writer.write_vector(vec);
        }
        writer.write_usize(
            self.prefix_multipliers.len()
        );
        for (abbrev, ratio) in &self.prefix_multipliers {
            writer.write_len_prefixed_str(abbrev);
            let (numer, denom) = ratio.into_raw();
            writer.write_slice(&numer.to_be_bytes());
            writer.write_slice(&denom.to_be_bytes());
        }
        vec.into_boxed_slice()
    }

    pub(crate) fn from_bytes(buf: &[u8]) -> Self {
        let mut reader = Reader {
            buf,
            pos: 0
        };
        let dim_count = reader.read_usize();
        let unit_count = reader.read_usize();
        let mut units = Vec::with_capacity(unit_count);
        for _ in 0..unit_count {
            let abbrev = reader.read_prefixed_string();
            let mut unit_vector = Vec::with_capacity(dim_count);
            for _ in 0..dim_count {
                unit_vector.push(i64::from_be_bytes(reader.read_array()));
            }
            units.push((abbrev.to_string(), unit_vector.into_boxed_slice()));
        }
        let prefix_count = reader.read_usize();
        let mut prefixes = Vec::with_capacity(prefix_count);
        for _ in 0..prefix_count {
            let abbrev = reader.read_prefixed_string();
            let numer = reader.read_array::<{size_of::<NumberRepr>()}>();
            let denom = reader.read_array::<{size_of::<NumberRepr>()}>();
            let numer = NumberRepr::from_be_bytes(numer);
            let denom = NumberRepr::from_be_bytes(denom);
            prefixes.push((abbrev.to_string(), Ratio::new(numer, denom)));
        }
        Self {
            dimension_count: dim_count,
            unit_vectors: units.into_boxed_slice(),
            prefix_multipliers: prefixes.into_boxed_slice(),
        }
    }
}

impl ToTokens for ParserData {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let byte_lit = LitByteStr::new(&self.to_bytes(), Span::call_site());
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        let hash = hasher.finish();
        let literal = LitInt::new(&hash.to_string(), Span::call_site());
        tokens.extend(quote! { #literal #byte_lit });
    }
}

impl Parse for ParserData {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let literal = input.parse::<LitByteStr>()?;
        let bytes = literal.value();
        Ok(
            Self::from_bytes(&bytes)
        )
    }
}