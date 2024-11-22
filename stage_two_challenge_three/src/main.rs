/// Convert the index to the respective dice face.
fn index_to_dice(index: u64, dice_index: u32) -> u64 {
    // The number we should divide by to get the face number
    let divisor = 6u64.pow(dice_index);

    // The minimum and maximum a die face can have
    const MIN_FACE: u64 = 1;
    const MAX_FACE: u64 = 6;

    ((index / divisor) % (MAX_FACE - MIN_FACE + 1)) + MIN_FACE
}

fn take_turn(depth: u8, valid: &mut u64, combos: &mut u64) {
    if depth >= 2 {
        return;
    }

    const COMBOS: u64 = 6u64.pow(6);

    for combo_index in 0..COMBOS {
        // Convert index into an array of dies
        let dies: [u64; 6] = core::array::from_fn(|i| index_to_dice(combo_index, i as u32));

        *combos += 1;

        if dies[0] + dies[1] == 7 {
            continue;
        }

        if dies[2] + dies[3] == 7 {
            *valid += 1;
            continue;
        }

        if dies[4] + dies[5] != 7 {
            take_turn(depth + 1, valid, combos);
        }
    }

    // println!("{}", depth);
}

fn main() {
    let mut combos: u64 = 0;
    let mut valid: u64 = 0;

    take_turn(0, &mut valid, &mut combos);

    println!("{} {} {}", valid, combos, valid as f64 / combos as f64)
}
