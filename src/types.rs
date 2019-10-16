use evmap::{self, ReadHandle, WriteHandle};
use std::hash::Hasher;
use std::marker::PhantomData;
#[derive(Copy, Clone, Debug, Default)]
pub struct U64TransparentHasher(u64, PhantomData<u64>);

impl Hasher for U64TransparentHasher {
    fn finish(&self) -> u64 {
        self.0
    }
    fn write(&mut self, _: &[u8]) {
        panic!("Hasher Error. Please use write_u64().")
    }
    fn write_u64(&mut self, n: u64) {
        self.0 = n
    }
}

pub type HasherType = ::std::hash::BuildHasherDefault<U64TransparentHasher>;
pub type Reader = ReadHandle<u64, u64, (), HasherType>;
pub type Writer = WriteHandle<u64, u64, (), HasherType>;
