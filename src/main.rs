use rayon::prelude::*;

fn solutions(n: usize) -> usize {
    fn solve(
        row: usize,
        n: usize,
        cols: isize,
        main_diags: isize,
        anti_diags: isize,
    ) -> usize {
        if row == n { return 1; }
        let mut count = 0;
        let mut avail = ((1 << n) - 1) & !(cols | main_diags | anti_diags);

        while avail != 0 {
            let col_bit = avail & -avail; 
            avail ^= col_bit;

            count += solve(
                row + 1,
                n,
                cols | col_bit,
                (main_diags | col_bit) << 1,
                (anti_diags | col_bit) >> 1,
            )
        }

        count
    }

    let mut count = (0..(n / 2))
        .into_par_iter()
        .map(|col| solve(1, n, 1 << col, 1 << (col + 1), 1 << (col - 1)))
        .sum();

    count *= 2;

    if n % 2 == 1 {
        let mid = 1 << (n / 2);
        count += solve(1, n, 1 << mid, 1 << (mid + 1), 1 << (mid - 1));
    }

    count
}

fn main() {
    let start = std::time::Instant::now();

    let n = 16;
    let result = solutions(n);

    let elapsed = start.elapsed();
    println!("found {} solutions for n = {}", result, n);
    println!("elapsed time: {:?}", elapsed);
}
