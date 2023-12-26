# Pouch: Bag data structure implementation in Rust with lock-free, atomic features
[![mac](https://github.com/rauljordan/pouch/actions/workflows/mac.yml/badge.svg)](https://github.com/rauljordan/pouch/actions/workflows/mac.yml)
[![linux](https://github.com/rauljordan/pouch/actions/workflows/linux.yml/badge.svg)](https://github.com/rauljordan/pouch/actions/workflows/linux.yml)

## Overview

Pouch implements a [bag](https://www.cs.umd.edu/class/spring2017/cmsc132-050X/projects/BagsAndDenseTrees/doc/student_classes/Bag.html) in Rust, also known as a multi set, but that's a much more boring name. Bags are quite versatile structures, allowing storage of heterogeneous and non-unique collections of items, supporting different types and allowing duplicates.

The Pouch crate provides an atomic, lock-free version of a bag via the `atomic` feature, which is thread-safe and backed by the popular [dashmap](https://github.com/xacrimon/dashmap).

## Key Features

- **Versatility:** Can contain any item, with duplicates and different types.
- **Mixed Collections:** Ideal for applications requiring a mix of different types.
- **Duplicate Handling:** Allows multiple instances of the same item.

## Crate Variants

Pouch comes in two flavors:

- **Standard Implementation:** Utilizes HashMap from Rust's standard library.
- **Atomic Variant:** Enabled by the `atomic` feature flag, uses DashMap for thread-safe operations without locks.

### Requirements

Types stored must implement the `Any` trait. For the atomic variant, types also need `Send` and `Sync`.

## Usage 

```rust
use std::any::TypeId;
use pouch::Pouch;
let mut pouch = Pouch::new();

pouch.insert("hello");
pouch.insert(42);
assert!(pouch.contains("hello"));
assert!(pouch.contains(42));
assert_eq!(pouch.count::<&str>(), 1);

// Get all of type.
let strings: Vec<&&str> = pouch.get_all_of_type();
assert_eq!(strings.len(), 4);
pouch.insert("foo");
pouch.insert("bar");
pouch.insert("baz");
assert!(strings.contains(&&"foo"));
assert!(strings.contains(&&"bar"));
assert!(strings.contains(&&"baz"));

// Check the most common type.
assert_eq!(pouch.most_common_type(), Some(TypeId::of::<&str>()));

// Clearing the bag.
pouch.clear();
assert!(pouch.is_empty());

// Add a few items again.
pouch.insert(5);
pouch.insert("foo");

// Iteration over all elements in the bag
let mut nums: Vec<i32> = vec![];
let mut strs: Vec<&str> = vec![];

pouch.iter().for_each(|item| {
if let Some(&t) = item.downcast_ref::<i32>() {
    nums.push(t);
} else if let Some(&t) = item.downcast_ref::<&str>() {
    strs.push(t);
} else {
    panic!("unexpected type found in bag");
}
});
assert_eq!(nums.first(), Some(&5));
assert_eq!(strs.first(), Some(&"foo"));
```

Add pouch to your Cargo.toml:

```toml
[dependencies]
pouch = "0.1.0"
```

or enable the atomic feature for thread safety:

```toml
[dependencies.pouch]
version = "0.1.0"
features = ["atomic"] # Enable the atomic variant of the pouch
```

## Contributing

Contributions are welcome! Please follow the standard Rust conventions for pull requests.

## License

This project is licensed under either of

- Apache License, Version 2.0 (licenses/Apache-2.0)
- MIT license (licenses/MIT)

at your option.

The SPDX license identifier for this project is MIT OR Apache-2.0.