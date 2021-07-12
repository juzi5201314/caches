use std::borrow::Borrow;
use std::hash::Hash;

use crate::fifo::FIFOCache;
use crate::lru_1::LruCache;

pub struct Lru2qCache<K, V> {
    capacity: usize,
    lru: LruCache<K, V>,
    fifo: FIFOCache<K, V>,
}

impl<K, V> Lru2qCache<K, V>
where
    K: Hash + Eq,
{
    pub fn new(capacity: usize) -> Self {
        Lru2qCache {
            capacity,
            lru: LruCache::new(capacity),
            fifo: FIFOCache::new(capacity),
        }
    }

    #[inline]
    pub fn get<Q: ?Sized>(&mut self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.fifo
            .take(key)
            .map(move |(k, v)| self.lru.put_and_return(k, v).map(|(_, v)| v))
            .flatten()
    }

    #[inline]
    pub fn get_or_init<F>(&mut self, key: K, init: F) -> Option<&V>
    where
        F: FnOnce() -> V,
    {
        if self.fifo.contains(&key) || self.lru.contains(&key) {
            self.get(&key)
        } else {
            self.put_and_return(key, init()).map(|(_, v)| v)
        }
    }

    #[inline]
    pub fn get_mut<Q: ?Sized>(&mut self, key: &Q) -> Option<&mut V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        todo!()
    }

    /// Take an element out of the cache
    #[inline]
    pub fn take<Q: ?Sized>(&mut self, key: &Q) -> Option<(K, V)>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        todo!()
    }

    /// Put an element
    /// If an element is popped, return it
    ///
    /// strategy: [PutStrategy]
    #[inline]
    pub fn put(&mut self, key: K, value: V) -> Option<(K, V)> {
        self.fifo.put(key, value)
    }

    #[inline]
    pub fn put_and_return(&mut self, key: K, value: V) -> Option<(&K, &V)> {
        self.fifo.put_and_return(key, value)
    }

    /// Get the actual size of the cache
    #[inline]
    pub fn len(&self) -> usize {
        self.fifo.len() + self.lru.len()
    }

    /// Get the capacity of the cache
    #[inline]
    pub fn capacity(&self) -> usize {
        debug_assert!(
            self.capacity == self.fifo.capacity() && self.capacity == self.lru.capacity()
        );
        self.capacity
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.fifo.is_empty() && self.lru.is_empty()
    }

    #[inline]
    pub fn clear(&mut self) {
        self.fifo.clear();
        self.lru.clear();
    }

    #[inline]
    pub fn front(&self) -> Option<&V> {
        self.lru.front()
    }

    #[inline]
    pub fn back(&self) -> Option<&V> {
        self.fifo.back()
    }

    #[inline]
    pub fn pos(&self, pos: usize) -> Option<&V> {
        todo!()
    }
}
