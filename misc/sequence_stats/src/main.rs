fn sequence(n: u64) -> u64 {
    if n == 1 {
        1
    } else if n % 2 == 0 {
        1 + sequence(n / 2)
    } else {
        1 + sequence(n * 3 + 1)
    }
}

fn main() {
    let mut sum: u64 = 0;
    let mut weighted_sum: f64 = 0.0;
    for i in 1..=10000000 {
        let count: u64 = sequence(i);
        sum += count;
        weighted_sum += (count as f64) / ((i+1) as f64).ln();
        if i % 100000 == 0 {
            println!("{:8}: mean={:0.3}, weighted={:0.3}", i,
                (sum as f64) / (i as f64), (weighted_sum / (i as f64)));
        }
    }
}
