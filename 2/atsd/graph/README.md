# Graph

Implements a weighted, undirected graph with a minimal CLI to build it. Algorithms like Dijkstra's Shortest Path and Kruskal's Minimum Spanning Tree (MST) are available.

Execution result:
```
Exit - 'q' or 'x'
Print the graph - 'p'
Calculate dijkstra to each vertex - 'd vertex'
Remove an edge - 'vertex1 vertex2'
Add an edge - 'vertex1 vertex2 distance'
1 2 6
Added 1 - 2, 6
2 4 5
Added 2 - 4, 5
4 5 22
Added 4 - 5, 22
5 7 10
Added 5 - 7, 10
7 6 12
Added 7 - 6, 12
6 1 15
Added 6 - 1, 15
3 2 11
Added 3 - 2, 11
3 4 17
Added 3 - 4, 17
3 6 25
Added 3 - 6, 25
3 7 12
Added 3 - 7, 12
3 7
Removed 3 - 7
3 7 9
Added 3 - 7, 9
p
1 - 2, 6
1 - 6, 15
2 - 3, 11
2 - 4, 5
3 - 4, 17
3 - 6, 25
3 - 7, 9
4 - 5, 22
5 - 7, 10
6 - 7, 12
d 1
{1: None, 2: Some((1, 6)), 3: Some((2, 17)), 4: Some((2, 11)), 5: Some((4, 33)), 6: Some((1, 15)), 7: Some((3, 26))}
q
1 - 2, 6
1 - 6, 15
2 - 3, 11
2 - 4, 5
3 - 4, 17
3 - 6, 25
3 - 7, 9
4 - 5, 22
5 - 7, 10
6 - 7, 12

1 - 2, 6
2 - 3, 11
2 - 4, 5
3 - 7, 9
5 - 7, 10
6 - 7, 12
```
