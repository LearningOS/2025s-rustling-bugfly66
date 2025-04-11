/*
    heap
    This question requires you to implement a binary heap function
*/


use std::cmp::Ord;
use std::default::Default;

fn min_compare<T: Ord>(a: &T, b: &T) -> bool {
    a < b
}

fn max_compare<T: Ord>(a: &T, b: &T) -> bool {
    a > b
}

pub struct Heap<T>
where
    T: Default,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default + Ord,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![T::default()],
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add(&mut self, value: T) {
        self.count += 1;
        self.items.push(value);
        let mut cur = self.count;

        while cur > 0 {
            let parent = self.parent_idx(cur);
            if parent == 0 {
                break;
            }
            if (self.is_min_heap() && self.items[cur] < self.items[parent])
                || (self.is_max_heap() && self.items[cur] > self.items[parent])
            {
                self.items.swap(cur, parent);
                cur = parent;
            } else {
                break;
            }
        }
    }

    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    fn largest_child_idx(&self, idx: usize) -> usize {
        let left = self.left_child_idx(idx);
        let right = self.right_child_idx(idx);
        if right <= self.count && self.items[right] > self.items[left] {
            right
        } else {
            left
        }
    }

    fn smallest_child_idx(&self, idx: usize) -> usize {
        let left = self.left_child_idx(idx);
        let right = self.right_child_idx(idx);
        if right <= self.count && self.items[right] < self.items[left] {
            right
        } else {
            left
        }
    }

    pub fn is_min_heap(&self) -> bool {
        (self.comparator as usize) == (min_compare::<T> as usize)
    }

    pub fn is_max_heap(&self) -> bool {
        (self.comparator as usize) == (max_compare::<T> as usize)
    }
}

impl<T> Heap<T>
where
    T: Default + Ord,
{
    pub fn new_min() -> Self {
        Self::new(min_compare)
    }

    pub fn new_max() -> Self {
        Self::new(max_compare)
    }
}

impl<T> Iterator for Heap<T>
where
    T: Default + Ord,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.count == 0 {
            return None;
        }
        self.items.swap(1, self.count);
        self.count -= 1;

        let mut cur = 1;
        while self.children_present(cur) {
            let child = if self.is_min_heap() {
                self.smallest_child_idx(cur)
            } else {
                self.largest_child_idx(cur)
            };

            if (self.is_min_heap() && self.items[child] < self.items[cur])
                || (self.is_max_heap() && self.items[child] > self.items[cur])
            {
                self.items.swap(child, cur);
                cur = child;
            } else {
                break;
            }
        }

        self.items.pop()
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new_min()
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new_max()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);

        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }
}
