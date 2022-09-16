use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ibig::UBig;
use num_traits::{One, Zero};
use roll::combinatorics::Combinations;

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut combinations = c.benchmark_group("Combinations");
    for r in [64, 128, 192] {
        let n = 255_usize;

        combinations.bench_function(format!("{} `choose` {:3}", n, r), |b| {
            b.iter(|| choose(black_box(n), black_box(r)));
        });

        combinations.bench_function(format!("{} `choose` {:3} (memoized)", n, r), |b| {
            b.iter(|| chooser_choose(black_box(n), black_box(r)));
        });
    }
    combinations.finish();

    let mut counting = c.benchmark_group("Combination Counting");
    counting.sample_size(30);
    for r in [32, 64, 128] {
        counting.bench_function(format!("{r:3}d{r}", r = r), |b| {
            b.iter(|| count_combinations(black_box(r), black_box(r)))
        });
    }
    counting.finish();
}

fn chooser_choose(n: usize, r: usize) -> UBig {
    let mut chooser = Combinations::default();

    chooser.choose(n, r)
}

// An optimized non-memoized combination counter.
fn choose(n: usize, r: usize) -> UBig {
    if r > n {
        return Zero::zero();
    }

    let r = if r > (n / 2) { n - r } else { r };

    let mut num = <UBig as One>::one();
    for i in (n - r + 1)..=n {
        num *= i;
    }

    let mut den = <UBig as One>::one();
    for i in 2..=r {
        den *= i;
    }

    num / den
}

fn count_combinations(n_dice: usize, sides: usize) -> UBig {
    let mut total_rolls = <UBig as num_traits::Zero>::zero();
    let mut chooser = Combinations::default();

    for i in n_dice..=(n_dice * sides) {
        total_rolls += chooser.dice_roll_sum_count(i, n_dice, sides)
    }

    total_rolls
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
