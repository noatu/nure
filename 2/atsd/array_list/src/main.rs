#[derive(Default)]
struct ArrayList {
    length: usize,
    allocate: usize,
    capacity: usize,
    data: Box<[i32]>,
}

impl ArrayList {
    const fn len(&self) -> usize {
        self.length
    }

    const fn capacity(&self) -> usize {
        self.capacity
    }

    const fn is_empty(&self) -> bool {
        self.length == 0
    }

    const fn is_full(&self) -> bool {
        self.length == self.capacity
    }
}

impl ArrayList {
    fn alloc(size: usize) -> Box<[i32]> {
        vec![0i32; size].into_boxed_slice()
    }

    fn change_capacity(&mut self, size: usize) {
        self.capacity = size;
        let old_data = std::mem::replace(&mut self.data, Self::alloc(size));
        self.data[..self.length].copy_from_slice(&old_data[..self.length]);
    }

    pub fn new(allocate: usize) -> Self {
        assert!(allocate > 0);

        Self {
            allocate,
            ..Default::default()
        }
    }

    pub fn push(&mut self, num: i32) {
        if self.is_full() {
            self.change_capacity(self.capacity + self.allocate);
        }

        self.data[self.length] = num;
        self.length += 1;
    }

    pub fn pop(&mut self) -> Option<i32> {
        if self.is_empty() {
            return None;
        }

        self.length -= 1;
        Some(self.data[self.length])
    }

    pub fn shrink(&mut self) {
        self.change_capacity(self.length);
    }

    pub fn slice(&self) -> &[i32] {
        &self.data[..self.length]
    }

    fn heapify(&mut self, idx: usize, len: usize, max_heap: bool) {
        if idx >= len {
            return;
        }

        let idxs = [idx, 2 * idx + 1, 2 * idx + 2]
            .into_iter()
            .filter(|&i| i < len);

        let swap_idx = if max_heap {
            idxs.max_by_key(|&i| self.data[i])
        } else {
            idxs.min_by_key(|&i| self.data[i])
        }
        .unwrap();

        if swap_idx != idx {
            self.data.swap(swap_idx, idx);
            self.heapify(swap_idx, len, max_heap);
        }
    }

    fn make_heap(&mut self, max_heap: bool) {
        // parent of last element (last_idx-1)/2 or (len/2)-1
        for idx in (0..self.length / 2).rev() {
            self.heapify(idx, self.length, max_heap);
        }
    }

    pub fn sort(&mut self, ascending: bool) {
        self.make_heap(ascending);

        for len in (1..self.length).rev() {
            self.data.swap(0, len);
            self.heapify(0, len, ascending);
        }
    }

    pub fn delete_top(&mut self) -> Option<i32> {
        if self.is_empty() {
            return None;
        }
        if self.length == 1 {
            return self.pop();
        }

        self.make_heap(true);
        self.length -= 1;
        self.data.swap(0, self.length);
        self.heapify(0, self.length, true);

        Some(self.data[self.length])
    }

    fn tree_internal(&self, idx: usize, lines: Vec<&str>, dir: i8) {
        if 2 * idx + 1 < self.length {
            let mut l = lines.clone();
            if dir == 1 {
                l.pop();
                l.push(" ");
            }
            l.push("│");
            self.tree_internal(2 * idx + 1, l, 1);
        }

        let mut l = lines.clone();
        l.pop();
        l.push(["└ ", "", "┌ "][(dir + 1) as usize]);
        println!("{}{}", l.join(" "), self.data[idx]);

        if 2 * idx + 2 < self.length {
            let mut l = lines;
            if dir == -1 {
                l.pop();
                l.push(" ");
            }
            l.push("│");
            self.tree_internal(2 * idx + 2, l, -1);
        }
    }

    pub fn tree(&self) {
        self.tree_internal(0, vec![], 0);
    }
}

use std::fmt;
impl fmt::Display for ArrayList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", &self.data[..self.length])
    }
}

fn main() {
    let mut arr = ArrayList::new(10);
    for i in 0..32 {
        arr.push(i % 7 * i);
    }

    println!("Data unsorted:   {arr}");
    arr.tree();
    println!();

    arr.sort(true);
    println!("Data ascending:  {arr}");
    arr.tree();
    println!();

    arr.sort(false);
    println!("Data descending: {arr}");
    arr.tree();
    println!();

    let mut arr = ArrayList::new(10);
    for i in [1, 12, 9, 5, 6, 10] {
        arr.push(i);
    }

    println!();
    println!("Data {arr}");
    arr.tree();
    println!("Deleting top: {:?}", arr.delete_top());
    arr.tree();

    println!();
    println!(
        "Deleting top with one element: {:?}",
        ArrayList::new(5).delete_top()
    );
}
