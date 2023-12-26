//! [`Pouch`](Pouch) implements a bag data structure, also known as a multiset. A bag
//! can contain anything, with possible duplicates of each item, and items of
//! different types - it's a free for all!
//!
//! [`Pouch`](Pouch) is helpful in a wide range of applications where a heterogeneous
//! and non-unique collection of items is required.
//!
//! The crate comes in two flavors:
//! - A standard implementation using a standard [`HashMap`](std::collections::HashMap).
//! - An atomic variant enabled by the `atomic` feature flag, which uses the [`DashMap`](https://github.com/xacrimon/dashmap) crate for thread-safe
//!   access without locks.
//!
//! The lock-free, atomic version of the bag only requires immutable references, allowing
//! it to be sent across threads safely without using mutexes underneath the hood.
//!
//! Types stored in [`Pouch`](Pouch) must implement the [`Any`](std::any::Any) trait. When using the atomic variant, types
//! must also implement [`Send`](Send) and [`Sync`](Sync).
//!
//! # Examples
//!
//! Basic usage:
//!
//! ```
//! use pouch::Pouch;
//! let mut pouch = Pouch::new();
//!
//! pouch.insert("hello");
//! pouch.insert(42);
//! assert!(pouch.contains("hello"));
//! assert!(pouch.contains(42));
//! assert_eq!(pouch.count::<&str>(), 1);
//!
//! // Get all of type.
//! let strings: Vec<&&str> = pouch.get_all_of_type();
//! assert_eq!(strings.len(), 4);
//! pouch.insert("foo");
//! pouch.insert("bar");
//! pouch.insert("baz");
//! assert!(strings.contains(&&"foo"));
//! assert!(strings.contains(&&"bar"));
//! assert!(strings.contains(&&"baz"));
//!
//! // Check the most common type.
//! assert_eq!(pouch.most_common_type(), Some(TypeId::of::<&str>()));
//!
//! // Clearing the bag.
//! pouch.clear();
//! assert!(pouch.is_empty());
//!
//! // Add a few items again.
//! pouch.insert(5);
//! pouch.insert("foo");
//!
//! // Iteration over all elements in the bag
//! let mut nums: Vec<i32> = vec![];
//! let mut strs: Vec<&str> = vec![];
//!
//! pouch.iter().for_each(|item| {
//!     if let Some(&t) = item.downcast_ref::<i32>() {
//!         nums.push(t);
//!     } else if let Some(&t) = item.downcast_ref::<&str>() {
//!         strs.push(t);
//!     } else {
//!         panic!("unexpected type found in bag");
//!     }
//! });
//! assert_eq!(nums.first(), Some(&5));
//! assert_eq!(strs.first(), Some(&"foo"));
//! ```

use std::any::{Any, TypeId};

#[cfg(feature = "atomic")]
use dashmap::DashMap;
#[cfg(not(feature = "atomic"))]
use std::collections::HashMap;

#[cfg_attr(feature = "atomic", doc = "Atomic, lock-free variant of `Pouch`")]
#[cfg_attr(
    not(feature = "atomic"),
    doc = "Standard, non-thread-safe variant of `Pouch`."
)]
#[derive(Default, Debug)]
pub struct Pouch {
    #[cfg(feature = "atomic")]
    data: DashMap<TypeId, Vec<Box<dyn Any + Send + Sync>>>,
    #[cfg(not(feature = "atomic"))]
    data: HashMap<TypeId, Vec<Box<dyn Any>>>,
    #[cfg(not(feature = "atomic"))]
    counts: HashMap<TypeId, u64>,
}

impl Pouch {
    #[cfg(feature = "atomic")]
    pub fn new() -> Self {
        Self {
            data: DashMap::default(),
        }
    }
    #[cfg(not(feature = "atomic"))]
    pub fn new() -> Self {
        Self {
            data: HashMap::default(),
            counts: HashMap::default(),
        }
    }
    /// Checks if the pouch is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pouch::Pouch;
    /// let mut pouch = Pouch::new();
    /// assert!(pouch.is_empty());
    /// pouch.insert("apple");
    /// assert!(!pouch.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    /// Provides an iterator over all elements in the pouch.
    ///
    /// This method returns a boxed iterator that yields references to each element stored in the pouch,
    /// regardless of their type. It iterates over all elements in a non-specific order.
    ///
    /// Note: This method is available only when the `atomic` feature is not enabled.
    ///
    /// # Returns
    /// A `Box<dyn Iterator<Item = &dyn Any> + '_>` that can be used to iterate over all elements
    /// in the pouch.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pouch::Pouch;
    /// # let mut pouch = Pouch::new();
    /// # pouch.insert(42);
    /// # pouch.insert("hello");
    /// for item in pouch.iter() {
    ///     // Process each item
    /// }
    /// ```
    #[cfg(not(feature = "atomic"))]
    pub fn iter(&self) -> Box<dyn Iterator<Item = &dyn Any> + '_> {
        Box::new(self.data.values().flatten().map(|b| &**b))
    }
    /// Retrieves all elements of a specific type from the pouch.
    ///
    /// This method returns a vector containing references to all elements of the type specified
    /// by the generic parameter `T`. It filters the elements in the pouch based on their type.
    ///
    /// Note: This method is available only when the `atomic` feature is not enabled.
    ///
    /// # Returns
    /// A `Vec<&T>` containing references to all elements of type `T` in the pouch.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pouch::Pouch;
    /// # let mut pouch = Pouch::new();
    /// # pouch.insert(42);
    /// # pouch.insert(42);
    /// let numbers: Vec<&i32> = pouch.get_all_of_type();
    /// assert_eq!(numbers.len(), 2);
    /// assert_eq!(*numbers[0], 42);
    /// assert_eq!(*numbers[1], 42);
    /// ```
    #[cfg(not(feature = "atomic"))]
    pub fn get_all_of_type<T: Any>(&self) -> Vec<&T> {
        let type_id = TypeId::of::<T>();
        self.data.get(&type_id).map_or(Vec::new(), |elems| {
            elems
                .iter()
                .filter_map(|el| el.downcast_ref::<T>())
                .collect()
        })
    }
    /// Checks if the pouch contains a given element.
    ///
    /// This method searches the pouch for an element equal to `t` and returns `true` if it is found.
    ///
    /// # Atomic version
    /// For the `atomic` feature, this method signature is:
    /// `pub fn contains<T: Any + Eq>(&self, t: T) -> bool`
    /// It operates on `DashMap`, making it thread-safe. This version does not require mutable access.
    ///
    /// # Non-Atomic version
    /// For the standard version without the `atomic` feature, the method signature is:
    /// `pub fn contains<T: Any + Eq>(&self, t: T) -> bool`
    /// This version operates on a `HashMap` and requires immutable access to the pouch.
    ///
    /// # Type Parameters
    /// - `T`: The type of the element to check. This type must implement `Any` and `Eq`.
    ///
    /// # Arguments
    /// - `t`: The element to check for in the pouch.
    ///
    /// # Returns
    /// Returns `true` if the element is found in the pouch, otherwise `false`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pouch::Pouch;
    /// # #[cfg(feature = "atomic")]
    /// # {
    /// let pouch = Pouch::new();
    /// pouch.insert("apple");
    /// assert!(pouch.contains("apple"));
    /// # }
    /// # #[cfg(not(feature = "atomic"))]
    /// # {
    /// # let mut pouch = Pouch::new();
    /// # pouch.insert("apple");
    /// # assert!(pouch.contains("apple"));
    /// # }
    /// ```
    #[cfg(not(feature = "atomic"))]
    pub fn contains<T: Any + Eq>(&self, t: T) -> bool {
        let type_id = TypeId::of::<T>();
        let Some(elems) = self.data.get(&type_id) else {
            return false;
        };
        for elem in elems {
            if let Some(elem) = elem.downcast_ref::<T>() {
                if elem == &t {
                    return true;
                }
            }
        }
        false
    }
    #[cfg(feature = "atomic")]
    pub fn contains<T: Any + Eq>(&self, t: T) -> bool {
        let type_id = TypeId::of::<T>();
        let Some(elems) = self.data.get(&type_id) else {
            return false;
        };
        let elems = elems.value();
        for elem in elems {
            if let Some(elem) = elem.downcast_ref::<T>() {
                if elem == &t {
                    return true;
                }
            }
        }
        false
    }
    /// Retrieves a list of `TypeId`s of the types currently stored in the pouch.
    ///
    /// This method returns a vector containing the `TypeId` of each unique type currently stored in the pouch.
    ///
    /// # Atomic version
    /// For the `atomic` feature, this method signature is:
    /// `pub fn types(&self) -> Vec<TypeId>`
    /// It iterates over the `DashMap` to collect the type identifiers. The operation is thread-safe.
    ///
    /// # Non-Atomic version
    /// For the standard version without the `atomic` feature, the method signature is:
    /// `pub fn types(&self) -> Vec<TypeId>`
    /// It iterates over a `HashMap` to collect the type identifiers.
    ///
    /// # Returns
    /// A `Vec<TypeId>` containing the unique type identifiers of all elements stored in the pouch.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pouch::Pouch;
    /// # use std::any::{Any, TypeId};
    /// # #[cfg(feature = "atomic")]
    /// # {
    /// let pouch = Pouch::new();
    /// pouch.insert(42);
    /// pouch.insert("apple");
    /// let types = pouch.types();
    /// assert_eq!(types, vec![TypeId::of::<i32>(), TypeId::of::<&str>()]);
    /// # }
    /// # #[cfg(not(feature = "atomic"))]
    /// # {
    /// # let mut pouch = Pouch::new();
    /// # pouch.insert(42u64);
    /// # pouch.insert("apple");
    /// # let types = pouch.types();
    /// # assert!(types.contains(&TypeId::of::<&str>()));
    /// # }
    /// ```
    #[cfg(not(feature = "atomic"))]
    pub fn types(&self) -> Vec<TypeId> {
        self.data.keys().cloned().collect()
    }
    #[cfg(feature = "atomic")]
    pub fn types(&self) -> Vec<TypeId> {
        self.data.iter().map(|r| r.key().clone()).collect()
    }
    /// Counts the number of elements of a specific type in the pouch.
    ///
    /// This method returns the number of elements of the type specified by the generic parameter `T`.
    ///
    /// # Atomic version
    /// For the `atomic` feature, this method signature is:
    /// `pub fn count<T: Any>(&self) -> u64`
    /// It iterates over the `DashMap` to count the elements of the specified type.
    ///
    /// Note: the atomic version of this method is slow, as it has O(N) runtime complexity
    /// where N is the number of elements of a certain type T. In the non-atomic version,
    /// a mapping of counts is stored so checking counts will be O(1) fast.
    ///
    /// # Non-Atomic version
    /// For the standard version without the `atomic` feature, the method signature is:
    /// `pub fn count<T: Any>(&self) -> u64`
    /// It accesses a `HashMap` that tracks counts of each type to retrieve the count.
    ///
    /// # Type Parameters
    /// - `T`: The type for which to count the occurrences. This type must implement the `Any` trait.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pouch::Pouch;
    /// # use std::any::TypeId;
    /// # #[cfg(feature = "atomic")]
    /// # {
    /// let pouch = Pouch::new();
    /// pouch.insert(42);
    /// pouch.insert(42);
    /// pouch.insert("hello");
    /// assert_eq!(pouch.count::<i32>(), 2);
    /// # }
    /// # #[cfg(not(feature = "atomic"))]
    /// # {
    /// # let mut pouch = Pouch::new();
    /// # pouch.insert(42);
    /// # pouch.insert(42);
    /// # pouch.insert("hello");
    /// # assert_eq!(pouch.count::<i32>(), 2);
    /// # }
    /// ```
    #[cfg(not(feature = "atomic"))]
    pub fn count<T: Any>(&self) -> u64 {
        let type_id = TypeId::of::<T>();
        *self.counts.get(&type_id).unwrap_or(&0)
    }
    #[cfg(feature = "atomic")]
    pub fn count<T: Any>(&self) -> u64 {
        let type_id = TypeId::of::<T>();
        let Some(elems) = self.data.get(&type_id) else {
            return 0;
        };
        elems.len() as u64
    }
    /// Determines the most common type stored in the pouch.
    ///
    /// This method returns the `TypeId` of the most frequently occurring type.
    ///
    /// # Atomic version
    /// For the `atomic` feature, this method signature is:
    /// `pub fn most_common_type(&self) -> Option<TypeId>`
    /// It iterates over the `DashMap` to find the type with the most elements.
    ///
    /// # Non-Atomic version
    /// For the standard version without the `atomic` feature, the method signature is:
    /// `pub fn most_common_type(&self) -> Option<TypeId>`
    /// It checks the `HashMap` of counts to find the most common type.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pouch::Pouch;
    /// use std::any::TypeId;
    /// # #[cfg(feature = "atomic")]
    /// # {
    /// let pouch = Pouch::new();
    /// pouch.insert(42);
    /// pouch.insert(42);
    /// pouch.insert("hello");
    /// assert_eq!(pouch.most_common_type(), Some(TypeId::of::<i32>()));
    /// # }
    /// # #[cfg(not(feature = "atomic"))]
    /// # {
    /// # let mut pouch = Pouch::new();
    /// # pouch.insert(42);
    /// # pouch.insert("hello");
    /// # pouch.insert("world");
    /// # assert_eq!(pouch.most_common_type(), Some(TypeId::of::<&str>()));
    /// # }
    /// ```
    #[cfg(not(feature = "atomic"))]
    pub fn most_common_type(&self) -> Option<TypeId> {
        self.counts.keys().max().copied()
    }
    #[cfg(feature = "atomic")]
    pub fn most_common_type(&self) -> Option<TypeId> {
        self.data
            .iter()
            .max_by_key(|r| r.value().len())
            .map(|r| *r.key())
    }
    /// Inserts an element into the pouch.
    ///
    /// # Non-Atomic version
    /// For the standard version without the `atomic` feature, the method signature is:
    /// `pub fn insert<T: Any>(&mut self, elem: T)`
    /// This method requires mutable access to the pouch.
    ///
    /// # Atomic version
    /// For the `atomic` feature, the method signature is:
    /// `pub fn insert<T: Any + Send + Sync>(&self, elem: T)`
    /// insertion requires the type to insert has `Send + Sync`, as it must be possible for
    /// the type to be shared across threads.
    /// The atomic version is thread-safe and does not require mutable access.
    ///
    /// # Examples
    /// ```
    /// # use pouch::Pouch;
    /// let mut pouch = Pouch::new();
    /// pouch.insert(42);
    /// assert!(pouch.contains(42));
    /// ```
    #[cfg(not(feature = "atomic"))]
    pub fn insert<T: Any>(&mut self, elem: T) {
        let type_id = TypeId::of::<T>();
        self.data.entry(type_id).or_default().push(Box::new(elem));

        *self.counts.entry(type_id).or_insert(0) += 1;
    }
    #[cfg(feature = "atomic")]
    pub fn insert<T: Any + Send + Sync>(&self, elem: T) {
        let type_id = TypeId::of::<T>();
        self.data
            .entry(type_id)
            .or_insert_with(Vec::new)
            .push(Box::new(elem));
    }
    /// Removes a single occurrence of an element from the pouch, if present.
    ///
    /// This method looks for an element equal to `elem` and removes the first occurrence it finds.
    ///
    /// # Atomic version
    /// For the `atomic` feature, this method signature is:
    /// `pub fn remove<T: Any + Eq>(&self, elem: T) -> bool`
    /// It operates on `DashMap`, making it thread-safe. This version does not require mutable access.
    ///
    /// # Non-Atomic version
    /// For the standard version without the `atomic` feature, the method signature is:
    /// `pub fn remove<T: Any + Eq>(&mut self, elem: T) -> bool`
    /// This version operates on a `HashMap` and requires mutable access to the pouch.
    ///
    /// # Type Parameters
    /// - `T`: The type of the element to remove. This type must implement `Any` and `Eq`.
    ///
    /// # Arguments
    /// - `elem`: The element to remove from the pouch.
    ///
    /// # Returns
    /// Returns `true` if an element was removed, or `false` if no such element was found.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pouch::Pouch;
    /// # #[cfg(feature = "atomic")]
    /// # {
    /// let pouch = Pouch::new();
    /// pouch.insert("apple");
    /// assert!(pouch.remove("apple"));
    /// assert!(!pouch.contains(&"apple"));
    /// # }
    /// # #[cfg(not(feature = "atomic"))]
    /// # {
    /// # let mut pouch = Pouch::new();
    /// # pouch.insert("apple");
    /// # assert!(pouch.remove("apple"));
    /// # assert!(!pouch.contains(&"apple"));
    /// # }
    /// ```
    #[cfg(not(feature = "atomic"))]
    pub fn remove<T: Any + Eq>(&mut self, elem: T) -> bool {
        let type_id = TypeId::of::<T>();
        if let Some(elems) = self.data.get_mut(&type_id) {
            if let Some(index) = elems
                .iter()
                .position(|el| el.downcast_ref::<T>() == Some(&elem))
            {
                elems.remove(index);
                *self.counts.entry(type_id).or_insert(0) =
                    self.counts.entry(type_id).or_insert(0).saturating_sub(1);
                return true;
            }
        }
        false
    }
    #[cfg(feature = "atomic")]
    pub fn remove<T: Any + Eq>(&self, elem: T) -> bool {
        let type_id = TypeId::of::<T>();
        if let Some(mut elems) = self.data.get_mut(&type_id) {
            if let Some(index) = elems
                .iter()
                .position(|el| el.downcast_ref::<T>() == Some(&elem))
            {
                elems.remove(index);
                return true;
            }
        }
        false
    }
    /// Clears all elements from the pouch.
    ///
    /// This method removes all elements from the pouch, effectively resetting it to its initial state.
    ///
    /// # Atomic version
    /// For the `atomic` feature, this method signature is:
    /// `pub fn clear(&self)`
    /// It clears the `DashMap` and does not require mutable access to the pouch, maintaining thread safety.
    ///
    /// # Non-Atomic version
    /// For the standard version without the `atomic` feature, the method signature is:
    /// `pub fn clear(&mut self)`
    /// It clears both the `HashMap` storing the elements and the `HashMap` tracking the counts, requiring mutable access to the pouch.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pouch::Pouch;
    /// # #[cfg(feature = "atomic")]
    /// # {
    /// let pouch = Pouch::new();
    /// pouch.insert(42);
    /// pouch.clear();
    /// assert!(pouch.is_empty());
    /// # }
    /// # #[cfg(not(feature = "atomic"))]
    /// # {
    /// # let mut pouch = Pouch::new();
    /// # pouch.insert(42);
    /// # pouch.clear();
    /// # assert!(pouch.is_empty());
    /// # }
    /// ```
    #[cfg(not(feature = "atomic"))]
    pub fn clear(&mut self) {
        self.data.clear();
        self.counts.clear();
    }
    #[cfg(feature = "atomic")]
    pub fn clear(&self) {
        self.data.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "atomic")]
    #[test]
    fn test_thread_safe() {
        use std::sync::Arc;
        use std::thread;
        let pouch = Arc::new(Pouch::new());

        let mut handles = vec![];
        for i in 0..10i32 {
            let pouch = pouch.clone();
            handles.push(thread::spawn(move || {
                pouch.insert(i);
            }));
        }
        for handle in handles {
            handle.join().unwrap();
        }
        assert_eq!(pouch.count::<i32>(), 10);
    }

    #[test]
    fn test_pouch() {
        #[derive(PartialEq, Eq, Clone, Copy, Debug)]
        enum RPS {
            Rock,
            Paper,
            Scissors,
        }
        #[cfg(feature = "atomic")]
        let pouch = Pouch::new();
        #[cfg(not(feature = "atomic"))]
        let mut pouch = Pouch::new();

        pouch.insert(5);
        pouch.insert("foo");
        pouch.insert(RPS::Rock);
        pouch.insert(RPS::Paper);

        // Check if items are contained.
        assert!(pouch.contains("foo"));
        assert!(!pouch.contains("bar"));
        assert!(!pouch.contains(0));
        assert!(pouch.contains(RPS::Rock));
        assert!(!pouch.contains(RPS::Scissors));

        // Check removal of items.
        pouch.remove(RPS::Rock);
        assert!(!pouch.contains(RPS::Rock));

        // Check counts.
        assert_eq!(pouch.count::<&str>(), 1);
        pouch.insert("bar");
        pouch.insert("baz");
        assert_eq!(pouch.count::<&str>(), 3);

        // Check types.
        let types = pouch.types();
        assert_eq!(types.len(), 3);
        assert!(types.contains(&TypeId::of::<i32>()));
        assert!(types.contains(&TypeId::of::<&str>()));
        assert!(types.contains(&TypeId::of::<RPS>()));

        #[cfg(not(feature = "atomic"))]
        {
            // Get all of type.
            let strings: Vec<&&str> = pouch.get_all_of_type();
            assert_eq!(strings.len(), 3);
            assert!(strings.contains(&&"foo"));
            assert!(strings.contains(&&"bar"));
            assert!(strings.contains(&&"baz"));
        }

        // Check the most common type.
        #[cfg(feature = "atomic")]
        assert_eq!(pouch.most_common_type(), Some(TypeId::of::<&str>()));
        #[cfg(not(feature = "atomic"))]
        assert_eq!(pouch.most_common_type(), Some(TypeId::of::<&str>()));

        // Clearing the bag.
        pouch.clear();
        assert!(pouch.is_empty());

        // Add a few items again.
        pouch.insert(5);
        pouch.insert("foo");
        pouch.insert(RPS::Paper);

        #[cfg(not(feature = "atomic"))]
        {
            // Iteration over all elements in the bag
            let mut nums: Vec<i32> = vec![];
            let mut strs: Vec<&str> = vec![];
            let mut moves: Vec<RPS> = vec![];

            pouch.iter().for_each(|item| {
                if let Some(&t) = item.downcast_ref::<i32>() {
                    nums.push(t);
                } else if let Some(&t) = item.downcast_ref::<&str>() {
                    strs.push(t);
                } else if let Some(&t) = item.downcast_ref::<RPS>() {
                    moves.push(t);
                } else {
                    panic!("unexpected type found in bag");
                }
            });
            assert_eq!(nums.first(), Some(&5));
            assert_eq!(strs.first(), Some(&"foo"));
            assert_eq!(moves.first(), Some(&RPS::Paper));
        }
    }
}
