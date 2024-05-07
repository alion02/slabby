This crate provides maximally efficient allocation and deallocation of a large number of instances of a single type.

You can choose the size of the key for each instance of a Slab, which allows you to use a type smaller than a pointer to hold the indices to your items, which can improve memory locality, at the cost of imposing a limit on the number of elements that can be stored.

Due to the design of this Slab being optimized for memory usage and efficiency of the common cases (adding, removing, and retrieving elements) it liberally uses unsafe code, cannot provide a safe user-facing API, and some operations are more expensive than expected. All such operations are documented.

# Usage

```rs
let mut slab = slabby::Slab32::new();
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
```

