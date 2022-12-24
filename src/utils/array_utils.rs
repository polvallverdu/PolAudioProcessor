extern crate itertools;
extern crate num_traits;

use itertools::izip;
use num_traits::float::Float;

pub fn max(a: &[f32]) -> f32 {
  a.iter().fold(f32::NEG_INFINITY, |max, &x| max.max(x))
}

pub fn max_abs(a: &[f32]) -> f32 {
  a.iter().fold(0.0, |max, &x| max.max(x.abs()))
}

pub fn min(a: &[f32]) -> f32 {
  a.iter().fold(f32::INFINITY, |min, &x| min.min(x))
}

pub fn sum_arrays(a: &[f32], b: &[f32]) -> Vec<f32> {
  let c: Vec<f32> = izip!(a, b).map(|(x, y)| x + y).collect();
  c
}

pub fn multiply_array(a: &[f32], b: f32) -> Vec<f32> {
  a.iter().map(|x| x * b).collect()
}

pub fn divide_array(a: &[f32], b: f32) -> Vec<f32> {
  a.iter().map(|x| x / b).collect()
}