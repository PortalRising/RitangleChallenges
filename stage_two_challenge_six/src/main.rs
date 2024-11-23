fn main() {
    let mut attempts = 0.0;
    let mut valid = 0.0;

    // x and y must be between 0.5 and 1.5 as they round to 1.0
    let resolution: u64 = 10000;
    let resolution_f = resolution as f64;
    for x in 0..resolution {
        let floating_x = (x as f64 / resolution_f) + 0.5;
        for y in 0..resolution {
            let floating_y = (y as f64 / resolution_f) + 0.5;

            // x * y must round to 1
            if (floating_x * floating_y).round() == 1.0 {
                valid += 1.0;
            }

            attempts += 1.0;
        }
    }

    println!("{} {} {}", valid, attempts, valid / attempts);
}
