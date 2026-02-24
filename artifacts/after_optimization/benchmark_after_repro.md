# Бенчмарки После Оптимизации

## Команда запуска
`cargo bench --bench criterion -- --baseline before_opt`

## Покрытые горячие пути
- `sum_even` на `50_000` и `1_000_000` элементов
- `slow_fib` на `n=28` и `n=32`
- `slow_dedup` на `5_000` и `20_000` уникальных значений

## Выходные артефакты
- Лог консоли: `artifacts/after_optimization/benchmark_after.log`
- Полные отчёты Criterion (HTML/SVG/JSON): `artifacts/after_optimization/criterion_after/`
- Главная страница отчёта: `artifacts/after_optimization/criterion_after/report/index.html`

## Ключевые результаты (из лога)
- `sum_even/50000`: ~22.0 us (улучшение ~1.1-1.6%)
- `sum_even/1000000`: ~0.80-0.83 ms
- `slow_fib/28`: ~31.7 ns (существенное ускорение относительно baseline)
- `slow_fib/32`: ~36.3 ns (существенное ускорение относительно baseline)
- `slow_dedup/5000`: ~22.6-23.0 us (существенное ускорение)
- `slow_dedup/20000`: ~94.9-97.9 us (существенное ускорение)

## Примечание
Показатели `slow_fib` и `slow_dedup` улучшились на порядки за счёт изменений алгоритма.
