//! 缓存实现
//!
//! 目前的实现暂时是对整个缓冲层加锁，但不会在拿锁和解锁之间读写块设备
//! 执行 `put` 方法的时候，当有数据需要写回的时候，返回 Some<(Key, Value)>，否则返回 None
//!
//! todo: 这里涉及到一个体系结构领域很经典的问题：缓存一致性，后面考虑对这部分实现进行优化，实现高性能的缓存一致性模型

/// 各种缓存替换算法需要实现的 trait
///
/// N: 缓存项的数量
pub trait Cache<const N: usize> {
    type Key;
    type Value;
    /// 根据 `Key` 返回对应的 `Value`
    fn get(&mut self, key: &Self::Key) -> Option<Self::Value>;
    /// 写入一对 `(Key, Value)`
    ///
    /// 如果有需要写回的值，将它返回
    fn put(&mut self, key: &Self::Key, value: Self::Value) -> Option<(Self::Key, Self::Value)>;
    /// 返回所有的缓存项，用于数据同步
    fn all(&mut self) -> Vec<(Self::Key, Self::Value)>;
}

/// [`LFUCache`] 的缓存项
///
/// 除了记录键值对，还记录访问次数，最后访问时间，是否写脏
#[derive(Clone, Copy)]
pub struct Node<K: Eq + PartialEq + Copy, V: Clone> {
    key: K,
    value: V,
    cnt: usize,
    time: usize,
    dirty: bool,
}

impl<K: Eq + PartialEq + Copy, V: Clone> Node<K, V> {
    pub fn new(key: K, value: V) -> Self {
        Self {
            key,
            value,
            cnt: 0,
            time: 0,
            dirty: false,
        }
    }
}

impl<K: Eq + PartialEq + Copy, V: Clone> PartialEq for Node<K, V> {
    fn eq(&self, other: &Self) -> bool {
        self.cnt == other.cnt
    }
}

impl<K: Eq + PartialEq + Copy, V: Clone> Eq for Node<K, V> {}

impl<K: Eq + PartialEq + Copy, V: Clone> Ord for Node<K, V> {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.cnt
            .cmp(&other.cnt)
            .then_with(|| self.time.cmp(&other.time))
    }
}

impl<K: Eq + PartialEq + Copy, V: Clone> PartialOrd for Node<K, V> {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

/// 采用 `LFU` 替换算法的缓存实现
///
/// 这里采用比较保守的 `LFU` 算法实现，将来可能考虑采用性能更好的方案
pub struct LFUCache<K: Eq + PartialEq + Copy, V: Clone, const N: usize> {
    data: [Node<K, V>; N],
    size: usize,
    time: usize,
}

impl<K: Eq + PartialEq + Copy, V: Clone, const N: usize> LFUCache<K, V, N> {
    /// 初始化
    pub fn init(data: [Node<K, V>; N]) -> Self {
        Self {
            data,
            size: N,
            time: 0,
        }
    }
    // todo: 用 `maybuninit`
    pub fn empty(data: [Node<K, V>; N]) -> Self {
        Self {
            data,
            size: 0,
            time: 0,
        }
    }
}

impl<K: Eq + PartialEq + Copy, V: Clone, const N: usize> Cache<N> for LFUCache<K, V, N> {
    type Key = K;
    type Value = V;
    fn get(&mut self, key: &Self::Key) -> Option<Self::Value> {
        self.time += 1;
        let time = self.time;
        self.data[0..self.size]
            .iter_mut()
            .find(|i| i.key == *key)
            .map(|node| {
                // 更新结点时间和访问次数
                node.time = time;
                node.cnt += 1;
                node.value.clone()
            })
    }
    fn put(&mut self, key: &Self::Key, value: Self::Value) -> Option<(Self::Key, Self::Value)> {
        self.time += 1;
        if let Some(node) = self.data.iter_mut().find(|i| i.key == *key) {
            node.value = value;
            node.cnt += 1;
            node.time = self.time;
            // 写脏
            node.dirty = true;
            return None;
        } else {
            if self.size < N {
                // 缓存未满
                self.data[self.size].key = *key;
                self.data[self.size].value = value;
                self.data[self.size].cnt = 1;
                self.data[self.size].time = self.time;
                self.size += 1;
                return None;
            } else {
                // 缓存已满
                // 顺序排序
                self.data[0..self.size].sort_by(|a, b| a.cmp(b));
                // 淘汰第一项
                let node = &mut self.data[0];
                let write_back = (node.key, node.value.clone());
                node.key = *key;
                node.value = value;
                node.cnt = 1;
                node.time = self.time;
                // 如果数据已经被写脏，现在需要写回
                match node.dirty {
                    true => Some(write_back),
                    false => None,
                }
            }
        }
    }
    fn all(&mut self) -> Vec<(Self::Key, Self::Value)> {
        self.data[0..self.size]
            .iter()
            .map(|n| (n.key, n.value.clone()))
            .collect()
    }
}

#[test]
fn lfu_cache_test() {
    let nodes = [Node::new(0, 0); 2];
    let mut lfu_cache = LFUCache::empty(nodes);

    assert!(lfu_cache.get(&0).is_none());
    assert!(lfu_cache.get(&1).is_none());
    assert!(lfu_cache.get(&2).is_none());
    lfu_cache.put(&1, 1);
    lfu_cache.put(&1, 2);
    lfu_cache.put(&2, 2);
    assert_eq!(lfu_cache.get(&1), Some(2));
    lfu_cache.put(&3, 3);
    assert_eq!(lfu_cache.get(&2), None);
    assert_eq!(lfu_cache.get(&3), Some(3));
    assert_eq!(lfu_cache.get(&3), Some(3));
    assert_eq!(lfu_cache.put(&4, 4), Some((1, 2)));
    assert_eq!(lfu_cache.get(&1), None);
    assert_eq!(lfu_cache.get(&3), Some(3));
    assert_eq!(lfu_cache.get(&4), Some(4));
}
