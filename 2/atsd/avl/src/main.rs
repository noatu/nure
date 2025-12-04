// #![allow(unused)]
use std::cmp::Ordering;

#[derive(Clone, Default)]
struct Node {
    value: i32,
    height: usize,
    left: Option<Box<Self>>,
    right: Option<Box<Self>>,
}

// internal
impl Node {
    fn height(node: &Option<Box<Self>>) -> usize {
        match node {
            Some(node) => 1 + node.height,
            None => 0,
        }
    }

    fn update_height(&mut self) {
        self.height = std::cmp::max(Node::height(&self.left), Node::height(&self.right));
    }

    fn balance_factor(&self) -> isize {
        Self::height(&self.left) as isize - Self::height(&self.right) as isize
    }

    fn right_rotate(&mut self) {
        let mut new_root = *self.left.take().unwrap();
        self.left = new_root.right.take();
        self.update_height();
        self.right = Some(Box::new(std::mem::replace(self, new_root)));
        self.update_height();
    }

    fn left_rotate(&mut self) {
        let mut new_root = *self.right.take().unwrap();
        self.right = new_root.left.take();
        self.update_height();
        self.left = Some(Box::new(std::mem::replace(self, new_root)));
        self.update_height();
    }

    fn rebalance(&mut self) {
        let balance_factor = self.balance_factor();

        if balance_factor > 1 {
            let node = self.left.as_mut().unwrap();
            if node.balance_factor() < 0 {
                // skewed to the right
                node.left_rotate();
            }
            self.right_rotate();
        }

        if balance_factor < -1 {
            let node = self.right.as_mut().unwrap();
            if node.balance_factor() > 0 {
                // skewed to the left
                node.right_rotate();
            }
            self.left_rotate();
        }
    }

    fn tree_internal(&self, lines: Vec<&str>, dir: i8) {
        if let Some(ref node) = self.right {
            let mut l = lines.clone();
            if dir == 1 {
                l.pop();
                l.push(" ");
            }
            l.push("│");
            node.tree_internal(l, 1);
        }

        let mut l = lines.clone();
        l.pop();
        l.push(["└ ", "", "┌ "][(dir + 1) as usize]);
        let dbg_info = ""; // format!(" ({})", self.height);
        println!("{}{}{}", l.join(" "), self.value, dbg_info);

        if let Some(ref node) = self.left {
            let mut l = lines;
            if dir == -1 {
                l.pop();
                l.push(" ");
            }
            l.push("│");
            node.tree_internal(l, -1);
        }
    }
}

// public
impl Node {
    pub fn new(value: i32) -> Self {
        Self {
            value,
            ..Default::default()
        }
    }

    pub fn from_vec(values: Vec<i32>) -> Self {
        let mut node = Self::new(values[0]);
        for value in values.into_iter().skip(1) {
            node.add(value);
        }
        node
    }

    pub fn add(&mut self, val: i32) {
        match (val.cmp(&self.value), &mut self.left, &mut self.right) {
            (Ordering::Less, opt, _) | (Ordering::Greater, _, opt) => match opt {
                Some(node) => node.add(val),
                None => *opt = Some(Box::new(Self::new(val))),
            },
            _ => return,
        };

        self.update_height();
        self.rebalance();
    }

    pub fn del(mut self, val: i32) -> Option<Box<Self>> {
        match (val.cmp(&self.value), &mut self.left, &mut self.right) {
            (Ordering::Less, opt, _) | (Ordering::Greater, _, opt) => {
                if opt.is_some() {
                    *opt = opt.take().unwrap().del(val);
                }
            }
            (Ordering::Equal, _, Some(node)) => {
                self.value = node.min();
                self.right = self.right.take().unwrap().del(self.value);
            }

            (Ordering::Equal, Some(_), None) => return self.left,
            (Ordering::Equal, None, None) => return None,
        };

        self.update_height();
        self.rebalance();
        Some(Box::new(self))
    }

    pub fn has(&self, val: i32) -> bool {
        match (val.cmp(&self.value), &self.left, &self.right) {
            (Ordering::Less, Some(n), _) | (Ordering::Greater, _, Some(n)) => n.has(val),
            (Ordering::Equal, ..) => true,
            _ => false,
        }
    }

    pub fn min(&self) -> i32 {
        match self.left {
            Some(ref node) => node.min(),
            None => self.value,
        }
    }

    pub fn max(&self) -> i32 {
        match self.right {
            Some(ref node) => node.max(),
            None => self.value,
        }
    }

    pub fn tree(&self) {
        self.tree_internal(vec![], 0);
    }

    pub fn clone_node(&self) -> Self {
        Self {
            value: self.value,
            height: self.height,
            left: self.left.as_ref().map(|node| Box::new(node.clone_node())),
            right: self.right.as_ref().map(|node| Box::new(node.clone_node())),
        }
    }
}

use std::fmt;
impl fmt::Display for Node {
    // inorder
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(ref node) = self.left {
            write!(f, "{node}")?;
        }
        write!(f, "{} ", self.value)?;
        if let Some(ref node) = self.right {
            write!(f, "{node}")?;
        }
        Ok(())
    }
}

impl FromIterator<i32> for Node {
    fn from_iter<I: IntoIterator<Item = i32>>(iter: I) -> Self {
        let mut iter = iter.into_iter();
        let mut node = Node::new(iter.next().unwrap());
        for value in iter {
            node.add(value);
        }
        node
    }
}

fn main() {
    let mut avl = Node::from_vec(vec![20, 4]);
    avl.add(15);
    avl.tree();
    println!();
    let mut avl = Node::from_vec(vec![20, 4, 26, 3, 9]);
    avl.add(15);
    avl.tree();
    println!();
    let mut avl = Node::from_vec(vec![20, 4, 26, 3, 9, 21, 30, 2, 7, 11]);
    avl.add(15);
    avl.tree();
    println!();

    println!();
    let mut avl = Node::from_vec(vec![20, 4]);
    avl.add(8);
    avl.tree();
    println!();
    let mut avl = Node::from_vec(vec![20, 4, 26, 3, 9]);
    avl.add(8);
    avl.tree();
    println!();
    let mut avl = Node::from_vec(vec![20, 4, 26, 3, 9, 21, 30, 2, 7, 11]);
    avl.add(8);
    avl.tree();
    println!();

    println!();
    let avl = Node::from_vec(vec![2, 1, 4, 3, 5]);
    let avl = avl.del(1).unwrap();
    avl.tree();
    println!();
    let avl = Node::from_vec(vec![6, 2, 9, 1, 4, 8, 11, 3, 5, 7, 10, 12, 13]);
    let avl = avl.del(1).unwrap();
    avl.tree();
    println!();

    let avl = Node::from_vec(vec![5, 2, 8, 1, 3, 7, 10, 4, 6, 11, 12]);
    let avl = avl.del(1).unwrap();
    avl.tree();
    println!("{}", avl.has(1));
    println!("{}", avl.has(7));
    println!("{}", avl.min());
    println!("{}", avl.max());

    let mut avl2 = avl.clone();
    avl2.add(42);
    avl2.tree();
    println!();
    println!("{avl}");
    println!("{avl2}");

    // Node::from_iter(-1000..=1000).tree();
}
