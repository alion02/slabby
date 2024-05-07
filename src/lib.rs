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

union Entry<T, K: Key> {
    val: ManuallyDrop<T>,
    next: K,
    uninit: (),
}

impl<T, K: Key> Entry<T, K> {
    #[inline]
    fn uninit() -> Self {
        Self { uninit: () }
    }
}

pub struct Slab<T, K: Key = u32> {
    entries: Box<[Entry<T, K>]>,
    next: K,
    len: K,
}

impl<T, K: Key> Slab<T, K> {
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self {
            entries: Box::new([]),
            next: K::ZERO,
            len: K::ZERO,
        }
    }

    #[inline(never)]
    fn extend(&mut self) {
        const INITIAL_SIZE: usize = 4;
        let ptr = &mut self.entries as *mut Box<[_]>;
        unsafe {
            let b = ptr::read(ptr);
            let extend_by = if b.len() == 0 { INITIAL_SIZE } else { b.len() };
            let mut vec = b.into_vec();
            vec.extend(repeat_with(Entry::uninit).take(extend_by));
            ptr::write(ptr, vec.into_boxed_slice());
        }
    }

    /// # Safety
    ///
    /// The number of occupied entries must be lower than the maximum value of `K`. This is
    /// trivially true if `K` is [`usize`].
    #[inline]
    pub unsafe fn insert(&mut self, val: T) -> K {
        let next = self.next;

        if next.as_usize() == self.entries.len() {
            self.extend();
        }

        let entry = unsafe { self.entries.get_unchecked_mut(next.as_usize()) };

        self.next = if self.next == self.len {
            self.next.inc()
        } else {
            unsafe { entry.next }
        };
        self.len = self.len.inc();

        entry.val = ManuallyDrop::new(val);

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
        let entry = unsafe { self.entries.get_unchecked_mut(key.as_usize()) };
        let entry = mem::replace(entry, Entry { next: self.next });

        self.next = key;
        self.len = self.len.dec();

        ManuallyDrop::into_inner(unsafe { entry.val })
    }

    /// # Safety
    ///
    /// The provided `key` must have been obtained from this instance of [`Slab`] and not removed
    /// between the insertion and this call.
    #[inline]
    pub unsafe fn get(&self, key: K) -> &T {
        unsafe { &self.entries.get_unchecked(key.as_usize()).val }
    }

    /// # Safety
    ///
    /// The provided `key` must have been obtained from this instance of [`Slab`] and not removed
    /// between the insertion and this call.
    #[inline]
    pub unsafe fn get_mut(&mut self, key: K) -> &mut T {
        unsafe { &mut self.entries.get_unchecked_mut(key.as_usize()).val }
    }
}

impl<T, K: Key> Default for Slab<T, K> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}
