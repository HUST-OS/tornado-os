//! LFU 算法缓存实现
use super::Cache;
use super::Node;
use alloc::vec::Vec;

/// 采用 `LFU` 替换算法的缓存实现
///
/// 这里采用比较保守的 `LFU` 算法实现，将来可能考虑采用性能更好的方案
pub struct LFUCache<K: Eq + PartialEq + Copy, V: Clone, const N: usize> {
    data: [Node<K, V>; N],
    size: usize,
    time: usize,
}

impl<K: Eq + PartialEq + Copy, V: Clone, const N: usize> LFUCache<K, V, N> {
    /// 初始化一个[`LFUCache`]
    ///
    /// # Example
    ///
    /// ```
    /// todo!()
    /// ```
    pub fn init(data: [Node<K, V>; N]) -> Self {
        Self {
            data,
            size: N,
            time: 0,
        }
    }
    /// 创建空的[`LFUCache`]
    ///
    /// # Example
    ///
    /// ```
    /// todo!()
    /// ```
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
