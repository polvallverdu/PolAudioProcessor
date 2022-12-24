pub fn limit(data: &mut Vec<f32>) {
    for i in 0..data.len() {
        if data[i] > 1.0 {
            data[i] = 1.0;
        } else if data[i] < -1.0 {
            data[i] = -1.0;
        }
    }
}