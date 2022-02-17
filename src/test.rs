#[cfg(test)]
mod tests {
    use crate::{input::*, graph::*, utils::*, a_star::*};

    #[test]
    fn test_input() {
      let input = get_input("src/input0.map");
      let input_fmt = format_input(input);
      assert!(input_fmt.length == 25.0);
      assert!(input_fmt.num_detectors == 5);
      assert!(input_fmt.detectors.len() == input_fmt.num_detectors as usize);
    }
    
    #[test]
    fn test_a_star() {
      let input = get_input("src/input0.map");
      let input_fmt = format_input(input);
      let mut graph = setup_graph(&input_fmt);

      let result = a_star(&mut graph, (26/2, 0), (26/2, 25));
      println!("A* result: {:.3}", result);
      assert!(false);
    }

    #[test]
    fn test_graph_setup() {
      let input = get_input("src/input0.map");
      let input_fmt = format_input(input);
      let graph = setup_graph(&input_fmt);
      assert!(graph.len() == 26);
      for row in graph {
        assert!(row.len() == 26);
      }
    }

    #[test]
    fn test_distance() {
      let p1 = (0.0, 0.0);
      let p2 = (10.0, 10.0);
      assert!(calc_distance(p1, p2) == 200.0_f64.sqrt());
      let p2 = (100.0, 100.0);
      assert!(calc_distance(p1, p2) == 20000.0_f64.sqrt());
    }

    #[test]
    fn test_closest_detector_dist() {
      let detectors: Vec<(f64,f64)> = vec![
        (1.0536920857525445, 2.8936703202385115),
        (1.3175678970133191, 10.019351994529405),
        (16.739302303324447, 15.87541372165791),
        (19.39788132776695, 14.174570106439353),
        (2.423929917008996, 20.187139309438546),
      ];

      let p = (0.0, 0.0);
      let closest_detector_dist = get_closest_detector_dist(p, &detectors);
      assert!(closest_detector_dist == calc_distance(p, detectors[0]));

      let p = (1.0, 10.0);
      let closest_detector_dist = get_closest_detector_dist(p, &detectors);
      assert!(closest_detector_dist == calc_distance(p, detectors[1]));
      
      let p = (16.0, 15.0);
      let closest_detector_dist = get_closest_detector_dist(p, &detectors);
      assert!(closest_detector_dist == calc_distance(p, detectors[2]));
    }

    #[test]
    fn test_detection_level() {
      let detectors: Vec<(f64,f64)> = vec![
        (1.0536920857525445, 2.8936703202385115),
        (1.3175678970133191, 10.019351994529405),
        (16.739302303324447, 15.87541372165791),
        (19.39788132776695, 14.174570106439353),
        (2.423929917008996, 20.187139309438546),
      ];

      let node = Node {
          x: 0.0,
          y: 0.0,
          open: true,
          edges: Vec::new(),
          detection_level: 0.0,
          tot_detection_level: 0.0,
          dist_to_goal: 20.0
      };
      let detection_level = calc_detection_level(20.0, (node.x, node.y), &detectors);
      assert!(detection_level == 0.7913631408404486);
    }

    #[test]
    fn test_get_edges() {
      let edges = get_edges(5, (0, 0));
      assert!(edges.len() == 2);
      assert!(edges == vec![(0, 1), (1, 0)]);

      let edges = get_edges(5, (2, 2));
      assert!(edges.len() == 4);
      assert!(edges == vec![(2, 3), (2, 1), (3, 2), (1, 2)]);
      
      let edges = get_edges(5, (5, 5));
      assert!(edges.len() == 2);
      assert!(edges == vec![(5, 4), (4, 5)]);
    }

    // #[test]
    // fn test_sort_ascending() {
    //   let mut list: Vec<Node> = Vec::new();
    //   let mut refs: Vec<&mut Node> = Vec::new();
    //   for mut node in list.into_iter() {
    //     refs.push(&mut node);
    //   }
    // }
}