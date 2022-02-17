use crate::graph::{Node};

// LINK (jp) https://yttm-work.jp/algorithm/algorithm_0015.html
// LINK (jp) https://qiita.com/2dgames_jp/items/f29e915357c1decbc4b7
pub fn a_star(graph: &mut Vec<Vec<Node>>, start: (usize, usize), goal: (usize, usize)) -> f64 {
  let mut open_list: Vec<(usize, usize, f64)> = vec![(start.0, start.1, 0.0)];

  while open_list[open_list.len()-1].0 != goal.0 || open_list[open_list.len()-1].1 != goal.1 {
    // remove node from visitable nodes
    let (x, y, tot) = open_list.pop().unwrap();
    let search_node = &mut graph[y][x];
    search_node.open = false;

    // update total detection level at current node
    let tot_detection_level = tot + search_node.detection_level;
    search_node.tot_detection_level = tot_detection_level;

    for (x, y) in &graph[y][x].edges {
      let neighbor = &graph[*y][*x];
      if neighbor.open {
        open_list.push((*x, *y, tot_detection_level));
      }
    }

    sort_descending_cost(graph, &mut open_list);
  }
  let (x, y, tot) = open_list[open_list.len()-1];
  tot + graph[y][x].detection_level
}

fn sort_descending_cost(graph: &mut Vec<Vec<Node>>, list: &mut Vec<(usize,usize, f64)>) {
  list.sort_by(|(x1, y1, _), (x2, y2, _)| {
    let a = &graph[*y1][*x1];
    let a_tot = a.detection_level + a.dist_to_goal;
    let b = &graph[*y2][*x2];
    let b_tot = b.detection_level + b.dist_to_goal;
    (b_tot).partial_cmp(&a_tot).unwrap()
  });
}