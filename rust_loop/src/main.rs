use std::time::Instant;

fn main() {
    let start = Instant::now();

    let mut result: i64 = 0;
    for i in 0..2000 {
        for j in 0..2000 {
            for k in 0..2000 {
                result += i * j * k;
            }
        }
    }

    let duration = start.elapsed();

    println!("Resultado: {}", result);
    println!("Tempo total: {:.9}", duration.as_secs_f64());
}
