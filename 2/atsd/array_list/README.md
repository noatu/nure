# Array List

Implements a custom dynamic array (similar to a standard Vector) that manually manages memory allocation and resizing. It includes built-in Binary Heap logic to perform in-place Heapsort and priority-queue operations (like deleting the top element). It also features a utility to visualize the array's internal structure as a printed ASCII tree.

Execution result:
```
Data unsorted:   [0, 1, 4, 9, 16, 25, 36, 0, 8, 18, 30, 44, 60, 78, 0, 15, 32, 51, 72, 95, 120, 0, 22, 46, 72, 100, 130, 162, 0, 29, 60, 93]
        ┌ 93
      ┌ 15
    ┌ 0
    │ └ 32
  ┌ 9
  │ │ ┌ 51
  │ └ 8
  │   └ 72
┌ 1
│ │   ┌ 95
│ │ ┌ 18
│ │ │ └ 120
│ └ 16
│   │ ┌ 0
│   └ 30
│     └ 22
0
│     ┌ 46
│   ┌ 44
│   │ └ 72
│ ┌ 25
│ │ │ ┌ 100
│ │ └ 60
│ │   └ 130
└ 4
  │   ┌ 162
  │ ┌ 78
  │ │ └ 0
  └ 36
    │ ┌ 29
    └ 0
      └ 60

Data ascending:  [0, 0, 0, 0, 0, 1, 4, 8, 9, 15, 16, 18, 22, 25, 29, 30, 32, 36, 44, 46, 51, 60, 60, 72, 72, 78, 93, 95, 100, 120, 130, 162]
        ┌ 162
      ┌ 30
    ┌ 8
    │ └ 32
  ┌ 0
  │ │ ┌ 36
  │ └ 9
  │   └ 44
┌ 0
│ │   ┌ 46
│ │ ┌ 15
│ │ │ └ 51
│ └ 0
│   │ ┌ 60
│   └ 16
│     └ 60
0
│     ┌ 72
│   ┌ 18
│   │ └ 72
│ ┌ 1
│ │ │ ┌ 78
│ │ └ 22
│ │   └ 93
└ 0
  │   ┌ 95
  │ ┌ 25
  │ │ └ 100
  └ 4
    │ ┌ 120
    └ 29
      └ 130

Data descending: [162, 130, 120, 100, 95, 93, 78, 72, 72, 60, 60, 51, 46, 44, 36, 32, 30, 29, 25, 22, 18, 16, 15, 9, 8, 4, 1, 0, 0, 0, 0, 0]
        ┌ 0
      ┌ 32
    ┌ 72
    │ └ 30
  ┌ 100
  │ │ ┌ 29
  │ └ 72
  │   └ 25
┌ 130
│ │   ┌ 22
│ │ ┌ 60
│ │ │ └ 18
│ └ 95
│   │ ┌ 16
│   └ 60
│     └ 15
162
│     ┌ 9
│   ┌ 51
│   │ └ 8
│ ┌ 93
│ │ │ ┌ 4
│ │ └ 46
│ │   └ 1
└ 120
  │   ┌ 0
  │ ┌ 44
  │ │ └ 0
  └ 78
    │ ┌ 0
    └ 36
      └ 0


Data [1, 12, 9, 5, 6, 10]
  ┌ 5
┌ 12
│ └ 6
1
│ ┌ 10
└ 9
Deleting top: Some(12)
  ┌ 5
┌ 6
│ └ 1
10
└ 9

Deleting top with one element: None
```
