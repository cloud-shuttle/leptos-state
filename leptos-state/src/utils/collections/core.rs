//! Core collection utilities and algorithms

/// Collection utilities for managing multiple stores/machines
#[derive(Debug)]
pub struct CollectionUtils;

impl CollectionUtils {
    /// Group items by a key function
    pub fn group_by<T, K, F>(items: Vec<T>, key_fn: F) -> std::collections::HashMap<K, Vec<T>>
    where
        K: Eq + std::hash::Hash,
        F: Fn(&T) -> K,
    {
        let mut groups = std::collections::HashMap::new();

        for item in items {
            let key = key_fn(&item);
            groups.entry(key).or_insert_with(Vec::new).push(item);
        }

        groups
    }

    /// Filter items by a predicate
    pub fn filter<T, F>(items: Vec<T>, predicate: F) -> Vec<T>
    where
        F: Fn(&T) -> bool,
    {
        items.into_iter().filter(predicate).collect()
    }

    /// Map items using a function
    pub fn map<T, U, F>(items: Vec<T>, mapper: F) -> Vec<U>
    where
        F: Fn(T) -> U,
    {
        items.into_iter().map(mapper).collect()
    }

    /// Find the first item matching a predicate
    pub fn find<T, F>(items: &[T], predicate: F) -> Option<&T>
    where
        F: Fn(&T) -> bool,
    {
        items.iter().find(|item| predicate(item))
    }

    /// Check if any item matches a predicate
    pub fn any<T, F>(items: &[T], predicate: F) -> bool
    where
        F: Fn(&T) -> bool,
    {
        items.iter().any(predicate)
    }

    /// Check if all items match a predicate
    pub fn all<T, F>(items: &[T], predicate: F) -> bool
    where
        F: Fn(&T) -> bool,
    {
        items.iter().all(predicate)
    }

    /// Count items matching a predicate
    pub fn count<T, F>(items: &[T], predicate: F) -> usize
    where
        F: Fn(&T) -> bool,
    {
        items.iter().filter(|item| predicate(item)).count()
    }

    /// Get unique items using a key function
    pub fn unique_by<T, K, F>(items: Vec<T>, key_fn: F) -> Vec<T>
    where
        K: Eq + std::hash::Hash,
        F: Fn(&T) -> K,
    {
        let mut seen = std::collections::HashSet::new();
        items.into_iter().filter(|item| seen.insert(key_fn(item))).collect()
    }

    /// Partition items into two groups based on a predicate
    pub fn partition<T, F>(items: Vec<T>, predicate: F) -> (Vec<T>, Vec<T>)
    where
        F: Fn(&T) -> bool,
    {
        let mut matching = Vec::new();
        let mut non_matching = Vec::new();

        for item in items {
            if predicate(&item) {
                matching.push(item);
            } else {
                non_matching.push(item);
            }
        }

        (matching, non_matching)
    }

    /// Take the first n items
    pub fn take<T>(items: Vec<T>, n: usize) -> Vec<T> {
        items.into_iter().take(n).collect()
    }

    /// Skip the first n items
    pub fn skip<T>(items: Vec<T>, n: usize) -> Vec<T> {
        items.into_iter().skip(n).collect()
    }

    /// Get items in a range
    pub fn range<T>(items: Vec<T>, start: usize, end: usize) -> Vec<T> {
        items.into_iter().skip(start).take(end - start).collect()
    }

    /// Reverse the order of items
    pub fn reverse<T>(items: Vec<T>) -> Vec<T> {
        let mut reversed = items;
        reversed.reverse();
        reversed
    }

    /// Sort items using a comparison function
    pub fn sort_by<T, F>(mut items: Vec<T>, compare: F) -> Vec<T>
    where
        F: Fn(&T, &T) -> std::cmp::Ordering,
    {
        items.sort_by(compare);
        items
    }

    /// Sort items by a key function
    pub fn sort_by_key<T, K, F>(mut items: Vec<T>, key_fn: F) -> Vec<T>
    where
        K: Ord,
        F: Fn(&T) -> K,
    {
        items.sort_by_key(key_fn);
        items
    }

    /// Get the maximum item by a key function
    pub fn max_by<T, K, F>(items: &[T], key_fn: F) -> Option<&T>
    where
        K: Ord,
        F: Fn(&T) -> K,
    {
        items.iter().max_by_key(key_fn)
    }

    /// Get the minimum item by a key function
    pub fn min_by<T, K, F>(items: &[T], key_fn: F) -> Option<&T>
    where
        K: Ord,
        F: Fn(&T) -> K,
    {
        items.iter().min_by_key(key_fn)
    }

    /// Fold items using an accumulator function
    pub fn fold<T, U, F>(items: Vec<T>, initial: U, accumulator: F) -> U
    where
        F: Fn(U, T) -> U,
    {
        items.into_iter().fold(initial, accumulator)
    }

    /// Reduce items using a reduction function
    pub fn reduce<T, F>(items: Vec<T>, reducer: F) -> Option<T>
    where
        F: Fn(T, T) -> T,
    {
        items.into_iter().reduce(reducer)
    }

    /// Check if collection is empty
    pub fn is_empty<T>(items: &[T]) -> bool {
        items.is_empty()
    }

    /// Get the length of the collection
    pub fn len<T>(items: &[T]) -> usize {
        items.len()
    }

    /// Get the first item
    pub fn first<T>(items: &[T]) -> Option<&T> {
        items.first()
    }

    /// Get the last item
    pub fn last<T>(items: &[T]) -> Option<&T> {
        items.last()
    }

    /// Get item at index
    pub fn nth<T>(items: &[T], index: usize) -> Option<&T> {
        items.get(index)
    }

    /// Create an iterator over the items
    pub fn iter<'a, T>(items: &'a [T]) -> std::slice::Iter<'a, T> {
        items.iter()
    }

    /// Create a mutable iterator over the items
    pub fn iter_mut<'a, T>(items: &'a mut [T]) -> std::slice::IterMut<'a, T> {
        items.iter_mut()
    }

    /// Convert to iterator
    pub fn into_iter<T>(items: Vec<T>) -> std::vec::IntoIter<T> {
        items.into_iter()
    }
}
