use std::borrow::Borrow;
use std::hash::Hash;

use linked_hash_map_rs::LinkedHashMap;

pub struct FIFOCache<K, V> {
    capacity: usize,
    map: LinkedHashMap<K, V>,

    hits: u64,
    misses: u64,
}

impl<K, V> FIFOCache<K, V>
where
    K: Hash + Eq,
{
    pub fn new(capacity: usize) -> Self {
        FIFOCache {
            capacity,
            map: LinkedHashMap::with_capacity(capacity),
            ..Default::default()
        }
    }

    /// Take an element out of the cache
    #[inline]
    pub fn take<Q: ?Sized>(&mut self, key: &Q) -> Option<(K, V)>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.map.remove(key)
    }

    #[inline]
    pub fn get<Q: ?Sized>(&mut self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        if let Some(value) = self.map.get(key) {
            self.hits += 1;
            Some(value)
        } else {
            self.misses += 1;
            None
        }
    }

    #[inline]
    pub fn get_mut<Q: ?Sized>(&mut self, key: &Q) -> Option<&mut V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        if let Some(value) = self.map.get_mut(key) {
            self.hits += 1;
            Some(value)
        } else {
            self.misses += 1;
            None
        }
    }

    /// Move the element to the back of the queue and return it
    #[inline]
    pub fn renew<Q: ?Sized>(&mut self, key: &Q) -> Option<(&K, &V)>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.map.move_to_back(&key)
    }

    #[inline]
    fn check_size(&mut self) -> Option<(K, V)> {
        debug_assert!(self.len() <= self.capacity(), "out of capacity");
        if self.len() >= self.capacity() {
            self.map.pop_front().or_else(|| panic!("out of capacity"))
        } else {
            None
        }
    }

    /// Put an element
    /// If an element is popped, return it
    #[inline]
    pub fn put(&mut self, key: K, value: V) -> Option<(K, V)> {
        let res = self.check_size();
        self.map.push_back(key, value);
        res
    }

    /// Put an element,
    /// and return it
    #[inline]
    pub fn put_and_return(&mut self, key: K, value: V) -> (&K, &V) {
        self.check_size();
        self.map.push_back_and_return(key, value)
    }

    /// Get the actual size of the cache
    #[inline]
    pub fn len(&self) -> usize {
        self.map.len()
    }

    /// Get the capacity of the cache
    #[inline]
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    #[inline]
    pub fn hits_ratio(&self) -> f64 {
        self.hits as f64 / (self.hits + self.misses) as f64
    }

    #[inline]
    pub fn hits_ratio_str(&self) -> String {
        format!("{:.2}%", self.hits_ratio() * 100f64)
    }

    #[inline]
    pub fn contains<Q: ?Sized>(&self, key: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.map.contains(key)
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    #[inline]
    pub fn clear(&mut self) {
        self.map.clear()
    }

    #[inline]
    pub fn front(&self) -> Option<&V> {
        self.map.front().map(|(_, v)| v)
    }

    #[inline]
    pub fn back(&self) -> Option<&V> {
        self.map.back().map(|(_, v)| v)
    }

    #[inline]
    pub fn pos(&self, pos: usize) -> Option<&V> {
        self.map.position(pos).map(|(_, v)| v)
    }
}

impl<K, V> Default for FIFOCache<K, V> {
    fn default() -> Self {
        FIFOCache {
            capacity: 0,
            map: Default::default(),

            hits: 0,
            misses: 0,
        }
    }
}
