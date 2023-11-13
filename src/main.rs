use std::time::Instant;

use collectclonebench::*;

fn main() {
    compare_string();
    compare_i32();
}

fn compare_string() {
    let start = Instant::now();
    for _ in 0..1_000_000 {
        let original: TestStringArray = (0..TEST_ARRAY_SIZE)
            .map(|num| format!("{}abc", num))
            .collect::<Vec<String>>()
            .try_into()
            .unwrap();
        string_iter_cloned_collect(original);
    }
    let duration_iter_cloned_collect = start.elapsed();

    let start = Instant::now();
    for _ in 0..1_000_000 {
        let original: TestStringArray = (0..TEST_ARRAY_SIZE)
            .map(|num| format!("{}abc", num))
            .collect::<Vec<String>>()
            .try_into()
            .unwrap();
        string_intoiter_collect(original);
    }
    let duration_intoiter_collect = start.elapsed();

    let start = Instant::now();
    for _ in 0..1_000_000 {
        let original: TestStringArray = (0..TEST_ARRAY_SIZE)
            .map(|num| format!("{}abc", num))
            .collect::<Vec<String>>()
            .try_into()
            .unwrap();
        string_iter_collect_clone(original);
    }
    let duration_iter_collect_clone = start.elapsed();

    println!(
        "[String; {}] Time for string_iter_cloned_collect: {:?}",
        TEST_ARRAY_SIZE,
        duration_iter_cloned_collect / 1_000_000
    );
    println!(
        "[String; {}] Time for string_intoiter_collect: {:?}",
        TEST_ARRAY_SIZE,
        duration_intoiter_collect / 1_000_000
    );
    println!(
        "[String; {}] Time for duration_iter_collect_clone: {:?}",
        TEST_ARRAY_SIZE,
        duration_iter_collect_clone / 1_000_000
    );
}

fn compare_i32() {
    let start = Instant::now();
    for _ in 0..1_000_000 {
        let original: TestInt32Array = (1000..1000 + TEST_ARRAY_SIZE as i32)
            .collect::<Vec<i32>>()
            .try_into()
            .unwrap();
        i32_iter_cloned_collect(original);
    }
    let duration_i32_iter_cloned_collect = start.elapsed();

    let start = Instant::now();
    for _ in 0..1_000_000 {
        let original: TestInt32Array = (1000..1000 + TEST_ARRAY_SIZE as i32)
            .collect::<Vec<i32>>()
            .try_into()
            .unwrap();
        i32_iter_copied_collect(original);
    }
    let duration_i32_iter_copied_collect = start.elapsed();

    let start = Instant::now();
    for _ in 0..1_000_000 {
        let original: TestInt32Array = (1000..1000 + TEST_ARRAY_SIZE as i32)
            .collect::<Vec<i32>>()
            .try_into()
            .unwrap();
        i32_intoiter_collect(original);
    }
    let duration_i32_intoiter_collect = start.elapsed();

    println!(
        "[i32; {}] Time for i32_iter_cloned_collect: {:?}",
        TEST_ARRAY_SIZE,
        duration_i32_iter_cloned_collect / 1_000_000
    );
    println!(
        "[i32; {}] Time for i32_iter_copied_collect: {:?}",
        TEST_ARRAY_SIZE,
        duration_i32_iter_copied_collect / 1_000_000
    );
    println!(
        "[i32; {}] Time for i32_intoiter_collect: {:?}",
        TEST_ARRAY_SIZE,
        duration_i32_intoiter_collect / 1_000_000
    );
}
