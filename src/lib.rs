//! Maximally efficient allocation and deallocation of a large number of instances of a type.

#![no_std]

extern crate alloc;

mod key;
mod slab;

pub use slab::Slab;

pub type Slab8<T> = Slab<T, u8>;
pub type Slab16<T> = Slab<T, u16>;
pub type Slab32<T> = Slab<T, u32>;
pub type SlabSize<T> = Slab<T, usize>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_ops() {
        let mut slab = Slab32::new();
        unsafe {
            let key1 = slab.insert(1);
            let key2 = slab.insert(2);
            let key3 = slab.insert(3);

            assert_eq!(slab.get(key1), &1);
            assert_eq!(slab.get(key2), &2);
            assert_eq!(slab.get(key3), &3);

            assert_eq!(slab.remove(key2), 2);
            assert_eq!(slab.remove(key1), 1);

            assert_eq!(slab.get(key3), &3);

            slab.insert(4);
            let key5 = slab.insert(5);
            slab.insert(6);

            assert_eq!(slab.len(), 4);

            *slab.get_mut(key5) += 1;
            assert_eq!(slab.remove(key5), 6);

            assert_eq!(slab.len(), 3);
        }
    }
}
