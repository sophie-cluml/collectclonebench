#![feature(test)]

extern crate test;

pub const TEST_ARRAY_SIZE: usize = 30;
pub type TestStringArray = [String; TEST_ARRAY_SIZE];
pub type TestInt32Array = [i32; TEST_ARRAY_SIZE];

pub fn string_iter_cloned_collect(original: TestStringArray) {
    let _iter_cloned_collect: Vec<String> = original.iter().cloned().collect();
}

pub fn string_intoiter_collect(original: TestStringArray) {
    let _intoiter_collect: Vec<String> = original.into_iter().collect::<Vec<_>>();
}

pub fn string_iter_collect_clone(original: TestStringArray) {
    let _iter_collect_clone: Vec<&String> = original.iter().collect::<Vec<_>>().clone();
}

pub fn i32_iter_cloned_collect(original: TestInt32Array) {
    let _iter_cloned_collect: Vec<i32> = original.iter().cloned().collect();
}

pub fn i32_iter_copied_collect(original: TestInt32Array) {
    let _iter_copied_collect: Vec<i32> = original.iter().copied().collect();
}

pub fn i32_intoiter_collect(original: TestInt32Array) {
    let _iter_collect: Vec<i32> = original.into_iter().collect::<Vec<_>>();
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_string_iter_cloned_collect(b: &mut Bencher) {
        let original: TestStringArray = (0..TEST_ARRAY_SIZE)
            .map(|num| format!("{}abc", num))
            .collect::<Vec<String>>()
            .try_into()
            .unwrap();

        b.iter(|| string_iter_cloned_collect(original.clone()));
    }

    #[bench]
    fn bench_string_intoiter_collect(b: &mut Bencher) {
        let original: TestStringArray = (0..TEST_ARRAY_SIZE)
            .map(|num| format!("{}abc", num))
            .collect::<Vec<String>>()
            .try_into()
            .unwrap();
        b.iter(|| string_intoiter_collect(original.clone()));
    }

    #[bench]
    fn bench_string_iter_collect_clone(b: &mut Bencher) {
        let original: TestStringArray = (0..TEST_ARRAY_SIZE)
            .map(|num| format!("{}abc", num))
            .collect::<Vec<String>>()
            .try_into()
            .unwrap();
        b.iter(|| string_iter_collect_clone(original.clone()));
    }

    #[bench]
    fn bench_i32_iter_cloned_collect(b: &mut Bencher) {
        let original: TestInt32Array = (1000..1000 + TEST_ARRAY_SIZE as i32)
            .collect::<Vec<i32>>()
            .try_into()
            .unwrap();
        b.iter(|| i32_iter_cloned_collect(original));
    }

    #[bench]
    fn bench_i32_iter_copied_collect(b: &mut Bencher) {
        let original: TestInt32Array = (1000..1000 + TEST_ARRAY_SIZE as i32)
            .collect::<Vec<i32>>()
            .try_into()
            .unwrap();
        b.iter(|| i32_iter_copied_collect(original));
    }

    #[bench]
    fn bench_i32_intoiter_collect(b: &mut Bencher) {
        let original: TestInt32Array = (1000..1000 + TEST_ARRAY_SIZE as i32)
            .collect::<Vec<i32>>()
            .try_into()
            .unwrap();
        b.iter(|| i32_intoiter_collect(original));
    }
}
