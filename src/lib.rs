pub mod fifo;
pub mod lru_1;
//pub mod lru_2q;

#[cfg(test)]
mod tests {
    use crate::lru_1::{LruCache, PutStrategy};

    macro_rules! cache {
        ($cache:expr, { $($k:expr => $v:expr),* }) => {
            $(
                $cache.put($k, $v);
            )*
        };
    }

    #[test]
    fn test_lru1() {
        let mut lru_cache = LruCache::new(2);
        lru_cache.put("1", "a"); // [1, a]
        assert_eq!(lru_cache.get("1"), Some(&"a"));
        lru_cache.put("2", "b"); // [1, a], [2, b]
        assert_eq!(lru_cache.get("2"), Some(&"b"));
        lru_cache.get("1"); // [2, b], [1, a]
        lru_cache.put("3", "c"); // [1, a], [3, c]
        assert_eq!(lru_cache.get("2"), None);
        assert_eq!(lru_cache.get("3"), Some(&"c"));

        lru_cache.put("1", "a"); // [3, c], [1, a]
        lru_cache.put("4", "d"); // [1, a], [4, d]
        assert_eq!(lru_cache.get("3"), None);
    }

    mod test_fifi {
        use crate::fifo::FIFOCache;

        #[test]
        fn test_put_get() {
            let mut fifo_cache = FIFOCache::new(3);
            cache!(fifo_cache, {
                1 => "a",
                2 => "b",
                3 => "c"
            });

            assert_eq!(fifo_cache.get(&1), Some(&"a"));

            fifo_cache.put(4, "d");
            assert_eq!(fifo_cache.get(&1), None);
            *fifo_cache.get_mut(&3).unwrap() = "CC";
            assert_eq!(fifo_cache.get(&3), Some(&"CC"));
        }

        #[test]
        #[should_panic]
        fn test_out_of_capacity() {
            let mut fifo_cache = FIFOCache::new(0);
            fifo_cache.put(1, "a");
        }

        #[test]
        fn test_hits_ratio() {
            let mut fifo_cache = FIFOCache::new(2);
            cache!(fifo_cache, {
                1 => "a",
                2 => "b"
            });
            fifo_cache.get(&1);
            fifo_cache.get(&1);
            fifo_cache.get(&3);
            assert_eq!(fifo_cache.hits_ratio(), 2f64 / 3f64);
            assert_eq!(fifo_cache.hits_ratio_str(), "66.67%".to_owned());
        }

        #[test]
        fn test_renew() {
            let mut fifo_cache = FIFOCache::new(2);
            cache!(fifo_cache, {
                1 => "a",
                2 => "b"
            });
            assert_eq!(fifo_cache.front(), Some(&"a"));
            fifo_cache.renew(&1);
            assert_eq!(fifo_cache.back(), Some(&"a"));
        }

        #[test]
        fn test_take() {
            let mut fifo_cache = FIFOCache::new(3);
            cache!(fifo_cache, {
                1 => "a",
                2 => "b"
            });
            assert_eq!(fifo_cache.take(&1), Some((1, "a")));
            assert_eq!(fifo_cache.len(), 1);
        }

        #[test]
        fn test_other() {
            let mut fifo_cache = FIFOCache::new(3);
            cache!(fifo_cache, {
                1 => "a",
                2 => "b"
            });
            assert!(!fifo_cache.is_empty());
            assert_eq!(fifo_cache.len(), 2);

            fifo_cache.clear();

            assert!(fifo_cache.is_empty());
            assert_eq!(fifo_cache.len(), 0);
            assert_eq!(fifo_cache.capacity(), 3);
        }
    }
}
