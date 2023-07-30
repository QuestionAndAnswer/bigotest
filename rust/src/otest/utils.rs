pub fn linspace(start: i32, end: i32, step: i32) -> Vec<i32> {
    if step <= 1 {
        return vec![start];
    }

    let mut result = Vec::with_capacity(step as usize);
    let delta = (end - start) / (step - 1);

    for i in 0..step {
        result.push(start + delta * i);
    }

    return result;
}
