# AVL Tree

Implements an AVL Tree, which is a self-balancing Binary Search Tree (BST) that ensures logarithmic time complexity for operations. It automatically maintains balance by tracking node heights and performing rotations (left and right) whenever an insertion or deletion causes the tree to become skewed.

The implementation includes standard operations like search, min/max finding, and removal, along with a recursive utility to visualize the tree structure in the console.

For this code:
```rs
Node::from_iter(-20..=20).tree();
```

The resulting tree looks like:
```
        ┌ 20
      ┌ 19
      │ └ 18
    ┌ 17
    │ └ 16
  ┌ 15
  │ │ ┌ 14
  │ └ 13
  │   └ 12
┌ 11
│ │     ┌ 10
│ │   ┌ 9
│ │   │ └ 8
│ │ ┌ 7
│ │ │ │ ┌ 6
│ │ │ └ 5
│ │ │   └ 4
│ └ 3
│   │   ┌ 2
│   │ ┌ 1
│   │ │ └ 0
│   └ -1
│     │ ┌ -2
│     └ -3
│       └ -4
-5
│     ┌ -6
│   ┌ -7
│   │ └ -8
│ ┌ -9
│ │ │ ┌ -10
│ │ └ -11
│ │   └ -12
└ -13
  │   ┌ -14
  │ ┌ -15
  │ │ └ -16
  └ -17
    │ ┌ -18
    └ -19
      └ -20
```
