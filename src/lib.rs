//! Maximally efficient allocation and deallocation of a large number of instances of a type.

#![no_std]

extern crate alloc;

mod key;
mod slab;

pub use slab::Slab;

/// A [`Slab`] which can hold up to 255 elements.
pub type Slab8<T> = Slab<T, u8>;
/// A [`Slab`] which can hold up to 65535 elements.
pub type Slab16<T> = Slab<T, u16>;
/// A [`Slab`] which can hold up to 4294967295 elements.
pub type Slab32<T> = Slab<T, u32>;
/// A [`Slab`] which can hold as many elements as the underlying [`Vec`](alloc::vec::Vec).
pub type SlabSize<T> = Slab<T, usize>;
