pub fn sum_windows(readings: &[usize], window_size: usize) -> i32 {
    let mut window = readings.windows(window_size);
    let mut previous_window = window.next().unwrap();
    let mut increased = 0;
    while let Some(vals) = window.next() {
        let previous_sum: usize = previous_window.iter().sum();
        let current_sum: usize = vals.iter().sum();
        if previous_sum < current_sum {
            increased = increased + 1;
        }
        previous_window = vals;
    }
    increased
}

pub fn count_total_depth(readings: &[usize], window_size: usize) -> i32 {
    let mut window = readings.windows(window_size);
    let mut increases = 0;
    while let Some(vals) = window.next() {
        if vals[0] < vals[1] {
            increases = increases + 1;
        }
    }
    increases
}
