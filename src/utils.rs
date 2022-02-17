pub fn calc_distance((x1, y1): (f64, f64), (x2, y2): (f64, f64)) -> f64 {
  ((x1 - x2).powf(2.0) + (y1 - y2).powf(2.0)).sqrt()
}


pub fn get_closest_detector_dist((x1, y1): (f64, f64), detectors: &Vec<(f64, f64)>) -> f64 {
  detectors.into_iter().fold(f64::MAX, |acc, det| {
      let dist = calc_distance((x1, y1), *det);
      if acc < dist { acc } else { dist }
  })
}

pub fn calc_detection_level(length: f64, (x, y): (f64, f64), detectors: &Vec<(f64,f64)>) -> f64 {
  let closest_detector_dist = get_closest_detector_dist((x, y), detectors);
  std::f64::consts::E.powf(-(std::f64::consts::PI * closest_detector_dist / length).powf(2.0))
}