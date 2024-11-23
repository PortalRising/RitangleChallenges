use itertools::Itertools;

fn attempt(a: u32, b: u32, c: u32, d: u32, e: u32, f: u32) -> Result<(u32, u32, u32), ()> {
    let last_age_str = c.to_string() + &a.to_string() + &b.to_string();
    let now_age_str = a.to_string() + &b.to_string() + &c.to_string();
    let next_age_str = b.to_string() + &c.to_string() + &a.to_string();

    if e == 1 || d == 1 || f == 1 {
        return Err(());
    }

    let last_age_num = u32::from_str_radix(&last_age_str, e).map_err(|_| ())?;
    let now_age_num = u32::from_str_radix(&now_age_str, d).map_err(|_| ())?;
    let next_age_num = u32::from_str_radix(&next_age_str, f).map_err(|_| ())?;

    Ok((last_age_num, now_age_num, next_age_num))
}

fn main() {
    let mut combos = 0;
    for order in (1u32..7).permutations(6) {
        let [a, b, c, d, e, f] = if let [a, b, c, d, e, f] = order[..] {
            [a, b, c, d, e, f]
        } else {
            unreachable!("The vec should have the len of 6");
        };

        let (last_age_num, now_age_num, next_age_num) = match attempt(a, b, c, d, e, f) {
            Ok(nums) => nums,
            Err(_) => continue,
        };

        if last_age_num + 1 == now_age_num && now_age_num == next_age_num - 1 {
            println!("a: {}, b: {}, c: {}, d: {}, e: {}, f: {}", a, b, c, d, e, f);
            println!("Current age: {}", now_age_num);
        }

        combos += 1;
    }

    println!("{}", combos)
}
