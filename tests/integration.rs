use broken_app::{algo, concurrency, leak_buffer, normalize, sum_even};

#[test]
fn sums_even_numbers() {
    let nums = [1, 2, 3, 4];
    // Ожидаем корректное суммирование: 2 + 4 = 6.
    assert_eq!(sum_even(&nums), 6);
}

#[test]
fn counts_non_zero_bytes() {
    let data = [0_u8, 1, 0, 2, 3];
    assert_eq!(leak_buffer(&data), 3);
}

#[test]
fn dedup_preserves_uniques() {
    let uniq = algo::slow_dedup(&[5, 5, 1, 2, 2, 3]);
    assert_eq!(uniq, vec![1, 2, 3, 5]); // порядок и состав важны
}

#[test]
fn fib_small_numbers() {
    assert_eq!(algo::slow_fib(10), 55);
}

#[test]
fn normalize_simple() {
    assert_eq!(normalize(" Hello World "), "helloworld");
}

#[test]
fn normalize_tabs_and_spaces() {
    assert_eq!(normalize(" \tHello\t  World \n"), "helloworld");
}

#[test]
fn averages_only_positive() {
    let nums = [-5, 5, 15];
    assert!((broken_app::average_positive(&nums) - 10.0).abs() < f64::EPSILON);
}

#[test]
fn averages_no_positive_is_zero() {
    let nums = [-5, 0, -15];
    assert!((broken_app::average_positive(&nums) - 0.0).abs() < f64::EPSILON);
}

#[test]
fn sum_even_empty_slice() {
    assert_eq!(sum_even(&[]), 0);
}

#[test]
fn counter_is_thread_safe() {
    let total = concurrency::race_increment(10_000, 4);
    assert_eq!(total, 40_000);
    assert_eq!(concurrency::read_after_sleep(), 40_000);
    concurrency::reset_counter();
    assert_eq!(concurrency::read_after_sleep(), 0);
}

#[test]
fn use_after_free_regression() {
    // Исторически здесь был UB через use-after-free.
    let res = unsafe { broken_app::use_after_free() };
    assert_eq!(res, 84);
}
