use core::f64;

use itertools::Itertools;

fn quadratic_formula(a: f64, b: f64, c: f64) -> Option<f64> {
    // Work out the discriminant
    let discriminant = b.powi(2) - (4.0 * a * c);

    // If the discriminant is negative them we have no real roots
    if discriminant < 0.0 {
        return None;
    }

    // Find the positive root
    let divisor = 2.0 * a;
    let negative_b = -b;
    let sqrt_discriminant = discriminant.sqrt();
    
    // The only time the root can be negative if b is already negative, 
    // but its a length so it cannot
    let positive_root = (negative_b + sqrt_discriminant) / divisor; 

    // println!("{} {} {} {}", negative_b, sqrt_discriminant, divisor, positive_root);

    // If root is negative then the other root is probably negative too so return nothing
    if positive_root < 0.0 {
        return None;
    }

    return Some(positive_root);
}

fn main() {
    let values: [f64; 3] = [50.0, 60.0, 70.0];

    let mut shortest_combo = [0.0; 4];
    let mut shortest_perimeter = f64::INFINITY;    
    for combination in values.iter().permutations(3) {
        let mut shortest_side = f64::INFINITY;

        let side_x = *combination[0];
        let side_y = *combination[1];
        let angle_rads = combination[2].to_radians();

        println!("{} {} {}", side_x, side_y, angle_rads);
        {
            // Assume x and y are the sides b and c
            // a^2 = b^2 + c^2 - 2bcCosA
            let a_squared = side_x.powi(2) + side_y.powi(2) - (2.0 * side_x * side_y * angle_rads.cos());
            let a = a_squared.sqrt();

            if a < shortest_side {
                println!("A {} {} {}", a, side_x, side_y);
                shortest_side = a;
            }
        }

        {
            // Assume x and y are the sides a and b
            // a^2 = b^2 + c^2 - 2bcCosA
            // c^2 - 2bcCosA  + b^2 - a^2 = 0
            // c^2 - (2bCosA)c + (b^2 - a^2) = 0
            
            let quad_a = 1.0;
            let quad_b = 2.0 * side_y * angle_rads.cos();
            let quad_c = side_y.powi(2) - side_x.powi(2);

            let side = quadratic_formula(quad_a, quad_b, quad_c);

            if side.is_some() && side.unwrap() < shortest_side {
                println!("B {} {} {}", side_x, side_y, side.unwrap());
                shortest_side = side.unwrap();
            }
        }
    
        {
            // Assume x and y are the sides a and c
            // a^2 = b^2 + c^2 - 2bcCosA
            // b^2 - 2bcCosA + c^2 - a^2 = 0
            // b^2 - (2cCosA)b + (c^2 - a^2) = 0
            let quad_a = 1.0;
            let quad_b = 2.0 * side_y * angle_rads.cos();
            let quad_c = side_y.powi(2) - side_x.powi(2);

            let side = quadratic_formula(quad_a, quad_b, quad_c);

            if side.is_some() && side.unwrap() < shortest_side {
                println!("C {} {} {}", side_x, side.unwrap(), side_y);
                shortest_side = side.unwrap();
            }
        }

        // Evaluate perimeter
        let perimeter = shortest_side + side_x + side_y;

        if perimeter < shortest_perimeter {
            shortest_combo = [side_x, side_y, shortest_side, angle_rads.to_degrees()];
            shortest_perimeter = perimeter;
        }
    }

    println!();
    println!("{:?}", shortest_combo);
    println!("shortest perimeter = {}", shortest_perimeter);
}
