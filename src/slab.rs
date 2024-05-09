use alloc::boxed::Box;
use core::{
    fmt,
    mem::{self, ManuallyDrop},
    ptr,
};

use crate::key::Key;

union Slot<T, K: Key> {
    val: ManuallyDrop<T>,
    next: K,
    _uninit: (),
}

/// A [`Slab`] which can hold some number of elements, depending on the chosen `K`.
///
/// # Leaks
///
/// This type does **not** track elements. In particular, this means that custom [`Drop`] logic
/// will not run for elements not [`removed`](Slab::remove) from the [`Slab`]. This is analogous
/// to a standard memory leak in a conventional allocator.
///
/// # Examples
///
/// ```
/// let mut slab = slabby::Slab32::new();
/// unsafe {
///     let key1 = slab.insert(1);
///     let key2 = slab.insert(2);
///     let key3 = slab.insert(3);
///
///     assert_eq!(slab.get(key1), &1);
///     assert_eq!(slab.get(key2), &2);
///     assert_eq!(slab.get(key3), &3);
///
///     assert_eq!(slab.remove(key2), 2);
///     assert_eq!(slab.remove(key1), 1);
///
///     assert_eq!(slab.get(key3), &3);
///
///     slab.insert(4);
///     let key5 = slab.insert(5);
///     slab.insert(6);
///
///     assert_eq!(slab.len(), 4);
///
///     *slab.get_mut(key5) += 1;
///     assert_eq!(slab.remove(key5), 6);
///
///     assert_eq!(slab.len(), 3);
/// }
/// ```
pub struct Slab<T, K: Key> {
    slots: Box<[Slot<T, K>]>,
    next: K,
    len: K,
}

impl<T, K: Key> Slab<T, K> {
    /// Create a new [`Slab`]. No allocations will occur until the first [`insert`](Slab::insert).
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
        let ptr: *mut _ = &mut self.slots;
        unsafe {
            let b = ptr::read(ptr);
            let extend_by = if b.len() == 0 { INITIAL_SIZE } else { b.len() };
            let mut vec = b.into_vec();
            vec.reserve_exact(extend_by);
            vec.set_len(vec.capacity());
            ptr::write(ptr, vec.into_boxed_slice());
        }
    }

    /// # Safety
    ///
    /// The number of occupied slots must be lower than the maximum value of `K`. This is trivially
    /// true if the maximum value of `K` is greater or equal to that of [`usize`].
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
    #[must_use]
    pub unsafe fn get(&self, key: K) -> &T {
        unsafe { &self.slots.get_unchecked(key.as_usize()).val }
    }

    /// # Safety
    ///
    /// The provided `key` must have been obtained from this instance of [`Slab`] and not removed
    /// between the insertion and this call.
    #[inline]
    #[must_use]
    pub unsafe fn get_mut(&mut self, key: K) -> &mut T {
        unsafe { &mut self.slots.get_unchecked_mut(key.as_usize()).val }
    }

    /// Get the key of the next element to be inserted into this [`Slab`].
    #[inline]
    #[must_use]
    pub fn next(&self) -> K {
        self.next
    }

    /// Get the number of elements contained within this [`Slab`].
    #[inline]
    #[must_use]
    pub fn len(&self) -> K {
        self.len
    }
}

impl<T, K: Key> Default for Slab<T, K> {
    #[inline]
    #[must_use]
    fn default() -> Self {
        Self::new()
    }
}

impl<T, K: Key> fmt::Debug for Slab<T, K> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Slab")
            .field("next", &self.next)
            .field("len", &self.len)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn does_not_forget_list() {
        let mut slab = crate::Slab32::new();
        unsafe {
            let [a, _, c, d] = [10, 11, 12, 13].map(|v| slab.insert(v));
            slab.remove(a);
            slab.remove(c);
            slab.insert(14);
            assert_ne!(slab.insert(15), d);
        }
    }
}
