use crate::input::{Input};
use crate::utils::{calc_detection_level, calc_distance};

pub struct Node {
  pub x: f64,
  pub y: f64,
  pub open: bool,
  pub edges: Vec<(usize, usize)>,
  pub detection_level: f64,
  pub dist_to_goal: f64,
  pub tot_detection_level: f64
}

pub fn setup_graph(input: &Input) -> Vec<Vec<Node>> {
  let mut result: Vec<Vec<Node>> = Vec::new();
  let side = input.length as usize;
  for y in 0..=side {
      result.push(Vec::new());
      for x in 0..=side {
          result[y].push(
              Node {
                  x: x as f64,
                  y: y as f64,
                  open: true,
                  edges: get_edges(side, (x,y)),
                  detection_level: calc_detection_level(input.length, (input.length, 0.0), &input.detectors),
                  tot_detection_level: 0.0,
                  dist_to_goal: calc_distance((x as f64, y as f64), (input.length / 2.0, input.length))
              }
          );
      }
  }

  result
}

pub fn get_edges(side: usize, (x, y): (usize, usize)) -> Vec<(usize, usize)>{
  match x {
      0 => match y {
          0 => vec![(0, 1), (1, 0)],
          y if y == side => vec![(0, side-1), (1, side)],
          _ => vec![(0, y+1), (0, y-1), (0+1, y)],
      },
      x if x == side => match y {
          0 => vec![(side, 1), (side-1, 0)],
          y if y == side => vec![(side, side-1), (side-1, side)],
          _ => vec![(side, y+1), (side, y-1), (side+1, y)],
      },
      _ => match y {
          0 => vec![(x-1, 0), (x+1, 0), (x, 1)],
          y if y == side => vec![(x-1, 0), (x+1, 0), (x, side-1)],
          _ => vec![(x, y+1), (x, y-1), (x+1, y), (x-1, y)],
      }
  }
}