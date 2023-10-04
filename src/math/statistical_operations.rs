

/// Takes the average of a vector of f64's
pub fn average(numbers: Vec<f64>) -> f64 {
  let length: f64 = numbers.clone().len() as f64;
  let sum:    f64 = numbers.clone().iter().sum();

  return sum / length;
}


/// Returns the minmum value of a vector of f64's
pub fn min(numbers: Vec<f64>) -> f64 {
  let mut min: f64 = f64::MIN;
  let mut first_iter: bool = true;

  for number in numbers {
    if first_iter {
      min = number;
      first_iter = false;
    }

    if number < min { min = number}
  }

  return min;
}


/// Returns the maximum value of a vector of f64's
pub fn max(numbers: Vec<f64>) -> f64 {
  let mut max:        f64  = f64::MAX;
  let mut first_iter: bool = true;

  for number in numbers {
    if first_iter == true {
      max = number;
      first_iter = false;
    }

    if number > max { max = number }
  }

  return max;
}