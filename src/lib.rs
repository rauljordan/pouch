//! [`Purse`](Purse) implements a bag data structure, also known as a multiset. A bag
//! can contain anything, with possible duplicates of each item, and items of
//! different types - it's a free for all!
//!
//! [`Purse`](Purse) is helpful in a wide range of applications where a heterogeneous
//! and non-unique collection of items is required.
//!
//!
//! # Examples
//!
//! Basic usage:
//!
//! ```
//! use std::any::TypeId;
//! use purse::Purse;
//! let mut purse = Purse::new();
//!
//! purse.insert("hello");
//! purse.insert(42);
//! assert!(purse.contains("hello"));
//! assert!(purse.contains(42));
//! assert_eq!(purse.count::<&str>(), 1);
//!
//! purse.insert("foo");
//! purse.insert("bar");
//! purse.insert("baz");
//!
//! // Check the most common type.
//! assert_eq!(purse.most_common_type(), Some(TypeId::of::<&str>()));
//!
//! // Clearing the bag.
//! purse.clear();
//! assert!(purse.is_empty());
//!
//! // Add a few items again.
//! purse.insert(5);
//! purse.insert("foo");
//!
//! // Iteration over all elements in the bag
//! let mut nums: Vec<i32> = vec![];
//! let mut strs: Vec<&str> = vec![];
//!
//! purse.iter().for_each(|item| {
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

use std::collections::HashMap;

trait ClonableAny: Any + Clone + Sized {}

#[derive(Default, Debug)]
pub struct Purse {
    data: HashMap<TypeId, Vec<Box<dyn Any>>>,
    counts: HashMap<TypeId, u64>,
}

impl Purse {
    pub fn new() -> Self {
        Self {
            data: HashMap::default(),
            counts: HashMap::default(),
        }
    }
    /// Checks if the purse is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// # use purse::Purse;
    /// let mut purse = Purse::new();
    /// assert!(purse.is_empty());
    /// purse.insert("apple");
    /// assert!(!purse.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    /// Provides an iterator over all elements in the purse.
    ///
    /// This method returns a boxed iterator that yields references to each element stored in the purse,
    /// regardless of their type. It iterates over all elements in a non-specific order.
    ///
    /// # Returns
    /// A `Box<dyn Iterator<Item = &dyn Any> + '_>` that can be used to iterate over all elements
    /// in the purse.
    ///
    /// # Examples
    ///
    /// ```
    /// # use purse::Purse;
    /// # let mut purse = Purse::new();
    /// # purse.insert(42);
    /// # purse.insert("hello");
    /// for item in purse.iter() {
    ///     // Process each item
    /// }
    /// ```
    pub fn iter(&self) -> Box<dyn Iterator<Item = &dyn Any> + '_> {
        Box::new(self.data.values().flatten().map(|b| &**b))
    }
    /// Retrieves all elements of a specific type from the purse.
    ///
    /// This method returns a vector containing references to all elements of the type specified
    /// by the generic parameter `T`. It filters the elements in the purse based on their type.
    ///
    /// # Returns
    /// A `Vec<&T>` containing references to all elements of type `T` in the purse.
    ///
    /// # Examples
    ///
    /// ```
    /// # use purse::Purse;
    /// # let mut purse = Purse::new();
    /// # purse.insert(42);
    /// # purse.insert(42);
    /// let numbers: Vec<&i32> = purse.get_all_of_type();
    /// assert_eq!(numbers.len(), 2);
    /// assert_eq!(*numbers[0], 42);
    /// assert_eq!(*numbers[1], 42);
    /// ```
    pub fn get_all_of_type<T: Any>(&self) -> Vec<&T> {
        let type_id = TypeId::of::<T>();
        self.data.get(&type_id).map_or(Vec::new(), |elems| {
            elems
                .iter()
                .filter_map(|el| el.downcast_ref::<T>())
                .collect()
        })
    }
    /// Checks if the purse contains a given element.
    ///
    /// This method searches the purse for an element equal to `t` and returns `true` if it is found.
    ///
    /// # Type Parameters
    /// - `T`: The type of the element to check. This type must implement `Any` and `Eq`.
    ///
    /// # Arguments
    /// - `t`: The element to check for in the purse.
    ///
    /// # Returns
    /// Returns `true` if the element is found in the purse, otherwise `false`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use purse::Purse;
    /// let mut purse = Purse::new();
    /// purse.insert("apple");
    /// assert!(purse.contains("apple"));
    /// ```
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
    /// Retrieves a list of `TypeId`s of the types currently stored in the purse.
    ///
    /// This method returns a vector containing the `TypeId` of each unique type currently stored in the purse.
    /// It iterates over a `HashMap` to collect the type identifiers.
    ///
    /// # Returns
    /// A `Vec<TypeId>` containing the unique type identifiers of all elements stored in the purse.
    ///
    /// # Examples
    ///
    /// ```
    /// # use purse::Purse;
    /// use std::any::{Any, TypeId};
    /// let mut purse = Purse::new();
    /// purse.insert(42u64);
    /// purse.insert("apple");
    /// let types = purse.types();
    /// assert!(types.contains(&TypeId::of::<&str>()));
    /// ```
    pub fn types(&self) -> Vec<TypeId> {
        self.data.keys().cloned().collect()
    }
    /// Counts the number of elements of a specific type in the purse.
    ///
    /// This method returns the number of elements of the type specified by the generic parameter `T`.
    /// It accesses a `HashMap` that tracks counts of each type to retrieve the count.
    ///
    /// # Type Parameters
    /// - `T`: The type for which to count the occurrences. This type must implement the `Any` trait.
    ///
    /// # Examples
    ///
    /// ```
    /// # use purse::Purse;
    /// # use std::any::TypeId;
    /// let mut purse = Purse::new();
    /// purse.insert(42);
    /// purse.insert(42);
    /// purse.insert("hello");
    /// assert_eq!(purse.count::<i32>(), 2);
    /// ```
    pub fn count<T: Any>(&self) -> u64 {
        let type_id = TypeId::of::<T>();
        *self.counts.get(&type_id).unwrap_or(&0)
    }
    /// Determines the most common type stored in the purse.
    ///
    /// This method returns the `TypeId` of the most frequently occurring type.
    /// It checks the `HashMap` of counts to find the most common type.
    ///
    /// # Examples
    ///
    /// ```
    /// # use purse::Purse;
    /// use std::any::TypeId;
    /// let mut purse = Purse::new();
    /// purse.insert(42);
    /// purse.insert("hello");
    /// purse.insert("world");
    /// assert_eq!(purse.most_common_type(), Some(TypeId::of::<&str>()));
    /// ```
    pub fn most_common_type(&self) -> Option<TypeId> {
        self.counts.keys().max().copied()
    }
    /// Inserts an element into the purse.
    ///
    /// # Examples
    /// ```
    /// # use purse::Purse;
    /// let mut purse = Purse::new();
    /// purse.insert(42);
    /// assert!(purse.contains(42));
    /// ```
    pub fn insert<T: Any>(&mut self, elem: T) {
        let type_id = TypeId::of::<T>();
        self.data.entry(type_id).or_default().push(Box::new(elem));

        *self.counts.entry(type_id).or_insert(0) += 1;
    }
    /// Removes a single occurrence of an element from the purse, if present.
    ///
    /// This method looks for an element equal to `elem` and removes the first occurrence it finds.
    ///
    /// # Type Parameters
    /// - `T`: The type of the element to remove. This type must implement `Any` and `Eq`.
    ///
    /// # Arguments
    /// - `elem`: The element to remove from the purse.
    ///
    /// # Returns
    /// Returns `true` if an element was removed, or `false` if no such element was found.
    ///
    /// # Examples
    ///
    /// ```
    /// # use purse::Purse;
    /// let mut purse = Purse::new();
    /// purse.insert("apple");
    /// assert!(purse.remove("apple"));
    /// assert!(!purse.contains(&"apple"));
    /// ```
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
    /// Clears all elements from the purse.
    ///
    /// This method removes all elements from the purse, effectively resetting it to its initial state.
    /// It clears both the `HashMap` storing the elements and the `HashMap` tracking the counts, requiring mutable access to the purse.
    ///
    /// # Examples
    ///
    /// ```
    /// # use purse::Purse;
    /// let mut purse = Purse::new();
    /// purse.insert(42);
    /// purse.clear();
    /// assert!(purse.is_empty());
    /// ```
    pub fn clear(&mut self) {
        self.data.clear();
        self.counts.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_purse() {
        #[derive(PartialEq, Eq, Clone, Copy, Debug)]
        enum RPS {
            Rock,
            Paper,
            Scissors,
        }
        let mut purse = Purse::new();

        purse.insert(5);
        purse.insert("foo");
        purse.insert(RPS::Rock);
        purse.insert(RPS::Paper);

        // Check if items are contained.
        assert!(purse.contains("foo"));
        assert!(!purse.contains("bar"));
        assert!(!purse.contains(0));
        assert!(purse.contains(RPS::Rock));
        assert!(!purse.contains(RPS::Scissors));

        // Check removal of items.
        purse.remove(RPS::Rock);
        assert!(!purse.contains(RPS::Rock));

        // Check counts.
        assert_eq!(purse.count::<&str>(), 1);
        purse.insert("bar");
        purse.insert("baz");
        assert_eq!(purse.count::<&str>(), 3);

        // Check types.
        let types = purse.types();
        assert_eq!(types.len(), 3);
        assert!(types.contains(&TypeId::of::<i32>()));
        assert!(types.contains(&TypeId::of::<&str>()));
        assert!(types.contains(&TypeId::of::<RPS>()));

        // Get all of type.
        let strings: Vec<&&str> = purse.get_all_of_type();
        assert_eq!(strings.len(), 3);
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
        purse.insert(RPS::Paper);

        // Iteration over all elements in the bag
        let mut nums: Vec<i32> = vec![];
        let mut strs: Vec<&str> = vec![];
        let mut moves: Vec<RPS> = vec![];

        purse.iter().for_each(|item| {
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
