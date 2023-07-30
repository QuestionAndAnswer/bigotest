#[global_allocator]
static GLOBAL: jemallocator::Jemalloc = jemallocator::Jemalloc;

pub mod otest;

use rand::Rng;

const POINTS: usize = 200;
const REPATES: usize = 3;
const PREHEAT: usize = 1;

fn bake_array(n: usize) -> Vec<i32> {
    return vec![0; n];
}

fn bake_n(n: usize) -> usize {
    return n;
}

fn bake_random_array(n: usize) -> Vec<i32> {
    let mut data = vec![0; n.try_into().unwrap()];
    let mut rng = rand::thread_rng();
    rng.try_fill(data.as_mut_slice()).unwrap();
    return data;
}

fn main() {
    otest::runner::run_o_test(
        "seq_search",
        || {
            fn search(data: &mut Vec<i32>, target: i32) {
                for d in data {
                    if *d == target {
                        *d = 0;
                        break;
                    }
                }
            }

            fn alg(data: &mut Vec<i32>) {
                for _ in 0..10 {
                    search(data, 100)
                }
            }

            return otest::runner::run_n(alg, bake_array, 10000000, POINTS);
        },
        PREHEAT,
        REPATES,
    );

    otest::runner::run_o_test(
        "fast_power",
        || {
            fn power(n: usize, mut m: usize) -> usize {
                let mut power = n;
                let mut sum = 1;

                while m > 0 {
                    if (m & 1) == 1 {
                        sum *= power;
                    }
                    power = power * power;
                    m = m >> 1;
                }

                return sum;
            }

            fn alg(n: &mut usize) {
                for _ in 0..10000000 {
                    std::hint::black_box(power(std::hint::black_box(1), std::hint::black_box(*n)));
                }
            }

            return otest::runner::run_n(alg, bake_n, 100000000, POINTS);
        },
        PREHEAT,
        REPATES,
    );

    otest::runner::run_o_test(
        "qsort",
        || {
            fn partition(arr: &mut Vec<i32>, low: usize, high: usize) -> usize {
                let pivot = arr[high];
                let mut i = low;
                for j in low..high {
                    if arr[j] < pivot {
                        arr.swap(i, j);
                        i += 1;
                    }
                }
                arr.swap(i, high);

                return i;
            }

            fn sort(arr: &mut Vec<i32>, low: isize, high: isize) {
                if low < high {
                    let p = partition(arr, low as usize, high as usize) as isize;
                    sort(arr, low, p - 1);
                    sort(arr, p + 1, high);
                }
            }

            fn alg(data: &mut Vec<i32>) {
                let mut data_copy = vec![0; data.len()];

                for _ in 0..4 {
                    data_copy.copy_from_slice(data);

                    sort(&mut data_copy, 0, (data.len() - 1).try_into().unwrap());
                }
            }

            return otest::runner::run_n(alg, bake_random_array, 1000000, POINTS);
        },
        PREHEAT,
        REPATES,
    );

    otest::runner::run_o_test(
        "bubble_sort",
        || {
            fn sort(data: &mut Vec<i32>) {
                for _ in 0..data.len() {
                    for j in 0..data.len() - 1 {
                        if data[j] > data[j + 1] {
                            data.swap(j, j + 1)
                        }
                    }
                }
            }

            fn alg(data: &mut Vec<i32>) {
                let mut data_copy = vec![0; data.len()];

                for _ in 0..4 {
                    data_copy.copy_from_slice(data);

                    sort(data);
                }
            }

            return otest::runner::run_n(alg, bake_random_array, 10000, POINTS);
        },
        PREHEAT,
        REPATES,
    );
}
