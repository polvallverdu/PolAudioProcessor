extern crate ndarray;
use ndarray::{Array};

pub fn limit(data: Vec<f32>) -> Vec<f32> {
    // pass vector to ndarray
    let mut data = Array::from(data);
    // if value is greater than 1, set to 1, and if less than -1, set to -1
    data.mapv_inplace(|x| if x > 1.0 { 1.0 } else if x < -1.0 { -1.0 } else { x });
    // convert back to vector
    data.to_vec()
}