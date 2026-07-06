use std::hint::black_box;
use std::time::Instant;

use lunar_rs::{Lunar, Solar};

fn run_bench(name: &str, iterations: usize, mut f: impl FnMut()) {
    for _ in 0..1_000 {
        f();
    }

    let started_at = Instant::now();
    for _ in 0..iterations {
        f();
    }
    let elapsed = started_at.elapsed();
    let nanos_per_iter = elapsed.as_secs_f64() * 1_000_000_000.0 / iterations as f64;

    println!("{name:<24} {iterations:>10} iters  total={elapsed:?}  avg={nanos_per_iter:.2}ns");
}

fn main() {
    let hot_solar = Solar::from_ymd(2020, 5, 1).unwrap();
    let hot_lunar = hot_solar.lunar();

    run_bench("solar_to_lunar_hot", 200_000, || {
        black_box(hot_solar.lunar());
    });

    let mut offset = 0_i32;
    run_bench("solar_to_lunar_mixed", 50_000, || {
        let year = 1900 + offset.rem_euclid(200);
        let solar = Solar::from_ymd(year, 5, 1).unwrap();
        black_box(solar.lunar());
        offset += 1;
    });

    run_bench("lunar_to_solar_hot", 200_000, || {
        black_box(hot_lunar.solar());
    });

    run_bench("lunar_full_string", 100_000, || {
        black_box(hot_lunar.to_full_string());
    });

    let jieqi_lunar = Lunar::from_ymd(2012, 9, 1).unwrap();
    run_bench("jie_qi_table_lookup", 150_000, || {
        black_box(jieqi_lunar.jie_qi_table().get("白露"));
    });
}
