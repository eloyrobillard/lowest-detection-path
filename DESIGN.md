# Design document

## Considerations

- The burglar moves along a grid with steps of 1 meter
  > the grid can be represented as a graph with the detection level as the weight for each node
- The shortest path goes straight up from the bottom, for L meters
- The burglar starts at $(\frac{L+1}{2}, 0)$
- The probability of detection is $e^{(-(\pi*\frac{D}{L}^2))}$
- For 2 detectors, the path with the least detection level passes through all points at equidistance

## General approach

For 2 detectors, the path with the least detection level passes through all points at equidistance. One possible solution could then be this path for all pairs of detectors, follow them all, and compare the total detection levels. The problem is, of course, that we have finite precision.

To skip having to draw continuous line, all the possible positions of the burglar can be represented as a grid, and a graph can be built out of these positions. The weight would be represented with the detection level at that position.

Once with a graph, we can:

- go through the graph, choosing the next position with the least detection level: Dijkstra's
- on the other hand, we know where the goal is, so we can calculate the distance of each position to the goal, and pick the Node closest to it: A*
  > this is to avoid having to check all nodes with a low detection level

## Implementation

For this simple implementation of A*, `graph.rs` contains

``` rust
pub struct Node {
  pub x: f64,
  pub y: f64,
  pub open: bool,
  pub edges: Vec<(usize, usize)>,
  pub detection_level: f64,
  pub dist_to_goal: f64,
  pub tot_detection_level: f64
}
```

All Nodes are initialized with a calculation of their `detection_level` by finding the closest detector and running the formula against it.
 > instead of checking the distance of a node against all detectors, a BSP tree would help check only against detectors from the same spatial partition. The gain would increase with the number of detectors.

- `open` is set to false upon visit, to prevent multiple visits of the same Node
- `edges` stores the coordinates for Nodes that can be visited from this one
- `tot_detection_level` stores the sum of all the `detection_level` of the Nodes that lead to this Node, plus the `detection_level` of this Node
  > Ideally, `tot_detection_level` could be calculated by storing a `parent` reference to the Node that led to this Node. Then it's just of matter of adding `detection_level` to `parent.tot_detection_level` \
  > This could be done by passing a reference to the current node inside `open_list`, but would violate ownership rules in the current implementation.

### A*

This implementation uses an `open_list` of tuples containing the coordinates for the next node, with a value for the total detection level up to the next node.

1. The corresponding Node is loaded
2. From the coordinates of its neighbors, all those that haven't been visited are pushed into `open_list` along the previous total detection level + that of the current Node.
3. `open_list` is sorted in descending order, such that the next `pop()` will return the Node with the lowest (detection level + distance to the goal).
   > switching open_list to a binary tree may help prevent mitigate sorting time, but since most of `open_list` is already sorted the time gain may not be very interesting

With this setup, the idea is to return the `tot_detection_level` of the goal Node once the traversal is complete. It should contain the total for the path with the lowest detection level possible.

## Improvements (notes to self)

- Calculation of the distance: it may be possible to use square of distance between nodes and exit since all edges form squares of equal side
