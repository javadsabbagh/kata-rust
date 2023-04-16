# priority queue

## Introduction
Ordinary queue is a first-in first-out data structure, elements are appended at the end of the queue and deleted from the head of the queue. In a priority queue, elements are given priority. When elements are accessed, the element with the highest priority is removed first. The priority queue has the highest-level first-out (largest-in, first-out) behavioral characteristics.

>The priority queue is a collection of 0 or more elements, each element has a priority or value, and the operations performed on the priority queue are:

1. Find;
2. Insert a new element;
3. Delete.

In the minimum priority queue (min priority queue), the search operation is used to search for the element with the lowest priority, and the delete operation is used to delete the element; for the maximum priority queue (max priority queue), the search operation is used to search for the element with the highest priority , the delete operation is used to delete the element. Elements in the priority queue can have the same priority, and search and delete operations can be performed according to any priority.

## Implementation of priority queue:

First define the PriorityQueue structure

```rust
#[derive(Debug)]
struct PriorityQueue<T> where T: PartialOrd + Clone {
    pq: Vec<T>
}
```

`where T: PartialOrd + Clone` in the second line means that the generic T stored in PriorityQueue satisfies the `PartialOrd` and `Clone` trait constraints, which means that the generic T is sortable and cloneable.

The following are some basic method implementations, which are relatively simple, just look at the code directly. This priority queue is implemented based on Vec, with O(1) insertion and O(n) max/min dequeue.

```rust
impl<T> PriorityQueue<T> where T: PartialOrd + Clone {
    fn new() -> PriorityQueue<T> {
        PriorityQueue { pq: Vec::new() }
    }

    fn len(&self) -> usize {
        self.pq.len()
    }

    fn is_empty(&self) -> bool {
        self.pq.len() == 0
    }

    fn insert(&mut self, value: T) {
        self.pq.push(value);
    }

    fn max(&self) -> Option<T> {
        if self.is_empty() { return None }
        let max = self.max_index();
        Some(self.pq[max].clone())
    }

    fn min(&self) -> Option<T> {
        if self.is_empty() { return None }
        let min = self.min_index();
        Some(self.pq[min].clone())
    }

    fn delete_max(&mut self) -> Option<T> {
        if self.is_empty() { return None; }
        let max = self.max_index();
        Some(self.pq.remove(max).clone())
    }

    fn delete_min(&mut self) -> Option<T> {
        if self.is_empty() { return None; }
        let min = self.min_index();
        Some(self.pq.remove(min).clone())
    }

    fn max_index(&self) -> usize {
        let mut max = 0;
        for i in 1..self.pq.len() - 1 {
            if self.pq[max] < self.pq[i] {
                max = i;
            }
        }
        max
    }

    fn min_index(&self) -> usize {
        let mut min = 0;
        for i in 0..self.pq.len() - 1 {
            if self.pq[i] < self.pq[i + 1] {
                min = i;
            }
        }
        min
    }
}
```

## Test code:

```rust
fn test_keep_min() {
    let mut pq = PriorityQueue::new();
    pq.insert(3);
    pq.insert(2);
    pq.insert(1);
    pq.insert(4);
    assert!(pq.min().unwrap() == 1);
}

fn test_keep_max() {
    let mut pq = PriorityQueue::new();
    pq.insert(2);
    pq.insert(4);
    pq.insert(1);
    pq.insert(3);
    assert!(pq.max().unwrap() == 4);
}

fn test_is_empty() {
    let mut pq = PriorityQueue::new();
    assert!(pq.is_empty());
    pq.insert(1);
    assert!(!pq.is_empty());
}

fn test_len() {
    let mut pq = PriorityQueue::new();
    assert!(pq.len() == 0);
    pq.insert(2);
    pq.insert(4);
    pq.insert(1);
    assert!(pq.len() == 3);
}

fn test_delete_min() {
    let mut pq = PriorityQueue::new();
    pq.insert(3);
    pq.insert(2);
    pq.insert(1);
    pq.insert(4);
    assert!(pq.len() == 4);
    assert!(pq.delete_min().unwrap() == 1);
    assert!(pq.len() == 3);
}

fn test_delete_max() {
    let mut pq = PriorityQueue::new();
    pq.insert(2);
    pq.insert(10);
    pq.insert(1);
    pq.insert(6);
    pq.insert(3);
    assert!(pq.len() == 5);
    assert!(pq.delete_max().unwrap() == 10);
    assert!(pq.len() == 4);
}

fn main() {
    test_len();
    test_delete_max();
    test_delete_min();
    test_is_empty();
    test_keep_max();
    test_keep_min();
}
```

## practise
Implement a priority queue based on a binary heap to achieve O(1) dequeue and O(log n) enqueue
