# Purse: Bag data structure implementation in Rust
[![mac](https://github.com/rauljordan/purse/actions/workflows/mac.yml/badge.svg)](https://github.com/rauljordan/purse/actions/workflows/mac.yml)
[![linux](https://github.com/rauljordan/purse/actions/workflows/linux.yml/badge.svg)](https://github.com/rauljordan/purse/actions/workflows/linux.yml)

## Overview

Purse implements a [bag](https://www.cs.umd.edu/class/spring2017/cmsc132-050X/projects/BagsAndDenseTrees/doc/student_classes/Bag.html) in Rust, also known as a multi set, but that's a much more boring name. Bags are quite versatile structures, allowing storage of heterogeneous and non-unique collections of items, supporting different types and allowing duplicates.

## Key Features

- **Versatility:** Can contain any item, with duplicates and different types.
- **Mixed Collections:** Ideal for applications requiring a mix of different types.
- **Duplicate Handling:** Allows multiple instances of the same item.

### Installing

```
cargo add purse
```

## Usage 

```rust
use std::any::TypeId;
use purse::Purse;
let mut purse = purse::new();

purse.insert("hello");
purse.insert(42);
assert!(purse.contains("hello"));
assert!(purse.contains(42));
assert_eq!(purse.count::<&str>(), 1);

// Get all of type.
let strings: Vec<&&str> = purse.get_all_of_type();
assert_eq!(strings.len(), 4);
purse.insert("foo");
purse.insert("bar");
purse.insert("baz");
assert!(strings.contains(&&"foo"));
assert!(strings.contains(&&"bar"));
assert!(strings.contains(&&"baz"));

// Check the most common type.
assert_eq!(purse.most_common_type(), Some(TypeId::of::<&str>()));

// Clearing the bag.
purse.clear();
assert!(purse.is_empty());

// Add a few items again.
purse.insert(5);
purse.insert("foo");

// Iteration over all elements in the bag
let mut nums: Vec<i32> = vec![];
let mut strs: Vec<&str> = vec![];

purse.iter().for_each(|item| {
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

## Contributing

Contributions are welcome! Please follow the standard Rust conventions for pull requests.

## License

This project is licensed under either of

- Apache License, Version 2.0 (licenses/Apache-2.0)
- MIT license (licenses/MIT)

at your option.

The SPDX license identifier for this project is MIT OR Apache-2.0.