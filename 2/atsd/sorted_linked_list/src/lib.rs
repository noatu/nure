use std::cell::RefCell;
use std::rc::Rc;

pub use List::Nil;
pub use List::Node;

#[derive(PartialEq, Debug)]
pub enum List<T> {
    Nil,
    Node(T, Rc<RefCell<List<T>>>),
}

impl<T> List<T> {
    pub fn new(val: T, next: Self) -> Self {
        Node(val, Rc::new(RefCell::new(next)))
    }
}

impl<T: Clone> List<T> {
    pub fn unref(node: &RefCell<Self>) -> Self {
        if let Node(val, next) = &*node.borrow() {
            return Node(val.clone(), Rc::clone(next));
        }
        Nil
    }

    pub fn unpack(node: &RefCell<Self>) -> Option<(T, Rc<RefCell<Self>>)> {
        if let Node(val, next) = &*node.borrow() {
            return Some((val.clone(), Rc::clone(next)));
        }
        None
    }
}

impl<T: Clone + std::fmt::Display> List<T> {
    pub fn print(&self) {
        print!("[");

        if let Node(val, next) = self {
            print!("{}", val);
            let mut ptr = Rc::clone(next);

            while let Some((val, next)) = List::unpack(&ptr) {
                print!(", {}", val);
                ptr = next;
            }
        }

        println!("]");
    }
}

impl<T: Clone + PartialOrd> List<T> {
    pub fn add(self, new_val: T) -> Self {
        let head = Rc::new(RefCell::new(self));
        let mut ptr = Rc::clone(&head);

        while let Some((val, next)) = List::unpack(&ptr) {
            if val < new_val {
                ptr = next;
                continue;
            }

            *ptr.borrow_mut() = List::new(new_val, Node(val, next));
            return Self::unref(&head);
        }

        if Self::unref(&ptr) == Nil {
            *ptr.borrow_mut() = List::new(new_val, Nil);
        }

        Self::unref(&head)
    }

    pub fn remove(self, rem_val: T, all: bool) -> Self {
        let head = Rc::new(RefCell::new(self));
        let mut ptr = Rc::clone(&head);

        while let Some((val, next)) = List::unpack(&ptr) {
            if val > rem_val {
                break;
            }
            if val != rem_val {
                ptr = next;
                continue;
            }

            *ptr.borrow_mut() = List::unref(&next);
            if !all {
                break;
            }
        }

        Self::unref(&head)
    }

    pub fn pop(&mut self, index: usize) -> Option<T> {
        if let Node(val, next) = self {
            if index == 0 {
                let res = val.clone();
                *self = Self::unref(next);
                return Some(res);
            }

            let mut idx = 1;
            let mut ptr = Rc::clone(next);

            while let Some((val, next)) = List::unpack(&ptr) {
                if idx < index {
                    idx += 1;
                    ptr = next;
                    continue;
                }

                *ptr.borrow_mut() = List::unref(&next);
                return Some(val);
            }
        }

        None
    }

    pub fn get(&self, index: usize) -> Option<T> {
        if let Node(val, next) = self {
            if index == 0 {
                return Some(val.clone());
            }

            let mut idx = 0;
            let mut ptr = Rc::clone(next);

            while let Some((val, next)) = List::unpack(&ptr) {
                idx += 1;
                ptr = next;
                if idx == index {
                    return Some(val);
                }
            }
        }

        None
    }
}

impl<T> List<T>
where
    T: Clone + std::fmt::Display,
{
    pub fn equal(&self, other: &Self) -> bool {
        let mut s1 = "[".to_string();
        if let Node(val, next) = self {
            s1 += &val.to_string();
            let mut ptr = Rc::clone(next);

            while let Some((val, next)) = List::unpack(&ptr) {
                s1 += &format!(", {val}");
                ptr = next;
            }
        }
        s1 += &"]".to_string();

        let mut s2 = "[".to_string();
        if let Node(val, next) = other {
            s2 += &val.to_string();
            let mut ptr = Rc::clone(next);

            while let Some((val, next)) = List::unpack(&ptr) {
                s2 += &format!(", {val}");
                ptr = next;
            }
        }
        s2 += &"]".to_string();

        println!("{s1}");
        println!("{s2}");

        s1 == s2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn equal() {
        let list1 = Nil.add(1).add(3).add(2).add(5).add(42).add(7);
        let list2 = Nil.add(1).add(2).add(3).add(7).add(42).add(5);
        let list3 = Nil.add(3).add(0);

        assert!(list1.equal(&list2));
        println!();
        assert!(!list1.equal(&list3));
        println!();
        assert!(!list1.equal(&Nil));
    }

    #[test]
    fn print() {
        let list = List::new(3, Nil).add(1).add(5).add(2).add(4);
        list.print();
    }

    #[test]
    fn get() {
        let list = List::new(1, List::new(2, List::new(4, Nil)));
        assert_eq!(list.get(2), Some(4));
        assert_eq!(list.get(0), Some(1));
        assert_eq!(list.get(1), Some(2));
        assert_eq!(list.get(10), None);
    }

    #[test]
    fn add() {
        let mut list = List::new(2, Nil);
        let mut desired = List::new(1, List::new(2, List::new(4, Nil)));
        list = list.add(3).add(1).add(2).add(4);
        desired = desired.add(3).add(2);

        assert_eq!(list, desired);
    }

    #[test]
    fn pop() {
        let mut list = Nil.add(2).add(2).add(4).add(3).add(1);
        assert_eq!(list.pop(0), Some(1));
        assert_eq!(list, Nil.add(2).add(2).add(4).add(3));
        assert_eq!(list.pop(2), Some(3));
        assert_eq!(list, Nil.add(2).add(2).add(4));
        assert_eq!(list.pop(100), None);
    }

    #[test]
    fn remove() {
        let mut list = Nil.add(2).add(2).add(4).add(3).add(1);
        let desired = List::new(2, List::new(4, Nil));

        list = list.remove(2, false).remove(3, false).remove(1, false);

        assert_eq!(list, desired);
    }

    #[test]
    fn remove_all() {
        let mut list = Nil.add(2).add(4).add(3).add(1);
        let desired = List::new(4, Nil);

        list = list.remove(2, true).remove(3, true).remove(1, true);

        assert_eq!(list, desired);
    }

    #[test]
    fn custom_type() {
        #[derive(Debug, Clone, PartialEq)]
        struct MyObj {
            id: u8,
            name: String,
        }

        impl MyObj {
            fn new(id: u8, name: &str) -> Self {
                let name = name.into();
                MyObj { id, name }
            }
        }

        impl PartialOrd for MyObj {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                Some(other.id.cmp(&self.id))
            }
        }

        let mut list = List::new(
            MyObj::new(1, "Alice"),
            List::new(MyObj::new(3, "Bob"), List::new(MyObj::new(5, "Jane"), Nil)),
        )
        .add(MyObj::new(2, "Tom"))
        .add(MyObj::new(1, "Mike"))
        .add(MyObj::new(4, "Kelvin"))
        .remove(MyObj::new(5, "Jane"), false)
        .remove(MyObj::new(1, "Mike"), true)
        .remove(MyObj::new(10, "Mike"), true);

        assert_eq!(list.pop(10), None);
        assert_eq!(list.pop(0), Some(MyObj::new(4, "Kelvin")));
    }
}
