use crate::utils::array_utils as au;

/*pub fn mixer(data: Vec<Vec<f32>>) -> Vec<f32> {
    let mut max_value = 0.0;

    // Get the max value, doing abs() on each value
    for i in 0..data.len() {
        let mut max = au::max_abs(data[i].as_slice());
        if max > max_value {
            max_value = max;
        }
    }
    println!("max_value: {}", max_value);

    // mix all arrays on nddata summing them up and dividing by len of nddata
    let mut sum = data[0].clone();
    for i in 1..data.len() {
        sum = au::sum_arrays(&sum, &data[i]);
    }
    sum = au::divide_array(&sum, data.len() as f32);

    let mut mixed_max_value = au::max_abs(sum.as_slice());
    println!("mixed_max_value: {}", mixed_max_value);
    let dif = max_value / mixed_max_value;
    println!("dif: {}", dif);
    return au::multiply_array(&sum, dif);
}*/

pub fn mixer(data: Vec<Vec<f32>>) -> Vec<f32> {
    let mut max_value = 0.0;
    let mut mixed_max_value = 0.0;
    
    let mut sum = data[0].clone();
    for i in 1..data.len() {
        let is_final = i == data.len() - 1;

        if is_final {
            for j in 0..data[i].len() {
                let ab = data[i][j];
                if ab > max_value || ab < -max_value {
                    max_value = ab.abs();
                }
    
                sum[j] += data[i][j];
    
                sum[j] /= data.len() as f32;

                let ab = sum[j].abs();
                if ab > mixed_max_value || ab < -mixed_max_value {
                    mixed_max_value = ab.abs();
                }
            }
        } else {
            for j in 0..data[i].len() {
                let ab = data[i][j];
                if ab > max_value || ab < -max_value {
                    max_value = ab.abs();
                }
    
                sum[j] += data[i][j];
            }
        }
    }

    println!("max_value: {}", max_value);
    println!("mixed_max_value: {}", mixed_max_value);
    let dif = max_value / mixed_max_value;
    println!("dif: {}", dif);
    return au::multiply_array(&sum, dif);
}