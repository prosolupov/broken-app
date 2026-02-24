/// Дедупликация с сортировкой.
/// Микро-оптимизация: одна аллокация под копию входа, одна сортировка и in-place dedup.
pub fn slow_dedup(values: &[u64]) -> Vec<u64> {
    let mut out = values.to_vec();
    out.sort_unstable();
    out.dedup();
    out
}

/// Алгоритмическая оптимизация:
/// вместо экспоненциальной рекурсии используем линейный итеративный расчёт.
pub fn slow_fib(n: u64) -> u64 {
    if n <= 1 {
        return n;
    }
    let mut prev = 0_u64;
    let mut curr = 1_u64;
    for _ in 2..=n {
        let next = prev + curr;
        prev = curr;
        curr = next;
    }
    curr
}
