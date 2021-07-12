use std::borrow::Borrow;
use std::hash::Hash;

use linked_hash_map_rs::LinkedHashMap;
use std::ops::Deref;

pub struct LruCache<K, V> {
    capacity: usize,
    map: LinkedHashMap<K, V>,
    put_strategy: PutStrategy,
    popped_count: usize,
}

pub enum PutStrategy {
    /// Replace the value without moving it to the end of the queue
    Replace,
    /// Pop the element from its original place and push it to the end of the queue
    Add,
    /// Move the element to the back of the queue without changing it. just like [get]
    Move,
}

impl Default for PutStrategy {
    fn default() -> Self {
        PutStrategy::Add
    }
}

impl<K, V> LruCache<K, V>
where
    K: Hash + Eq,
{
    pub fn new(capacity: usize) -> Self {
        LruCache {
            capacity,
            map: LinkedHashMap::with_capacity(capacity),
            ..Default::default()
        }
    }

    pub fn with_put_strategy(capacity: usize, strategy: PutStrategy) -> Self {
        LruCache {
            capacity,
            map: LinkedHashMap::with_capacity(capacity),
            put_strategy: strategy,
            ..Default::default()
        }
    }

    pub fn put_strategy(&mut self, strategy: PutStrategy) {
        self.put_strategy = strategy;
    }

    #[inline]
    pub fn get<Q: ?Sized>(&mut self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.map.move_to_back(key).map(|(_, v)| v)
    }

    #[inline]
    pub fn get_mut<Q: ?Sized>(&mut self, key: &Q) -> Option<&mut V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        if self.map.move_to_back(key).is_some() {
            self.map.get_mut(key)
        } else {
            None
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

    /// Put an element
    /// If an element is popped, return it
    ///
    /// strategy: [PutStrategy]
    #[inline]
    pub fn put(&mut self, key: K, value: V) -> Option<(K, V)> {
        let mut res = None;
        if self.map.contains(&key) {
            match self.put_strategy {
                PutStrategy::Add => {
                    self.map.remove(&key);
                    self.map.push_back(key, value);
                },
                PutStrategy::Move => {
                    self.map.move_to_back(&key);
                },
                PutStrategy::Replace => {
                    self.map.push_back(key, value);
                }
            }
        } else {
            if self.len() >= self.capacity() {
                res = self.map.pop_front();
                self.popped_count += 1;
            };
            self.map.push_back(key, value);
        }
        debug_assert!(self.len() <= self.capacity());
        res
    }

    /// Put an element
    /// and return the put element.
    ///
    /// strategy: [PutStrategy]
    #[inline]
    pub fn put_and_return(&mut self, key: K, value: V) -> Option<(&K, &V)> {
        debug_assert!(self.len() <= self.capacity());
        let res = if self.map.contains(&key) {
            match self.put_strategy {
                PutStrategy::Add => {
                    self.map.remove(&key);
                    Some(self.map.push_back_and_return(key, value))
                },
                PutStrategy::Move => {
                    self.map.move_to_back(&key)
                },
                PutStrategy::Replace => {
                    Some(self.map.push_back_and_return(key, value))
                }
            }
        } else {
            if self.len() >= self.capacity() {
                self.map.pop_front();
                self.popped_count += 1;
            };
            self.map.push_back(key, value);
            None
        };
        res
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

    /// Get the number of elements that have been popped
    #[inline]
    pub fn popped_count(&self) -> usize {
        self.popped_count
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


impl<K, V> Default for LruCache<K, V> {
    fn default() -> Self {
        LruCache {
            capacity: 0,
            map: Default::default(),
            put_strategy: Default::default(),
            popped_count: 0
        }
    }
}

impl<K, V> Deref for LruCache<K, V> {
    type Target = LinkedHashMap<K, V>;

    fn deref(&self) -> &Self::Target {
        &self.map
    }
}
