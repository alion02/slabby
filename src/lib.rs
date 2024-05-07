#![no_std]

extern crate alloc;

mod key;

use alloc::boxed::Box;
use core::{
    iter::repeat_with,
    mem::{self, ManuallyDrop},
    ptr,
};

use key::Key;

union Slot<T, K: Key> {
    val: ManuallyDrop<T>,
    next: K,
    uninit: (),
}

impl<T, K: Key> Slot<T, K> {
    #[inline]
    fn uninit() -> Self {
        Self { uninit: () }
    }
}

pub struct Slab<T, K: Key = u32> {
    slots: Box<[Slot<T, K>]>,
    next: K,
    len: K,
}

impl<T, K: Key> Slab<T, K> {
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self {
            slots: Box::new([]),
            next: K::ZERO,
            len: K::ZERO,
        }
    }

    #[inline(never)]
    fn extend(&mut self) {
        const INITIAL_SIZE: usize = 4;
        let ptr = &mut self.slots as *mut Box<[_]>;
        unsafe {
            let b = ptr::read(ptr);
            let extend_by = if b.len() == 0 { INITIAL_SIZE } else { b.len() };
            let mut vec = b.into_vec();
            vec.extend(repeat_with(Slot::uninit).take(extend_by));
            ptr::write(ptr, vec.into_boxed_slice());
        }
    }

    /// # Safety
    ///
    /// The number of occupied slots must be lower than the maximum value of `K`. This is trivially
    /// true if `K` is [`usize`].
    #[inline]
    pub unsafe fn insert(&mut self, val: T) -> K {
        let next = self.next;

        if next.as_usize() == self.slots.len() {
            self.extend();
        }

        let slot = unsafe { self.slots.get_unchecked_mut(next.as_usize()) };

        self.next = if self.next == self.len {
            self.next.inc()
        } else {
            unsafe { slot.next }
        };
        self.len = self.len.inc();

        slot.val = ManuallyDrop::new(val);

        next
    }

    /// Remove a previously inserted element from the [`Slab`]. Returns the contained `T`.
    ///
    /// # Safety
    ///
    /// The provided `key` must have been obtained from this instance of [`Slab`] and not removed
    /// between the insertion and this call.
    #[inline]
    pub unsafe fn remove(&mut self, key: K) -> T {
        let slot = unsafe { self.slots.get_unchecked_mut(key.as_usize()) };
        let slot = mem::replace(slot, Slot { next: self.next });

        self.next = key;
        self.len = self.len.dec();

        ManuallyDrop::into_inner(unsafe { slot.val })
    }

    /// # Safety
    ///
    /// The provided `key` must have been obtained from this instance of [`Slab`] and not removed
    /// between the insertion and this call.
    #[inline]
    pub unsafe fn get(&self, key: K) -> &T {
        unsafe { &self.slots.get_unchecked(key.as_usize()).val }
    }

    /// # Safety
    ///
    /// The provided `key` must have been obtained from this instance of [`Slab`] and not removed
    /// between the insertion and this call.
    #[inline]
    pub unsafe fn get_mut(&mut self, key: K) -> &mut T {
        unsafe { &mut self.slots.get_unchecked_mut(key.as_usize()).val }
    }

    /// Get the number of elements contained within this [`Slab`].
    #[inline]
    pub fn len(&self) -> K {
        self.len
    }
}

impl<T, K: Key> Default for Slab<T, K> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_ops() {
        let mut slab = Slab::<u32, u32>::new();
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
