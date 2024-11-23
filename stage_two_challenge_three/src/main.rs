/// Convert the index to the respective dice face.
fn index_to_dice(index: u64, dice_index: u32) -> u64 {
    // The number we should divide by to get the face number
    let divisor = 6u64.pow(dice_index);

    // The minimum and maximum a die face can have
    const MIN_FACE: u64 = 1;
    const MAX_FACE: u64 = 6;

    ((index / divisor) % (MAX_FACE - MIN_FACE + 1)) + MIN_FACE
}

fn take_turns(one_wins: &mut u64, two_wins: &mut u64, three_wins: &mut u64) {
    const COMBOS: u64 = 6u64.pow(6);

    for combo_index in 0..COMBOS {
        // Convert index into an array of dies
        let dies: [u64; 6] = core::array::from_fn(|i| index_to_dice(combo_index, i as u32));

        if dies[0] + dies[1] == 7 {
            *one_wins += 1;
            continue;
        }

        if dies[2] + dies[3] == 7 {
            *two_wins += 1;
            continue;
        }

        if dies[4] + dies[5] == 7 {
            *three_wins += 1;
            continue;
        }
    }

    // println!("{}", depth);
}

fn main() {
    let mut one_wins: u64 = 0;
    let mut two_wins: u64 = 0;
    let mut three_wins: u64 = 0;

    take_turns(&mut one_wins, &mut two_wins, &mut three_wins);

    println!(
        "{} {} {} {}",
        one_wins,
        two_wins,
        three_wins,
        (two_wins) as f64 / (one_wins + two_wins + three_wins) as f64
    )
}
