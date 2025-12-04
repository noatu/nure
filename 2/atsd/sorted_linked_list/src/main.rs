use sorted_linked_list::*;

// Write a method ListEqual() to test if the calling List L1 and the argument
// linked lists L2 are equal (i.e., have the same values in the same order).
// Print both L1 and L2.

fn main() {
    let list1 = Nil.add(1).add(3).add(2).add(5).add(42).add(7);
    let list2 = Nil.add(1).add(2).add(3).add(7).add(42).add(5);
    let list3 = List::new(1, List::new(2, List::new(3, Nil)));

    assert!(list1.equal(&list2));
    println!();
    assert!(!list1.equal(&list3));
    println!();
    assert!(!list1.equal(&Nil));
    println!();
    assert!(Nil::<i32>.equal(&Nil));
}
