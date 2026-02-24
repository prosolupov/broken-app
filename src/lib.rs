pub mod algo;
pub mod concurrency;

/// Сумма чётных значений.
pub fn sum_even(values: &[i64]) -> i64 {
    values.iter().copied().filter(|v| v % 2 == 0).sum()
}

/// Подсчёт ненулевых байтов.
pub fn leak_buffer(input: &[u8]) -> usize {
    input.iter().filter(|&&b| b != 0).count()
}

/// Небрежная нормализация строки: удаляем пробелы и приводим к нижнему регистру,
/// но игнорируем повторяющиеся пробелы/табуляции внутри текста.
pub fn normalize(input: &str) -> String {
    input
        .split_whitespace()
        .collect::<String>()
        .to_lowercase()
}

/// Логическая ошибка: усредняет по всем элементам, хотя требуется учитывать
/// только положительные. Деление на длину среза даёт неверный результат.
pub fn average_positive(values: &[i64]) -> f64 {
    let (sum, count) = values
        .iter()
        .copied()
        .filter(|v| *v > 0)
        .fold((0_i64, 0_usize), |(s, c), v| (s + v, c + 1));
    if count == 0 {
        return 0.0;
    }
    sum as f64 / count as f64
}

/// Совместимость API: функция исторически была `unsafe`,
/// но реализация теперь не использует опасные операции.
pub unsafe fn use_after_free() -> i32 {
    let b = Box::new(42_i32);
    let val = *b;
    val + val
}
