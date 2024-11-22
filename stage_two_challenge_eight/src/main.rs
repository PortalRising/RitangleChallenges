fn main() {
    let all_integers: Vec<usize> = (2..2024).into_iter().collect();

    let mut product = 1;

    for n in 2..2024usize {
        for offset in 0..(2023 - n) {
            let start = offset;
            let end = offset + n;

            println!("{} {} {}", n, start, end);

            let sum: usize = all_integers[start..end].iter().sum();

            if sum == 2024 {
                product *= n;
            }
        }
    }

    println!("{}", product);
}
